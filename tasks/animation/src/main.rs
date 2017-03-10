extern crate gtk;

use gtk::prelude::*;
use std::ops::Not;
use std::sync::{Arc, RwLock};


fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let button = gtk::Button::new_with_label("Hello World! ");

    window.add(&button);

    let lock = Arc::new(RwLock::new(false));

    let lock_button = lock.clone();
    button.connect_clicked(move |_| {
        let mut reverse = lock_button.write().unwrap();
        *reverse = reverse.not();
    });

    let lock_thread = lock.clone();
    gtk::timeout_add(100, move || {
        let reverse = lock_thread.read().unwrap();
        let mut text = button.get_label().unwrap();
        let len = &text.len();

        if *reverse {
            let begin = &text.split_off(1);
            text.insert_str(0, begin);
        } else {
            let end = &text.split_off(len - 1);
            text.insert_str(0, end);
        }

        button.set_label(&text);

        gtk::Continue(true)
    });


    window.show_all();
    gtk::main();
}
