extern crate libc;

use libc::c_char;
use std::ffi::CString;

/// C functions are declared in an `extern "C"` block.
extern "C" {
    fn strcmp(a: *const c_char, b: *const c_char) -> i32;
}

fn main() {
    let a = CString::new("a").unwrap();
    let b = CString::new("b").unwrap();

    println!("{}", unsafe { strcmp(a.as_ptr(), b.as_ptr()) });
}

#[test]
fn test_strcmp() {
    let a = CString::new("a").unwrap();
    let b = CString::new("b").unwrap();

    assert_eq!(unsafe { strcmp(a.as_ptr(), b.as_ptr()) }, -1);
}
