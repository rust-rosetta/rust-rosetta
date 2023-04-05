use iced::{Element, Sandbox, Settings};

struct Goodbye;

impl Sandbox for Goodbye {
    type Message = ();

    fn new() -> Goodbye {
        Goodbye
    }

    fn title(&self) -> String {
        String::from("Hello, World!")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        "Goodbye, World!".into()
    }
}

fn main() -> iced::Result {
    Goodbye::run(Settings::default())
}
