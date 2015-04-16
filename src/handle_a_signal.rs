// Implements http://rosettacode.org/wiki/Handle_a_signal
//
// Note that this solution only works on Unix.
#![feature(libc)]
#![cfg_attr(unix, feature(std_misc))]

extern crate libc;
extern crate time;

#[cfg(all(unix, not(test)))]
fn main()
{
    use libc::consts::os::posix88::SIGINT;
    use libc::funcs::posix01::signal;
    use std::mem;
    use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
    use std::time::duration::Duration;

    // The time between ticks of our counter.
    let duration = 500u32; //Duration::seconds(1) / 2;
    // "SIGINT received" global variable.
    static mut GOT_SIGINT: AtomicBool = ATOMIC_BOOL_INIT;
    unsafe {
        // Initially, "SIGINT received" is false.
        GOT_SIGINT.store(false, Ordering::Release);
        // Interrupt handler that handles the SIGINT signal
        unsafe fn handle_sigint() {
            // It is dangerous to perform any system calls in interrupts, so just set the atomic
            // "SIGINT received" global to true when it arrives.
            GOT_SIGINT.store(true, Ordering::Release);
        }
        // Make handle_sigint the signal handler for SIGINT.
        signal::signal(SIGINT, mem::transmute(handle_sigint));
    }
    // Get the start time...
    let start = time::precise_time_ns();
    // Integer counter
    let mut i = 0u32;
    // Every `duration`...
    loop {
        std::thread::sleep_ms(duration);
        // Break if SIGINT was handled
        if unsafe { GOT_SIGINT.load(Ordering::Acquire) } { break }
        // Otherwise, increment and display the integer and continue the loop.
        i += 1;
        println!("{}", i);
    }
    // Get the end time.
    let end = time::precise_time_ns();
    // Compute the difference
    let diff = Duration::nanoseconds((end - start) as i64);
    // Print the difference and exit
    println!("Program has run for {} seconds", diff);
}

#[cfg(not(unix))]
fn main()
{
    println!("Not supported on this platform");
}
