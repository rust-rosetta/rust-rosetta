extern crate time;

use std::f64::consts::PI;
use std::io::timer::Timer;
use std::num::{Zero, zero};
use std::sync::{Arc, Mutex};
use std::time::duration::Duration;

pub struct Integrator<S, T> {
    input: Sender<|i64|: Send -> S>,
    output: Arc<Mutex<T>>,
}

impl<S: Mul<f64, T> + Zero,
     T: Clone + Send + Zero> Integrator<S, T> {
    pub fn new(frequency: Duration) -> Integrator<S, T> {
        let (tx, input) = channel();
        let s = Arc::new(Mutex::new(zero::<T>()));
        let integrator = Integrator {
            input: tx,
            output: s.clone(),
        };
        spawn(proc() {
            let mut timer = match Timer::new() {
                Ok(timer) => timer,
                Err(_) => return
            };
            let periodic = timer.periodic(frequency);
            let frequency_ms = frequency.num_milliseconds();
            let mut t = 0;
            let mut k = |_| zero();
            let mut k_0: S = zero();
            loop {
                select! {
                    res = periodic.recv_opt() => match res {
                        Ok(_) => {
                            t += frequency_ms;
                            let k_1: S = k(t);
                            let mut s = s.lock();
                            *s = *s + (k_1 + k_0) * (frequency_ms as f64 / 2.);
                            k_0 = k_1;
                        }
                        Err(_) => break,
                    },
                    res = input.recv_opt() => match res {
                        Ok(k_new) => k = k_new,
                        Err(_) => break,
                    }
                }
            }
        });
        integrator
    }

    pub fn input(&self, k: |i64|: Send -> S) -> Result<(), |i64|: Send -> S> {
        self.input.send_opt(k)
    }

    pub fn output(&self) -> T {
        self.output.lock().clone()
    }
}

fn integrate() -> f64 {
    let object = Integrator::new(Duration::milliseconds(10));
    let mut timer = Timer::new().unwrap();
    object.input(|t| {
        let f = 1. / Duration::seconds(2).num_milliseconds() as f64;
        (2. * PI * f * t as f64).sin()
    }).ok().expect("Failed to set input");
    timer.sleep(Duration::seconds(2));
    object.input(|_| 0.).ok().expect("Failed to set input");
    timer.sleep(Duration::seconds(1) / 2);
    object.output()
}

#[cfg(not(test))]
fn main() {
    println!("{}", integrate());
}

#[test]
fn solution() {
    assert_eq!(integrate() as i64, 0);
}
