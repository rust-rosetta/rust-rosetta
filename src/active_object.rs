// Implements http://rosettacode.org/wiki/Active_object
#![feature(std_misc)]
#![feature(old_io)]
#![feature(core)]

extern crate time;

use std::num::Float;
use std::f64::consts::PI;
use std::old_io::timer::Timer;
use std::sync::{Arc, Mutex};
use std::time::duration::Duration;
use std::thread::spawn;
use std::sync::mpsc::{channel, Sender, SendError};
use std::ops::Mul;

// Rust supports both shared-memory and actor models of concurrency, and the Integrator utilizes
// both.  We use a Sender (actor model) to send the Integrator new functions, while we use a Mutex
// (shared-memory concurrency) to hold the result of the integration.
//
// Note that these are not the only options here--there are many, many ways you can deal with
// concurrent access.  But when in doubt, a plain old Mutex is often a good bet.  For example, this
// might look like a good situation for a RwLock--after all, there's no reason for a read in the
// main task to block writes.  Unfortunately, unless you have significantly more reads than
// writes (which is certainly not the case here), a Mutex will usually outperform a RwLock.
pub struct Integrator<S: 'static, T> {
    input: Sender<Box<Fn(i64) -> S + Send>>,
    output: Arc<Mutex<T>>,
}

// In Rust, time durations are strongly typed.  This is usually exactly what you want, but for a
// problem like this--where the integrated value has unusual (unspecified?) units--it can actually
// be a bit tricky.  Right now, Durations can only be multiplied or divided by i32s, so in order to
// be able to actually do math with them we say that the type parameter S (the result of the
// function being integrated) must yield T (the type of the integrated value) when multiplied by
// f64.  We could possibly replace f64 with a generic as well, but it would make things a bit more
// complex.
impl<S: Mul<f64, Output=T> + Float,
     T: 'static + Clone + Send + Float> Integrator<S, T> {
    pub fn new(frequency: Duration) -> Integrator<S, T> {
        // We create a pipe allowing functions to be sent from tx (the sending end) to input (the
        // receiving end).  In order to change the function we are integrating from the task in
        // which the Integrator lives, we simply send the function through tx.
        let (tx, input) = channel();
        // The easiest way to do shared-memory concurrency in Rust is to use atomic reference
        // counting, or Arc, around a synchronized type (like Mutex<T>).  Arc gives you a guarantee
        // that memory will not be freed as long as there is at least one reference to it.
        // It is similar to C++'s shared_ptr, but it is guaranteed to be safe and is never
        // incremented unless explicitly cloned (by default, it is moved).
        let s: Arc<Mutex<T>> = Arc::new(Mutex::new(Float::zero()));
        let integrator = Integrator {
            input: tx,
            // Here is the aforementioned clone.  We have to do it before s enters the closure,
            // because once that happens it is moved into the closure (and later, the new task) and
            // becomes inaccessible to the outside world.
            output: s.clone(),
        };
        spawn(move || -> () {
            // There is a reasonable argument for failure on failure to initialize the Timer, but
            // it's essentially certain that if we return from the spawn this early main will
            // detect that something is amiss.
            let mut timer = match Timer::new() {
                Ok(timer) => timer,
                Err(_) => return
            };
            // The frequency is how often we want to "tick" as we update our integrated total.  In
            // Rust, timers can yield Receivers that are periodically notified with an empty
            // message (where the period is the frequency).  This is useful because it lets us wait
            // on either a tick or another type of message (in this case, a request to change the
            // function we are integrating).
            let periodic = timer.periodic(frequency);
            let frequency_ms = frequency.num_milliseconds();
            let mut t = 0;
            let mut k: Box<Fn(i64) -> S + Send> = Box::new(|_| Float::zero());
            let mut k_0: S = Float::zero();
            loop {
                // Here's the selection we talked about above.  Note that we are careful to call
                // the *non*-failing function, recv(), here.  The reason we do this is because
                // recv() will return Err when the sending end of a channel is dropped.  While
                // this is unlikely to happen for the timer (so again, you could argue for failure
                // there), it's normal behavior for the sending end of input to be dropped, since
                // it just happens when the Integrator falls out of scope.  So we handle it cleanly
                // and break out of the loop, rather than failing.
                select! {
                    res = periodic.recv() => match res {
                        Ok(_) => {
                            t += frequency_ms;
                            let k_1: S = k(t);
                            // Rust Mutexes are a bit different from Mutexes in many other
                            // languages, in that the protected data is actually encapsulated by
                            // the Mutex.  The reason for this is that Rust is actually capable of
                            // enforcing (via its borrow checker) the invariant that the contents
                            // of a Mutex may only be read when you have acquired its lock.  This
                            // is enforced by way of a MutexGuard, the return value of lock(),
                            // which implements some special traits (Deref and DerefMut) that allow
                            // access to the inner element "through" the guard.  The element so
                            // acquired has a lifetime bounded by that of the MutexGuard, the
                            // MutexGuard can only be acquired by taking a lock, and the only way
                            // to release the lock is by letting the MutexGuard fall out of scope,
                            // so it's impossible to access the data incorrectly.  There are some
                            // additional subtleties around the actual implementation, but that's
                            // the basic idea.
                            let mut s = s.lock().unwrap();
                            *s = *s + (k_1 + k_0) * (frequency_ms as f64 / 2.);
                            k_0 = k_1;
                        }
                        Err(_) => break,
                    },
                    res = input.recv() => match res {
                        Ok(k_new) => k = k_new,
                        Err(_) => break,
                    }
                }
            }
        });
        integrator
    }

    pub fn input(&self, k: Box<Fn(i64) -> S + Send>) ->
                Result<(), SendError<Box<Fn(i64) -> S + Send>>> {
        // The meat of the work is done in the other thread, so to set the
        // input we just send along the Sender we set earlier...
        self.input.send(k)
    }

    pub fn output(&self) -> T {
        // ...and to read the input, we simply acquire a lock on the output Mutex and return a
        // clone.  Why do we have to clone it?  Because, as mentioned above, Rust won't let us
        // retain access to the interior of the Mutex unless we have possession of its lock.  There
        // are ways and circumstances in which one can avoid this (e.g. by using atomic types) but
        // clone() is a perfectly reasonable solution as well, and a lot easier to reason about :)
        self.output.lock().unwrap().clone()
    }
}

// This function is fairly straightforward.  We create the integrator, set its input function k(t)
// to 2pi * f * t, and then wait as described in the Rosetta stone problem.
#[cfg(not(test))]
fn integrate() -> f64 {
    let object = Integrator::new(Duration::milliseconds(10));
    let mut timer = Timer::new().unwrap();
    object.input(Box::new(|t: i64| {
        let f = 1. / Duration::seconds(2).num_milliseconds() as f64;
        (2. * PI * f * t as f64).sin()
    })).ok().expect("Failed to set input");
    timer.sleep(Duration::seconds(2));
    object.input(Box::new(|_| 0.)).ok().expect("Failed to set input");
    timer.sleep(Duration::seconds(1) / 2);
    object.output()
}

#[cfg(not(test))]
fn main() {
    println!("{}", integrate());
}

#[test]
#[ignore] // Will fail on a heavily loaded machine
fn solution() {
    // We should just be able to call integrate, but can't represent the closure properly due to
    // rust-lang/rust issue #17060 if we make frequency or period a variable.
    // FIXME(pythonesque): When unboxed closures are fixed, fix integrate() to take two arguments.
    let object = Integrator::new(Duration::milliseconds(10));
    let mut timer = Timer::new().unwrap();
    object.input(Box::new(|t: i64| {
        let f = 1. / (Duration::seconds(2) / 10).num_milliseconds() as f64;
        (2. * PI * f * t as f64).sin()
    })).ok().expect("Failed to set input");
    timer.sleep(Duration::seconds(2) / 10);
    object.input(Box::new(|_| 0.)).ok().expect("Failed to set input");
    timer.sleep(Duration::seconds(1) / 10);
    assert_eq!(object.output() as i64, 0)
}
