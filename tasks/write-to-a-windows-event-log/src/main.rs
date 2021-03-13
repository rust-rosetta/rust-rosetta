#[cfg(target_os = "windows")]
mod bindings {
    ::windows::include_bindings!();
}

const TOKEN_READ: u32 = 0x00020008;
const EVENTLOG_WARNING_TYPE: u16 = 2;

#[cfg(target_os = "windows")]
fn main() {
    use bindings::windows::win32::security::*;
    use bindings::windows::win32::system_services::*;

    unsafe {
        let ph = GetCurrentProcess();
        let mut th: HANDLE = HANDLE(0);
        OpenProcessToken(ph, TOKEN_READ, &mut th);

        let mut ti = std::ptr::null_mut();
        let mut length: u32 = 0;
        GetTokenInformation(th, TOKEN_INFORMATION_CLASS::TokenUser, ti, 0, &mut length);

        // use the Application event log
        let event_log_handle = OpenEventLogA(PSTR::default(), "Application");

        let mut event_msg = PSTR(b"Hello in the event log\0".as_ptr() as _);
        ReportEventA(
            HANDLE(event_log_handle.0), //h_event_log: T0__,
            EVENTLOG_WARNING_TYPE,      // for type use EVENTLOG_WARNING_TYPE w_type: u16,
            5,                          // for category use "Shell" w_category: u16,
            1,                          // for ID use 1  dw_event_id: u32,
            ti,                         // lp_user_sid: *mut c_void,
            1,                          // w_num_strings: u16,
            0,                          // dw_data_size: u32,
            &mut event_msg,             // lp_strings: *mut PSTR,
            std::ptr::null_mut(),       // lp_raw_data: *mut c_void,
        );
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("Not implemented");
}
