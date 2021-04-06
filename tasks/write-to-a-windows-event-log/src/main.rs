#[cfg(windows)]
mod bindings {
    ::windows::include_bindings!();
}

#[cfg(windows)]
use bindings::{
    Windows::Win32::Security::{
        GetTokenInformation, OpenProcessToken, PSID, TOKEN_ACCESS_MASK, TOKEN_INFORMATION_CLASS,
        TOKEN_USER,
    },
    Windows::Win32::SystemServices::{
        GetCurrentProcess, OpenEventLogA, ReportEventA, ReportEvent_wType, HANDLE, PSTR,
    },
};

#[cfg(windows)]
fn main() -> windows::Result<()> {
    let ph = unsafe { GetCurrentProcess() };
    let mut th: HANDLE = HANDLE(0);
    unsafe { OpenProcessToken(ph, TOKEN_ACCESS_MASK::TOKEN_QUERY, &mut th) }.ok()?;

    // Determine the required buffer size, ignore ERROR_INSUFFICIENT_BUFFER
    let mut length = 0_u32;
    unsafe {
        GetTokenInformation(
            th,
            TOKEN_INFORMATION_CLASS::TokenUser,
            std::ptr::null_mut(),
            0,
            &mut length,
        )
    }
    .ok()
    .unwrap_err();

    // Retrieve the user token.
    let mut token_user_bytes = vec![0u8; length as usize];
    unsafe {
        GetTokenInformation(
            th,
            TOKEN_INFORMATION_CLASS::TokenUser,
            token_user_bytes.as_mut_ptr().cast(),
            length,
            &mut length,
        )
    }
    .ok()?;

    // Extract the pointer to the user SID.
    let user_sid: PSID = unsafe { (*token_user_bytes.as_ptr().cast::<TOKEN_USER>()).User.Sid };

    // use the Application event log
    let event_log_handle = unsafe { OpenEventLogA(PSTR::default(), "Application") };

    let mut event_msg = PSTR(b"Hello in the event log\0".as_ptr() as _);
    unsafe {
        ReportEventA(
            HANDLE(event_log_handle.0),               //h_event_log: T0__,
            ReportEvent_wType::EVENTLOG_WARNING_TYPE, // for type use EVENTLOG_WARNING_TYPE w_type: u16,
            5,                                        // for category use "Shell" w_category: u16,
            1,                                        // for ID use 1  dw_event_id: u32,
            user_sid,                                 // lp_user_sid: *mut c_void,
            1,                                        // w_num_strings: u16,
            0,                                        // dw_data_size: u32,
            &mut event_msg,                           // lp_strings: *mut PSTR,
            std::ptr::null_mut(),                     // lp_raw_data: *mut c_void,
        )
    }
    .ok()?;

    Ok(())
}

#[cfg(not(windows))]
fn main() {
    println!("Not implemented");
}
