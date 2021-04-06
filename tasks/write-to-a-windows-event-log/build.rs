fn main() {
    #[cfg(windows)]
    {
        windows::build!(Windows::Win32::SystemServices::{GetCurrentProcess, ReportEventA, OpenEventLogA, ReportEvent_wType, HANDLE, PSTR},
    Windows::Win32::Security::{OpenProcessToken, GetTokenInformation, TOKEN_ACCESS_MASK, TOKEN_INFORMATION_CLASS, TOKEN_USER, PSID});
    }
}
