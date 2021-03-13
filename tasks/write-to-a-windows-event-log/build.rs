fn main() {
    #[cfg(target_os = "windows")]
    {
        windows::build!(windows::win32::system_services::{GetCurrentProcess, ReportEventA, OpenEventLogA, EventLogHandle, HANDLE, BOOL, PSTR},
    windows::win32::security::{OpenProcessToken, GetTokenInformation, TOKEN_INFORMATION_CLASS});
    }
}
