//! This is mostly a straight port of the D version.  Originally, the "non-locking" Go solution was
//! tried, because it was supposed to be faster than the version with Mutexes, but my experience
//! was that this was not the case.  Perhaps it is true with green threads.  D's version was much
//! faster and this version seems to achieve parity with the benchmarks on the Rosetta Code site
//! (at least on my machine).  I am pretty sure it could be made faster, though--for example, the
//! Mutex type we're using here was the fourth type I tried but the first to produce acceptable
//! performance (previously I tried, in order, `std::sync::RwLock`, `std::sync::Mutex`, and
//! `std::sync::Semaphore`) and this type still appears to have quite a bit of overhead.

extern crate rand;

use std::thread::{self, spawn};
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use rand::{Rng, weak_rng};
use rand::distributions::{IndependentSample, Range};

/// The reason I used a module here is simply to keep it clearer who can access what.  Rust
/// protects against data races just fine, but it's not as good at protecting against deadlocks or
/// other types of race conditions.
mod buckets {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    /// We hardcode the number of buckets mostly for convenience.  Now that Rust has dynamically
    /// sized types, this is possibly no longer a problem.
    ///
    /// To expand: in Rust, there are two special kinds, `Sync` and `Send`, used for concurrency.
    ///
    /// If `T` is `Sync`, and you take an immutable reference of type `&T`, then it's safe to share
    /// between threads.  Most types are `Sync` unless they contain non-threadsafe interior
    /// mutability: for example, `Cell` and `RefCell` are not Sync, because they can be modified
    /// through a `&` reference in a non-threadsafe way.  On the other hand, atomic types are
    /// `Sync` even though they can be modified this way.  So are types accessible through a
    /// `Mutex`.
    ///
    /// The other type is `Send`.  `Send`'s semantic meaning is "a type that can be sent between
    /// tasks."  For most practical purposes, this means "has no non-`'static` references" (with a
    /// very few exceptions, like the `Rc` type, which are non-`Send` anyway).  The idea is that it
    /// is not safe to send a type between tasks if it has any non-static references, because one
    /// doesn't know when the data it's referencing will be deallocated if it's on some other
    /// task's stack.
    ///
    /// Usually, that's a reasonable assumption.  But it means that if we want to put a slice in a
    /// structure and share it between tasks, it can't be a `&[T]`--it has to be a bare [T].  `Arc`
    /// has `Send` + `Sync` bounds for this reason.  We could also use a `Vec`, but this would
    /// introduce unnecessary indirection.  With dynamically sized types, this problem may be
    /// solved--we can have a dynamically sized `struct` that doesn't contain explicit references.
    /// But until that is fully baked, this seems like the sanest solution.
    ///
    /// Another way to solve this would be associated constants.
    pub const N_BUCKETS: usize = 20;

    /// We don't really have to hardcode the workers.  This is left over from the Go
    /// implementation. All the counting statistics could be moved outside of buckets and probably
    /// should, since they have no influence on the correctness of the algorithm.
    pub const N_WORKERS: usize = 2;

    struct Bucket {
        /// The actual data.  It is atomic because it is read (not written) outside the `Mutex`,
        /// unless a consistent snapshot is required.
        data: AtomicUsize,

        /// The mutex used to synchronize writes and snapshot reads of the
        /// bucket.
        /// As the D solution says, using a per-bucket `Mutex` dramatically
        /// improves scalability compared to the alternatives.
        mutex: Mutex<()>,
    }

    pub struct Buckets {
        /// Buckets containing values to be transferred.
        buckets: [Bucket; N_BUCKETS],

        /// Statistics about total transfers this go-around.
        transfers: [AtomicUsize; N_WORKERS],
    }

    impl Buckets {
        /// Create a new Buckets instance.
        pub fn new(buckets: [usize; N_BUCKETS]) -> Buckets {
            // The unsafe initialization here is required because Bucket is not Clone (it can't be,
            // since neither AtomicUsize nor Mutex are) and we would otherwise have to literally
            // write out N_BUCKETS different values, which would be painful.  As a result, we have
            // to be careful not to allow any failure here, or we'll segfault (by Drop-ing empty
            // buckets).
            let mut buckets_: [Bucket; N_BUCKETS] = unsafe { ::std::mem::uninitialized() };
            let mut transfers: [AtomicUsize; N_WORKERS] = unsafe { ::std::mem::uninitialized() };
            for (dest, &src) in buckets_.iter_mut().zip(buckets.iter()) {
                let bucket = Bucket {
                    data: AtomicUsize::new(src),
                    mutex: Mutex::new(()),
                };
                // If we don't use an unsafe write(), the uninitialized mutex in the bucket will be
                // dropped.
                unsafe { ::std::ptr::write(dest as *mut _, bucket) }
            }
            for t in (&mut transfers[..]).iter_mut() {
                *t = AtomicUsize::new(0);
            }
            Buckets {
                buckets: buckets_,
                transfers: transfers,
            }
        }

        /// Get the value of the bucket at index i, or None if out of bounds.
        pub fn get(&self, i: usize) -> Option<usize> {
            // This is used as an estimate, and is used without the mutex lock, so there's no
            // compelling reason to demand consistency here.
            self.buckets.get(i).map(|b| b.data.load(Ordering::Relaxed))
        }

        /// Transfer at most `amount` from the bucket at index `from` to that at index `to`, and
        /// increment the transfer count for worker `worker` (like I said, that last part can
        /// likely be done elsewhere).
        pub fn transfer(&self, from: usize, to: usize, amount: usize, worker: usize) {
            // The from == to check is important so we don't deadlock, since Rust mutexes are
            // nonreentrant.
            if from == to || N_BUCKETS <= from || N_BUCKETS <= to || N_WORKERS <= worker {
                return;
            }
            // We know this won't fail, and the compiler seems to know as well.  However, if it
            // *did* fail, it wouldn't fail while we were holding mutexes (which can cause
            // problems since they may need to poison other tasks).
            let b1 = &self.buckets[from];
            let b2 = &self.buckets[to];
            // It's very important to lock our Mutexes in the same order everywhere to avoid
            // deadlock.  We arbitrarily choose the convention that we lock in ascending index
            // order.
            let (low, high) = if from < to {
                (b1, b2)
            } else {
                (b2, b1)
            };
            {
                // The reason we introduce a new scope here is that we want to make it clear how
                // long we're locking for.  Locks should be held as briefly as possible and
                // anything that happens here should really *require* the locks.
                let _s1 = low.mutex.lock();
                let _s2 = high.mutex.lock();
                // It is possible that SeqCst is too strong for this section, but it is hard to
                // test on x86 because it has unusually strong consistency by default.
                let v1 = b1.data.load(Ordering::SeqCst);
                let real_amount = ::std::cmp::min(v1, amount);
                b1.data.store(v1 - real_amount, Ordering::SeqCst);
                b2.data.fetch_add(real_amount, Ordering::SeqCst);
            }
            // Doing this outside the critical section increases throughput substantially.  Since
            // this is just a summary statistic, it's okay for it to be a few off.  That's also why
            // we use Acquire semantics rather than AcqRel or SeqCst here--we only really care that
            // we synchronize when the transfer count is set to 0.
            self.transfers[worker].fetch_add(1, Ordering::Acquire);
        }

        /// Acquire a consistent snapshot of the state of the bucket list.  This should maintain
        /// the invariant that total buckets are conserved.  Also returns the list of transfer
        /// counts.
        pub fn snapshot(&self) -> ([usize; N_BUCKETS], [usize; N_WORKERS]) {
            // Since this method is called relatively rarely, we aren't too concerned about
            // performance here.
            let mut buckets = [0; N_BUCKETS];
            let mut transfers = [0; N_WORKERS];
            // We collect all the locks in order, being careful not to drop any until we're done
            // (so as to preserve consistency of the snapshot).
            let locks = buckets.iter_mut().zip(self.buckets.iter()).map( |(dest, src)| {
                let lock = src.mutex.lock();
                // Is SeqCst necessary here?  Maybe, maybe not, but when in doubt go with SeqCst.
                *dest = src.data.load(Ordering::SeqCst);
                lock
            }).collect::<Vec<_>>();
            for (dest, src) in transfers.iter_mut().zip(self.transfers.iter()) {
                // We synchronize with the Acquire in transfer, making sure that our zeroing out
                // gets noticed.
                *dest = src.swap(0, Ordering::Release);
            }
            // We can drop the locks before we return.  This probably gets optimized out, but it's
            // rarely a bad idea to drop locks explicitly.
            drop(locks);
            (buckets, transfers)
        }
    }
}

/// Convenience method to create a distribution of buckets summing to `initial_sum`.
fn make_buckets(initial_sum: usize) -> buckets::Buckets {
    let mut buckets = [0; buckets::N_BUCKETS];
    let mut dist = initial_sum;
    for (i, b) in (&mut buckets[..]).iter_mut().enumerate() {
        let v = dist / (buckets::N_BUCKETS - i);
        *b = v;
        dist -= v;
    }
    buckets::Buckets::new(buckets)
}

/// The equalize task--it chooses two random buckets and tries to make their values the same.
fn equalize(bl: &buckets::Buckets, running: &AtomicBool, worker: usize) {
    // We preallocate the Range for improved performance.
    let between = Range::new(0, buckets::N_BUCKETS);
    // We use the weak random number generator for improved performance.
    let mut r = weak_rng();
    // Running is read Relaxed because it's not important that the task stop right away as long as
    // it happens eventually.
    while running.load(Ordering::Relaxed) {
        let b1 = between.ind_sample(&mut r);
        let b2 = between.ind_sample(&mut r);
        let v1 = bl.get(b1).unwrap();
        let v2 = bl.get(b2).unwrap();
        if v1 > v2 {
            bl.transfer(b1, b2, (v1 - v2) / 2, worker)
        } else {
            bl.transfer(b2, b1, (v2 - v1) / 2, worker)
        }
    }
}

/// The randomize task--it chooses two random buckets and randomly redistributes their values.
fn randomize(bl: &buckets::Buckets, running: &AtomicBool, worker: usize) {
    // We preallocate the Range for improved performance.
    let between = Range::new(0, buckets::N_BUCKETS);
    // We use the weak random number generator for improved performance.
    let mut r = weak_rng();
    // Running is read Relaxed because it's not important that the task stop right away as long as
    // it happens eventually.
    while running.load(Ordering::Relaxed) {
        let b1 = between.ind_sample(&mut r);
        let b2 = between.ind_sample(&mut r);
        bl.transfer(b1, b2, r.gen_range(0, bl.get(b1).unwrap() + 1), worker);
    }
}

/// The display task--for a total time of `duration`, it displays information about the update
/// process and checks to make sure that the invariant (that the total remains constant) is
/// preserved.  It prints an update `nticks` times, evenly spaced.
fn display(bl: &buckets::Buckets,
           running: &AtomicBool,
           original_total: usize,
           duration: Duration,
           nticks: u32) {
    println!("transfers, N. transfers, buckets, buckets sum:");

    let duration = duration / nticks;
    for _ in 0..nticks {
        // Get a consistent snapshot
        let (s, tc) = bl.snapshot();
        // Sum up the buckets
        let sum = s.iter().fold(0, |a, &b| a + b);
        // Sum up the transfers.
        let n_transfers = tc.iter().fold(0, |a, &b| a + b);
        // Print the relevant information.
        println!("{:?}, {}, {:?}, {}", tc, n_transfers, s, sum);
        // Check the invariant, failing if necessary.
        assert_eq!(sum, original_total);
        // Sleep before printing again.
        thread::sleep(duration);
    }
    // We're done--cleanly exit the other update tasks.
    running.store(false, Ordering::Relaxed);
}

/// Putting together all three tasks.
fn perform_atomic_updates(duration: Duration, original_total: usize, num_ticks: u32) {
    // Worker IDs for the two updater tasks.
    const ID_EQUALIZE: usize = 0;
    const ID_RANDOMIZE: usize = 1;

    // `running` is an atomic boolean that we use to signal when to stop to the updater tasks.
    let running = AtomicBool::new(true);
    // We use an Arc here in order to share a reference to the buckets between threads.  Since the
    // Buckets are already Sync, and we don't need &mut references to them, there's no need to
    // create a Mutex here.
    let arc = Arc::new((make_buckets(original_total), running));
    // Cloning the arc bumps the reference count.
    let arc_ = arc.clone();
    // Start off the equalize task
    spawn(move || equalize(&arc_.0, &arc_.1, ID_EQUALIZE));
    let arc_ = arc.clone();
    // Start off the randomize task
    spawn(move || randomize(&arc_.0, &arc_.1, ID_RANDOMIZE));
    let (ref bl, ref running) = *arc;
    // Run the display task in the current thread, so failure propagates to the user.
    display(bl, running, original_total, duration, num_ticks);
}

const ORIGINAL_TOTAL: usize = 1000;
const NUM_TICKS: u32 = 10;

fn main() {
    perform_atomic_updates(Duration::from_secs(10), ORIGINAL_TOTAL, NUM_TICKS);
}

#[test]
fn test_atomic_updates() {
    perform_atomic_updates(Duration::from_secs(1) / 10, ORIGINAL_TOTAL, NUM_TICKS);
}
