#![feature(maybe_uninit_as_bytes)]

#[cfg(windows)]
use std::ffi::CString;
#[cfg(windows)]
use std::ptr;

#[cfg(windows)]
use windows::{
    core::*,
    Win32::{
        Foundation::{ERROR_INSUFFICIENT_BUFFER, HANDLE, WIN32_ERROR},
        Security::{GetTokenInformation, TokenUser, TOKEN_QUERY, TOKEN_USER},
        System::{
            EventLog::{EventSourceHandle, OpenEventLogA, ReportEventA, EVENTLOG_WARNING_TYPE},
            Threading::{GetCurrentProcess, OpenProcessToken},
        },
    },
};

#[cfg(windows)]
fn main() -> Result<()> {
    let ph = unsafe { GetCurrentProcess() };
    let mut th: HANDLE = HANDLE(0);
    unsafe { OpenProcessToken(ph, TOKEN_QUERY, &mut th) }.ok()?;

    // Determine the required buffer size for the TOKEN_USER. This buffer must also include
    // data that the TOKEN_USER points to, so we can't just pass a MaybeUninit<TOKEN_USER>.
    // Instead, we first call GetTokenInformation with a zero-sized buffer to determine the
    // required buffer size.
    let mut token_user: *mut TOKEN_USER = ptr::null_mut();
    let mut length = 0;

    let err = unsafe { GetTokenInformation(th, TokenUser, token_user as _, 0, &mut length) }
        .ok()
        .unwrap_err();
    assert!(WIN32_ERROR::from_error(&err) == Some(ERROR_INSUFFICIENT_BUFFER));

    // Retrieve the user token.
    let mut token_buffer = vec![0; length as usize];
    unsafe {
        GetTokenInformation(
            th,
            TokenUser,
            token_buffer.as_mut_ptr() as _,
            token_buffer.len() as u32,
            &mut length,
        )
    }
    .ok()?;
    token_user = token_buffer.as_mut_ptr() as *mut TOKEN_USER;

    // Extract the pointer to the user SID.
    let user_sid = unsafe { *token_user }.User.Sid;

    // use the Application event log
    let event_log_handle = unsafe { OpenEventLogA(PCSTR::default(), "Application") }?;

    let message = CString::new("Hello in the event log").unwrap();
    let message = PSTR(message.as_ptr() as _);

    let category = 5; // "Shell"
    unsafe {
        ReportEventA(
            EventSourceHandle(event_log_handle.0),
            EVENTLOG_WARNING_TYPE,
            category,
            1,
            user_sid,
            0,
            &[message],
            ptr::null_mut(),
        )
    }
    .ok()?;

    Ok(())
}

#[cfg(not(windows))]
fn main() {
    println!("Not implemented");
}
