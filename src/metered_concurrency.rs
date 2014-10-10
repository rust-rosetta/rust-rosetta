// Rust has a perfectly good Semaphore type already.  It lacks count(), though, so we can't use it
// directly.
#![feature(unsafe_destructor)]

extern crate sync;

use std::io::timer;
use std::sync::Arc;
use std::sync::atomic::AtomicUint;
use std::sync::atomics;
use std::time::duration::Duration;

pub struct CountingSemaphore {
    count: AtomicUint, // Remaining resource count
    backoff: Duration, // How long to sleep if a resource is being contended
}

pub struct CountingSemaphoreGuard<'a> {
    sem: &'a CountingSemaphore,
}

impl CountingSemaphore {
    pub fn new(max: uint, backoff: Duration) -> CountingSemaphore {
        CountingSemaphore { count: AtomicUint::new(max), backoff: backoff }
    }

    pub fn acquire(&self) -> CountingSemaphoreGuard {
        // Spinlock, more or less
        loop {
            let count = self.count.load(atomics::SeqCst);
            if count == 0 || self.count.compare_and_swap(count, count - 1, atomics::SeqCst) != count {
                timer::sleep(self.backoff);
            } else {
                break
            }
        }
        CountingSemaphoreGuard { sem: self }
    }

    pub fn count(&self) -> uint {
        self.count.load(atomics::SeqCst)
    }
}

#[unsafe_destructor]
impl<'a> Drop for CountingSemaphoreGuard<'a> {
    // When the guard is dropped, a resource is released back to the pool.
    fn drop(&mut self) {
        self.sem.count.fetch_add(1, atomics::SeqCst);
    }
}

fn metered(duration: Duration, backoff: Duration) {
    static MAX_COUNT: uint = 4;
    static NUM_THREADS: u8 = 10;
    let sem = Arc::new(CountingSemaphore::new(MAX_COUNT, backoff));
    let (tx, rx) = channel();
    for i in range(0, NUM_THREADS) {
        let sem = sem.clone();
        let tx = tx.clone();
        spawn(proc() {
            let guard = sem.acquire();
            let count = sem.count();
            assert!(count <= MAX_COUNT);
            println!("Worker {} after acquire: count = {}", i, count);
            timer::sleep(duration);
            drop(guard);
            let count = sem.count();
            assert!(count <= MAX_COUNT);
            println!("Worker {} after release: count = {}", i, count);
            tx.send(());
        })
    }
    drop(tx);
    for _ in range(0, NUM_THREADS) {
        rx.recv();
    }
}

#[test]
fn test_metered_concurrency() {
    metered(Duration::seconds(1) / 20, Duration::seconds(1) / 20);
}

#[cfg(not(test))]
fn main() {
    metered(Duration::seconds(2), Duration::seconds(1) / 10);
}
