extern crate libc;

#[cfg(all(
    target_os = "linux",
    any(target_pointer_width = "32", target_pointer_width = "64")
))]
fn main() {
    use std::mem;
    use std::ptr;

    let page_size: usize = 4096;
    let (bytes, size): (Vec<u8>, usize) = if cfg!(target_pointer_width = "32") {
        (
            vec![0x8b, 0x44, 0x24, 0x04, 0x03, 0x44, 0x24, 0x08, 0xc3],
            9,
        )
    } else {
        (vec![0x48, 0x89, 0xf8, 0x48, 0x01, 0xf0, 0xc3], 7)
    };
    let f: fn(u8, u8) -> u8 = unsafe {
        let mut page: *mut libc::c_void = ptr::null_mut();
        libc::posix_memalign(&mut page, page_size, size);
        libc::mprotect(
            page,
            size,
            libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE,
        );
        let contents: *mut u8 = page as *mut u8;
        ptr::copy(bytes.as_ptr(), contents, 9);
        mem::transmute(contents)
    };

    let return_value = f(7, 12);
    println!("Returned value: {}", return_value);
    assert_eq!(return_value, 19);
}

#[cfg(any(
    not(target_os = "linux"),
    not(any(target_pointer_width = "32", target_pointer_width = "64"))
))]
fn main() {
    println!("Not supported on this platform.");
}
