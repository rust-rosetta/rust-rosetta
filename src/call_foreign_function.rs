// Implements http://rosettacode.org/wiki/Call_a_foreign-language_function
#![cfg(not_tested)]

extern crate libc;

use libc::c_char;
use std::c_str::CString;

extern "C" {
    // C functions are declared in an `extern "C"` block.
    fn strdup(s: *c_char) -> *c_char;
}

fn main() {
    // Create a Rust static string. No allocations.
    let rust_str = "Hello World!";
    // Call strdup. C functions are considered possibly unsafe to call, so
    // an unsafe block is needed.
    let dup_c_str = unsafe {
        // Use the `with_c_str` method to create a nul-terminated C string. This
        // results in an allocation in order to add the nul. If you placed an
        // explicit nul inside the Rust string, you could avoid the allocation
        // by using the `with_c_str_unchecked` method. Both methods accept a
        // closure that is called with the nul-terminated C string.
        rust_str.with_c_str(|c_str| {
            strdup(c_str)
        })
    };
    // Wrap the C string in the `CString` struct. We instruct the `CString` to
    // take ownership of the C string, so it will free it automatically when
    // the scope ends using the C `free` function. Creating a `CString` is
    // possibly unsafe because you could pass it an invalid address.
    let wrap_dup_c_str = unsafe {
        CString::new(dup_c_str, true)
    };
    // Get a Rust string slice out of the wrapper. The `as_str` method returns
    // Option<&str> because you may have passed invalid UTF-8 encoded C string
    // to `CString`. This does not allocate memory.
    let dup_rust_str = wrap_dup_c_str.as_str().unwrap();
    // Now you can easily print the result
    println!("{}", dup_rust_str);
    // The block ends here, and `CString` frees the C string we created.
}
