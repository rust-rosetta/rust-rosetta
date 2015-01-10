// Implements http://rosettacode.org/wiki/Call_a_foreign-language_function

extern crate libc;

use libc::c_char;
use std::ffi::{self, CString};

extern "C" {
    // C functions are declared in an `extern "C"` block.
    fn strdup(s: *const c_char) -> *const c_char;
}

fn main() {
    // Create a Rust static string. No allocations.
    let rust_str = "Hello World!";

    // Use the `from_slice` method to create a nul-terminated C string 
    // and wrap it in a CString. Then call strdup.
    let dup_c_str = {
        let c_str = CString::from_slice(rust_str.as_bytes()); 
        unsafe { strdup(c_str.as_ptr()) }
    };
    
    // read the duplicated c_string into a rust String
    let c_str_as_bytes = unsafe { ffi::c_str_to_bytes(&dup_c_str) };
    let dup_rust_string = String::from_utf8_lossy(c_str_as_bytes);
    
    // free the duplicate c string
    unsafe { libc::free(dup_c_str as *mut _) };

    // Now you can easily print the result
    println!("{:?}", dup_rust_string);
}
