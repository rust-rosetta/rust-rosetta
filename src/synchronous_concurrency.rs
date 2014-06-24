// Reader unit reads lines from input.txt, send lines one at a time to printer
// unit, which then prints lines, keeping track of lines printed. At the end of
// the file, the reader unit requests number of lines printed from the printer
// unit, and then prints them.
//
// http://rosettacode.org/wiki/Synchronous_concurrency

use std::io::File;
use std::io::BufferedReader;
use std::comm::{DuplexStream, duplex};

static FILENAME: &'static str = "resources/input.txt";

enum Message {
    Line(String),
    End
}

fn printer(chan: DuplexStream<int, Message>) {
    let mut count = 0;
    loop {
        match chan.recv() {
            Line(line) => {
                print!("{}", line);
                count += 1;
            }
            End => {break;}
        }
    }
    chan.send(count);
}

fn reader(chan: DuplexStream<Message, int>) {
    let mut file = BufferedReader::new(File::open(&Path::new(FILENAME)));
    for line in file.lines() {
        chan.send(Line(line.unwrap()));
    }
    chan.send(End);
    println!("Total Lines: {}", chan.recv());
}

// not_tested
fn main() {
    let (to_reader, to_printer) = duplex();
    spawn(proc() printer(to_reader));
    spawn(proc() reader(to_printer));
}
