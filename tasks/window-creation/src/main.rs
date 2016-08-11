// http://rosettacode.org/wiki/Window_creation

#[cfg(feature = "gtk")]
mod graphical {
    extern crate gtk;

    use self::gtk::traits::*;
    use self::gtk::{Inhibit, Window, WindowType};

    pub fn create_window() {
        gtk::init().expect("Failed to initialize GTK");

        let window = Window::new(WindowType::Toplevel);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
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
