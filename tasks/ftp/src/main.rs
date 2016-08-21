extern crate ftp;

use ftp::FtpStream;
use ftp::types::FileType;
use std::fs::File;
use std::io::Write;

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
    ftp.retr("README", |stream| {
        let mut file = File::create("README").unwrap();
        let mut buf = [0; 2048];
        loop {
            match stream.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => file.write_all(&buf[0..n]).unwrap(),
                Err(err) => panic!(err)
            };
        }
        Ok(())
    }).unwrap();
}

#[cfg(test)]
mod test {
    use ftp::FtpStream;

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
}
