extern crate ftp;

use std::fs::File;
use std::io;

use ftp::types::FileType;
use ftp::FtpStream;

static FILENAME: &str = "README";

fn main() {
    // connect to the server
    let mut ftp = FtpStream::connect("kernel.org:21").unwrap();
    ftp.login("anonymous", "").unwrap();

    // change working directory
    ftp.cwd("/pub/linux/kernel").unwrap();

    // list files in the current directory
    let list = ftp.list(None).unwrap();
    println!("{}", list.join(""));

    // download a file a write it on the disk
    ftp.transfer_type(FileType::Binary).unwrap();
    let mut contents = ftp.simple_retr(FILENAME).unwrap();
    let mut file = File::create(FILENAME).unwrap();
    io::copy(&mut contents, &mut file).unwrap();
}
