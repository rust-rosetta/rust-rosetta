// http://rosettacode.org/wiki/Call_a_foreign-language_function
extern crate libc;

use libc::c_char;
use std::ffi::CString;

extern "C" {
    // C functions are declared in an `extern "C"` block.
    fn strcmp(a: *const c_char, b: *const c_char) -> i32;
}

#[cfg(not(test))]
fn main() {
    let a = CString::new("a").unwrap();
    let b = CString::new("b").unwrap();

    println!("{}", unsafe {
        strcmp(a.as_ptr(), b.as_ptr())
    });
}

#[cfg(test)]
#[test]
fn test_strcmp() {
    let a = CString::new("a").unwrap();
    let b = CString::new("b").unwrap();

    assert_eq!(unsafe {
        strcmp(a.as_ptr(), b.as_ptr())
    }, -1);
}
