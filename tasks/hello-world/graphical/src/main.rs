#[cfg(feature = "gtk")]
mod graphical {
    extern crate gtk;

    use self::gtk::traits::*;
    use self::gtk::{Inhibit, Window, WindowType, WindowPosition};

    pub fn hello_world() {
        gtk::init().unwrap();
        let window = Window::new(WindowType::Toplevel);

        window.set_title("Hello World!");
        window.set_border_width(10);
        window.set_position(WindowPosition::Center);
        window.set_default_size(350, 70);

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
    graphical::hello_world();
}

#[cfg(not(feature = "gtk"))]
fn main() {}
