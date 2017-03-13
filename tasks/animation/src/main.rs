#[cfg(feature = "gtk")]
mod graphical {
    extern crate gtk;

    use self::gtk::traits::*;
    use self::gtk::{Inhibit, Window, WindowType};
    use std::ops::Not;
    use std::sync::{Arc, RwLock};

    pub fn create_window() {
        gtk::init().expect("Failed to initialize GTK");

        let window = Window::new(WindowType::Toplevel);
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
}


#[cfg(feature = "gtk")]
fn main() {
    graphical::create_window();
}

#[cfg(not(feature = "gtk"))]
fn main() {}
