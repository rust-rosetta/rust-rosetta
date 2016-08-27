use std::thread;
use std::time::Duration;

fn sleepsort<I>(nums: I)
    where I: Iterator<Item = u64>
{
    let threads: Vec<_> = nums.map(|n| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(n));
                println!("{}", n);
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }
}

fn main() {
    sleepsort(std::env::args().skip(1).map(|s| s.parse().unwrap()));
}
