#![feature(conservative_impl_trait)]

extern crate ftp;

use ftp::FtpStream;
use ftp::types::{Result,FileType};
use std::fs::{File};
use std::io::{Read,Write};

fn write_file(filename: &str) -> impl Fn(&mut Read) -> Result<()> {
    let filename = filename.to_string();
    move |stream| {
        let mut file = File::create(&filename).unwrap();
        let mut buf = [0; 2048];
        loop {
            match stream.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => file.write_all(&buf[0..n]).unwrap(),
                Err(err) => panic!(err)
            };
        }
        Ok(())
    }
}

fn main() {
    // connect to the server
    let mut ftp = FtpStream::connect("kernel.org:21").unwrap();
    ftp.login("anonymous", "").unwrap();

    // change working directory
    ftp.cwd("/pub/linux/kernel").unwrap();

    // list files in the current directory
    match ftp.list(None) {
        Ok(output) => println!("{}", output.join("")),
        Err(err) => panic!(err)
    }

    // download a file a write it on the disk
    ftp.transfer_type(FileType::Binary).unwrap();
    ftp.retr("README", write_file("README")).unwrap();
}

#[cfg(test)]
mod test {
    use ftp::FtpStream;
    use std::fs;
    use ftp::types::{FileType};
    use super::{write_file};

    fn connect() -> FtpStream {
        let mut ftp = FtpStream::connect("kernel.org:21").unwrap();
        ftp.login("anonymous", "").unwrap();
        return ftp;
    }

    #[ignore]
    #[test]
    fn test_cwd() {
        let mut ftp = connect();
        // make sure the current directory is /
        assert_eq!(ftp.pwd().unwrap(), "/");
        ftp.cwd("/pub/linux/kernel").unwrap();
        assert_eq!(ftp.pwd().unwrap(), "/pub/linux/kernel");
    }

    #[ignore]
    #[test]
    fn test_list_dir() {
        let mut ftp = connect();
        assert_eq!(
            ftp.list(Some("/")).unwrap().join(""),
            "drwxr-xr-x    9 ftp      ftp          4096 Dec 01  2011 pub");
    }

    #[ignore]
    #[test]
    fn test_download_file() {
        let filename = ".test_download_file";
        let mut ftp = connect();
        ftp.cwd("/pub/linux/kernel").unwrap();
        // make sure the file does not already exist
        match fs::metadata(filename) {
            Ok(_) => fs::remove_file(filename).unwrap(),
            Err(_) => {}
        }
        ftp.transfer_type(FileType::Binary).unwrap();
        ftp.retr("README", write_file(filename)).unwrap();
        match fs::metadata(filename) {
            Ok(metadata) => {
                assert_eq!(metadata.is_file(), true);
                assert_eq!(metadata.len(), 12056);
                fs::remove_file(filename).unwrap();
            }
            Err(_) => panic!("file not downloaded")
        }
    }
}
