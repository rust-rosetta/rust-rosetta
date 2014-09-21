// http://rosettacode.org/wiki/Synchronous_concurrency
#![allow(dead_code)] // not_tested
//
// Reader unit reads lines from input.txt, send lines one at a time to printer
// unit, which then prints lines, keeping track of lines printed. At the end of
// the file, the reader unit requests number of lines printed from the printer
// unit, and then prints them.

use std::io::File;
use std::io::BufferedReader;
use std::comm::{channel, Sender, Receiver};

static FILENAME: &'static str = "resources/input.txt";

enum Message {
    Line(String),
    End
}

fn printer(i_snd: Sender<int>, msg_rcv: Receiver<Message>) {
    let mut count = 0;
    loop {
        match msg_rcv.recv() {
            Line(line) => {
                print!("{}", line);
                count += 1;
            }
            End => {break;}
        }
    }
    i_snd.send(count);
}

fn reader(msg_snd: Sender<Message>, i_rcv: Receiver<int>) {
    let mut file = BufferedReader::new(File::open(&Path::new(FILENAME)));
    for line in file.lines() {
        msg_snd.send(Line(line.unwrap()));
    }
    msg_snd.send(End);
    println!("Total Lines: {}", i_rcv.recv());
}

fn main() {
    let (msg_snd, msg_rcv) = channel();
    let (i_snd, i_rcv) = channel();

    spawn(proc() printer(i_snd, msg_rcv));
    spawn(proc() reader(msg_snd, i_rcv));
}
