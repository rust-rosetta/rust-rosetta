// Implements http://rosettacode.org/wiki/Call_a_foreign-language_function

extern crate libc;

use libc::c_char;
use std::ffi::CString;

extern "C" {
    // C functions are declared in an `extern "C"` block.
    fn strcmp(a: *const c_char, b: *const c_char) -> isize;
}

#[cfg(test)]
#[test]
fn test_strcmp() {
    let a = CString::from_slice(b"a");
    let b = CString::from_slice(b"b");

    assert_eq!(strcmp(a.as_ptr(), b.as_ptr()), -1);
}
