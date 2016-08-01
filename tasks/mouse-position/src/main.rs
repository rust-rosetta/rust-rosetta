#[cfg(all(target_os = "linux", feature = "x11"))]
extern crate x11;

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
extern crate user32;

use std::thread;
use std::time::Duration;

#[cfg(all(target_os = "linux", feature = "x11"))]
fn get_mouse_position() -> (usize, usize) {
    use std::ffi::CString;
    use std::mem;
    use std::ptr;
    use std::slice;

    use x11::xlib::{self, True, False, AnyPropertyType};

    let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
    if display.is_null() {
        panic!("Could not open display!");
    }

    let active_window = unsafe {
        let mut actual_type_ret = mem::uninitialized();
        let mut actual_format_ret = mem::uninitialized();
        let mut n_items_ret = mem::uninitialized();
        let mut bytes_after_ret = mem::uninitialized();
        let mut prop_ret = mem::uninitialized();

        xlib::XGetWindowProperty(display,
                                 xlib::XDefaultRootWindow(display),
                                 xlib::XInternAtom(display,
                                                   CString::new("_NET_ACTIVE_WINDOW")
                                                       .unwrap()
                                                       .as_ptr(),
                                                   True),
                                 0,
                                 1,
                                 False,
                                 AnyPropertyType as u64,
                                 &mut actual_type_ret,
                                 &mut actual_format_ret,
                                 &mut n_items_ret,
                                 &mut bytes_after_ret,
                                 &mut prop_ret);
        let windows: &[xlib::Window] = slice::from_raw_parts_mut(mem::transmute(prop_ret),
                                                                 n_items_ret as usize);
        windows[0]
    };

    let (x, y) = unsafe {
        let mut root_x = mem::uninitialized();
        let mut root_y = mem::uninitialized();
        let mut win_x = mem::uninitialized();
        let mut win_y = mem::uninitialized();
        let mut mask = mem::uninitialized();
        let mut child_ret = mem::uninitialized();
        let mut root_ret = mem::uninitialized();

        xlib::XQueryPointer(display,
                            active_window,
                            &mut root_ret,
                            &mut child_ret,
                            &mut root_x,
                            &mut root_y,
                            &mut win_x,
                            &mut win_y,
                            &mut mask);

        (win_x, win_y)
    };

    (x as usize, y as usize)
}

#[cfg(all(target_os = "macos"))]
fn get_mouse_position() -> (usize, usize) {
    panic!("unsupported platform!");
}

#[cfg(all(target_os = "linux", not(feature = "x11")))]
fn get_mouse_position() -> (usize, usize) {
    panic!("requires xlib!");
}

#[cfg(windows)]
fn get_mouse_position() -> (i64, i64) {
    use std::mem;

    let h = unsafe { user32::GetForegroundWindow() };

    let (x, y) = unsafe {
        let mut point = mem::uninitialized();
        user32::GetCursorPos(&mut point);
        user32::ScreenToClient(h, &mut point);
        (point.x, point.y)
    };

    (x as i64, y as i64)
}

fn main() {
    loop {
        let (x, y) = get_mouse_position();
        thread::sleep(Duration::from_millis(100));
        println!("x: {}, y: {}", x, y);
    }
}
