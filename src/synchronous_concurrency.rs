// http://rosettacode.org/wiki/Synchronous_concurrency
//
// Reader unit reads lines from input.txt, send lines one at a time to printer
// unit, which then prints lines, keeping track of lines printed. At the end of
// the file, the reader unit requests number of lines printed from the printer
// unit, and then prints them.
#![feature(old_io)]
#![feature(old_path)]

use std::old_io::File;
use std::old_io::BufferedReader;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::spawn;

const FILENAME: &'static str = "resources/input.txt";

enum Message {
    Line(String),
    End
}

fn printer(i_snd: Sender<i32>, msg_rcv: Receiver<Message>) {
    let mut count = 0;
    loop {
        match msg_rcv.recv().unwrap() {
            Message::Line(line) => {
                print!("{}", line);
                count += 1;
            }
            Message::End => {break;}
        }
    }
    i_snd.send(count).unwrap();
}

fn reader(msg_snd: Sender<Message>, i_rcv: Receiver<i32>) {
    let mut file = BufferedReader::new(File::open(&Path::new(FILENAME)));
    for line in file.lines() {
        msg_snd.send(Message::Line(line.unwrap())).unwrap();
    }
    msg_snd.send(Message::End).unwrap();
    println!("Total Lines: {:?}", i_rcv.recv());
}

fn main() {
    let (msg_snd, msg_rcv) = channel();
    let (i_snd, i_rcv) = channel();

    spawn(move || printer(i_snd, msg_rcv));
    spawn(move || reader(msg_snd, i_rcv));
}
