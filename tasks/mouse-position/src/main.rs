use std::thread;
use std::time::Duration;

#[cfg(all(target_os = "linux", feature = "x11"))]
fn get_mouse_position() -> (i64, i64) {
    use std::ffi::CString;
    use std::ptr;
    use std::slice;

    use x11::xlib::{self, AnyPropertyType, False, True};

    let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
    if display.is_null() {
        panic!("Could not open display!");
    }

    let active_window = unsafe {
        let mut type_ = 0;
        let mut format = 0;
        let mut n_items = 0;
        let mut bytes_after = 0;
        let mut prop = ptr::null_mut();

        let property_name = CString::new("_NET_ACTIVE_WINDOW").unwrap();
        let property_name_atom = xlib::XInternAtom(display, property_name.as_ptr(), True);

        xlib::XGetWindowProperty(
            display,
            xlib::XDefaultRootWindow(display),
            property_name_atom,
            0,
            1,
            False,
            AnyPropertyType as u64,
            &mut type_,
            &mut format,
            &mut n_items,
            &mut bytes_after,
            &mut prop,
        );
        let windows: &[xlib::Window] =
            slice::from_raw_parts_mut(prop as *mut xlib::Window, n_items as usize);
        windows[0]
    };

    let (x, y) = unsafe {
        let mut root = 0;
        let mut child = 0;
        let mut root_x = 0;
        let mut root_y = 0;
        let mut win_x = 0;
        let mut win_y = 0;
        let mut mask = 0;

        xlib::XQueryPointer(
            display,
            active_window,
            &mut root,
            &mut child,
            &mut root_x,
            &mut root_y,
            &mut win_x,
            &mut win_y,
            &mut mask,
        );

        (win_x, win_y)
    };

    unsafe {
        xlib::XCloseDisplay(display);
    }

    (x as i64, y as i64)
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
    use windows::Win32::{
        Foundation::POINT,
        Graphics::Gdi::ScreenToClient,
        UI::WindowsAndMessaging::{GetCursorPos, GetForegroundWindow},
    };

    let h = unsafe { GetForegroundWindow() };

    let (x, y) = unsafe {
        let mut point = POINT::default();
        GetCursorPos(&mut point);
        ScreenToClient(h, &mut point);
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
