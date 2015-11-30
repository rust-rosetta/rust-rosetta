// http://rosettacode.org/wiki/Use_another_language_to_call_a_function
//
// In order to run this task, you will need to compile the C program locating in the task linked
// above. The C program will need to be linked with the library produced by this file.
//
// 1. Compile this library:
//      $ rustc -L /path/to/folder/containing/libc src/use_another_language_to_call_a_function.rs \
//          -o libquery
// 2. Copy the C program into query.c.
// 3. Compile and link the C program with the produced library:
//      $ LD_LIBRARY_PATH=/path/to/libquery gcc query.c -o query -Wall -Werror libquery
// 4. Run the resulting binary.
//      $ LD_LIBRARY_PATH=/path/to/libquery ./query
//      Here am I
#![crate_type = "dylib"]
extern crate libc;

use std::ffi::CString;

use libc::{c_char, c_int, size_t};

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern fn Query(data: *mut c_char, length: *mut size_t) -> c_int {
    let string = "Here am I";
    if *length + 1 < string.len() {
        0
    } else {
        libc::strcpy(data, CString::new(string).unwrap().as_ptr());
        *length = string.len();
        1
    }
}
