#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(windows)]
mod cfg_windows;

#[cfg(windows)]
fn main() {
    cfg_windows::main();
}

#[cfg(not(windows))]
fn main() {
    println!("So far implemented only for Windows");
}
