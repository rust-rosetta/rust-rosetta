//! Rust has a perfectly good Semaphore type already. It lacks count(), though, so we can't use it
//! directly.

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::thread::{self, spawn};
use std::time::Duration;
use std::sync::mpsc::channel;

pub struct CountingSemaphore {
    /// Remaining resource count
    count: AtomicUsize,

    /// How long to sleep if a resource is being contended
    backoff: Duration,
}

pub struct CountingSemaphoreGuard<'a> {
    /// A reference to the owning semaphore.
    sem: &'a CountingSemaphore,
}

impl CountingSemaphore {
    /// Create a semaphore with `max` available resources and a linearly increasing backoff of
    /// `backoff` (used during spinlock contention).
    pub fn new(max: usize, backoff: Duration) -> CountingSemaphore {
        CountingSemaphore {
            count: AtomicUsize::new(max),
            backoff: backoff,
        }
    }

    /// Acquire a resource, returning a RAII CountingSemaphoreGuard.
    pub fn acquire(&self) -> CountingSemaphoreGuard {
        // Spinlock until remaining resource count is at least 1
        let mut backoff = self.backoff;
        loop {
            // Probably don't need SeqCst here, but it doesn't hurt.
            let count = self.count.load(SeqCst);
            // The check for 0 is necessary to make sure we don't go negative, which is why this
            // must be a compare-and-swap rather than a straight decrement.
            if count == 0 || self.count.compare_and_swap(count, count - 1, SeqCst) != count {
                // Linear backoff a la Servo's spinlock contention.
                thread::sleep(backoff);
                backoff += self.backoff;
            } else {
                // We successfully acquired the resource.
                break;
            }
        }
        CountingSemaphoreGuard { sem: self }
    }

    // Return remaining resource count
    pub fn count(&self) -> usize {
        self.count.load(SeqCst)
    }
}

impl<'a> Drop for CountingSemaphoreGuard<'a> {
    /// When the guard is dropped, a resource is released back to the pool.
    fn drop(&mut self) {
        self.sem.count.fetch_add(1, SeqCst);
    }
}

fn metered(duration: Duration) {
    static MAX_COUNT: usize = 4; // Total available resources
    static NUM_WORKERS: u8 = 10; // Number of workers contending for the resources
    let backoff = Duration::from_millis(1); // Linear backoff time
    // Create a shared reference to the semaphore
    let sem = Arc::new(CountingSemaphore::new(MAX_COUNT, backoff));
    // Create a channel for notifying the main task that the workers are done
    let (tx, rx) = channel();
    for i in 0..NUM_WORKERS {
        let sem = sem.clone();
        let tx = tx.clone();
        spawn(move || -> () {
            // Acquire the resource
            let guard = sem.acquire();
            let count = sem.count();
            // Make sure the count is legal
            assert!(count < MAX_COUNT);
            println!("Worker {} after acquire: count = {}", i, count);
            // Sleep for `duration`
            thread::sleep(duration);
            // Release the resource
            drop(guard);
            // Make sure the count is legal
            let count = sem.count();
            assert!(count <= MAX_COUNT);
            println!("Worker {} after release: count = {}", i, count);
            // Notify the main task of completion
            tx.send(()).unwrap();
        });
    }
    drop(tx);
    // Wait for all the subtasks to finish
    for _ in 0..NUM_WORKERS {
        rx.recv().unwrap();
    }
}

#[test]
fn test_metered_concurrency() {
    // Hold each resource for 1/20 of a second per worker
    metered(Duration::from_secs(1) / 20);
}

fn main() {
    // Hold each resource for 2 seconds per worker
    metered(Duration::from_secs(2));
}
