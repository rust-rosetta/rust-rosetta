use std::error::Error;
use std::io;

use ftp::types::{FileType, FtpError};
use ftp::FtpStream;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // connect to the server
    let mut ftp = FtpStream::connect("mirrors.sonic.net:21")?;
    ftp.login("anonymous", "")?;

    // change working directory
    ftp.cwd("/pub/OpenBSD/doc")?;

    // list files in the current directory
    for line in ftp.list(None)? {
        println!("{}", line);
    }

    // download a file a write it on the disk
    let file_name = "README";
    ftp.transfer_type(FileType::Binary)?;
    ftp.retr(file_name, move |data| {
        let mut local_file = File::create(file_name).map_err(FtpError::ConnectionError)?;
        io::copy(data, &mut local_file).map_err(FtpError::ConnectionError)
    })?;

    Ok(())
}
