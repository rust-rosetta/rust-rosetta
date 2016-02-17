// http://rosettacode.org/wiki/Use_another_language_to_call_a_function
extern crate use_another_language_to_call_a_function;

#[test]
fn buffer_too_small() {
    unsafe {
        const BUF_SIZE: usize = 3;
        let mut buffer = [0; BUF_SIZE];
        assert_eq!(0,
                   use_another_language_to_call_a_function::Query(buffer.as_mut_ptr(),
                                                                  &mut BUF_SIZE));
    }
}

#[test]
fn buffer_contains_data() {
    use std::ffi::{CStr, CString};

    unsafe {
        const BUF_SIZE: usize = 1024;
        let mut buffer = [0; BUF_SIZE];
        assert_eq!(1,
                   use_another_language_to_call_a_function::Query(buffer.as_mut_ptr(),
                                                                  &mut BUF_SIZE));
        assert_eq!(CString::new("Here am I").unwrap(),
                   CStr::from_ptr(buffer.as_ptr()).to_owned());
    }
}

fn main() {
    println!("This task is not a binary.");
    println!("Please see the comment at the top of \
              src/use_another_language_to_call_a_function/src/lib.rs.");
}
