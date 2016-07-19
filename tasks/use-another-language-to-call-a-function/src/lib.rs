//! In order to run this task, you will need to compile the C program locating in the task linked
//! above. The C program will need to be linked with the library produced by this file.
//!
//! 1. Compile this library:
//!
//!     ```bash
//!     $ cargo build --release
//!     ```
//!
//! 2. Copy the C program into query.c.
//! 3. Compile and link the C program with the produced library:
//!
//!     ```bash
//!     $ LD_LIBRARY_PATH=/path/to/library gcc query.c -o query -Wall -Werror libquery
//!     ```
//! 4. Run the resulting binary.
//!
//!     ```bash
//!     $ LD_LIBRARY_PATH=/path/to/library ./query
//!     Here am I
//!     ```

#![crate_type = "dylib"]

extern crate libc;

use std::ffi::CString;

use libc::{c_char, c_int, size_t};

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Query(data: *mut c_char, length: *mut size_t) -> c_int {
    let string = "Here am I";
    if *length + 1 < string.len() {
        0
    } else {
        let c_string = CString::new(string).unwrap();
        libc::strcpy(data, c_string.as_ptr());
        *length = string.len();
        1
    }
}

#[cfg(test)]
mod tests {
    use super::Query;

    #[test]
    fn buffer_too_small() {
        unsafe {
            const BUF_SIZE: usize = 3;
            let mut buffer = [0; BUF_SIZE];
            assert_eq!(0, Query(buffer.as_mut_ptr(), &mut BUF_SIZE));
        }
    }

    #[test]
    fn buffer_contains_data() {
        use std::ffi::{CStr, CString};

        unsafe {
            const BUF_SIZE: usize = 1024;
            let mut buffer = [0; BUF_SIZE];
            assert_eq!(1, Query(buffer.as_mut_ptr(), &mut BUF_SIZE));
            assert_eq!(CString::new("Here am I").unwrap(),
                       CStr::from_ptr(buffer.as_ptr()).to_owned());
        }
    }
}
