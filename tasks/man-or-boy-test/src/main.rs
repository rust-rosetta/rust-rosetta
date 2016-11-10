//! As originally [posted] by Kimundi on Reddit
//! [posted]: http://www.reddit.com/r/rust/comments/2t80mw/the_man_or_boy_test_in_rust/

use std::cell::Cell;

fn a(k: i32,
     x1: &Fn() -> i32,
     x2: &Fn() -> i32,
     x3: &Fn() -> i32,
     x4: &Fn() -> i32,
     x5: &Fn() -> i32)
     -> i32 {
    let k = Cell::new(k);

    let (b, tmp): (Cell<Option<&Fn() -> i32>>, _);
    b = Cell::new(None);
    tmp = || {
        k.set(k.get() - 1);
        a(k.get(), &*b.get().unwrap(), x1, x2, x3, x4)
    };
    b.set(Some(&tmp));

    if k.get() <= 0 {
        x4() + x5()
    } else {
        b.get().unwrap()()
    }
}

fn main() {
    println!("%{}", a(10, &|| 1, &|| -1, &|| -1, &|| 1, &|| 0));
}

#[test]
fn result() {
    assert_eq!(a(10, &|| 1, &|| -1, &|| -1, &|| 1, &|| 0), -67)
}
