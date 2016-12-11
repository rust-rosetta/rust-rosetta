//! We implement this task using Rust's Barriers.  Barriers are simply thread synchronization
//! points--if a task waits at a barrier, it will not continue until the number of tasks for which
//! the variable was initialized are also waiting at the barrier, at which point all of them will
//! stop waiting.  This can be used to allow threads to do asynchronous work and guarantee
//! properties at checkpoints.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Barrier};
use std::thread::spawn;
use std::sync::mpsc::channel;


pub fn checkpoint() {
    const NUM_TASKS: usize = 10;
    const NUM_ITERATIONS: u8 = 10;

    let barrier = Barrier::new(NUM_TASKS);
    let mut events: [AtomicBool; NUM_TASKS];
    unsafe {
        // Unsafe because it's hard to initialize arrays whose type is not Clone.
        events = ::std::mem::uninitialized();
        for e in &mut events {
            // Events are initially off
            *e = AtomicBool::new(false);
        }
    }
    // Arc for sharing between tasks
    let arc = Arc::new((barrier, events));
    // Channel for communicating when tasks are done
    let (tx, rx) = channel();
    for i in 0..NUM_TASKS {
        let arc = arc.clone();
        let tx = tx.clone();
        // Spawn a new worker
        spawn(move || -> () {
            let (ref barrier, ref events) = *arc;
            // Assign an event to this task
            let event = &events[i];
            // Start processing events
            for _ in 0..NUM_ITERATIONS {
                // Between checkpoints 4 and 1, turn this task's event on.
                event.store(true, Ordering::Release);
                // Checkpoint 1
                barrier.wait();
                // Between checkpoints 1 and 2, all events are on.
                assert!(events.iter().all(|e| e.load(Ordering::Acquire)));
                // Checkpoint 2
                barrier.wait();
                // Between checkpoints 2 and 3, turn this task's event off.
                event.store(false, Ordering::Release);
                // Checkpoint 3
                barrier.wait();
                // Between checkpoints 3 and 4, all events are off.
                assert!(events.iter().all(|e| !e.load(Ordering::Acquire)));
                // Checkpoint 4
                barrier.wait();
            }
            // Finish processing events.
            tx.send(()).unwrap();
        });
    }
    drop(tx);
    // The main thread will not exit until all tasks have exited.
    for _ in 0..NUM_TASKS {
        rx.recv().unwrap();
    }
}

fn main() {
    checkpoint();
}

#[test]
fn test_checkpoint() {
    checkpoint();
}
