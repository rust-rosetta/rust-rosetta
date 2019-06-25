extern crate libc;

use std::mem;
use std::ptr::copy;

const PAGE_SIZE: usize = 4096;

fn main() {
    let size = 9;
    let bytes: [u8; 9] = [0x8b, 0x44, 0x24, 0x04, 0x03, 0x44, 0x24, 0x08, 0xc3];
    let f: fn(u8, u8) -> u8 = unsafe {
        let mut page: *mut libc::c_void = mem::uninitialized();
        libc::posix_memalign(&mut page, PAGE_SIZE, size);
        libc::mprotect(
            page,
            size,
            libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE,
        );
        let contents: *mut u8 = mem::transmute(page);
        copy(bytes.as_ptr(), contents, 9);
        mem::transmute(contents)
    };

    println!("{}", f(7, 12));
}
