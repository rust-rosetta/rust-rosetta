//! Reader unit reads lines from input.txt, send lines one at a time to printer unit, which then
//! prints lines, keeping track of lines printed. At the end of the file, the reader unit requests
//! number of lines printed from the printer unit, and then prints them.

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

const FILENAME: &'static str = "resources/input.txt";

enum Message {
    Line(String),
    End,
}

fn printer(i_snd: Sender<i32>, msg_rcv: Receiver<Message>) {
    let mut count = 0;
    while let Message::Line(line) = msg_rcv.recv().unwrap() {
        println!("{}", line);
        count += 1;

    }
    i_snd.send(count).unwrap();
}

fn reader(msg_snd: Sender<Message>, i_rcv: Receiver<i32>) {
    let file = BufReader::new(File::open(FILENAME).unwrap());
    for line in file.lines() {
        msg_snd.send(Message::Line(line.unwrap())).unwrap();
    }
    msg_snd.send(Message::End).unwrap();
    println!("Total Lines: {}", i_rcv.recv().unwrap());
}

fn main() {
    let (msg_snd, msg_rcv) = channel();
    let (i_snd, i_rcv) = channel();

    thread::spawn(move || printer(i_snd, msg_rcv));
    thread::spawn(move || reader(msg_snd, i_rcv));
}
