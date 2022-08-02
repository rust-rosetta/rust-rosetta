use iced::{executor, Application, Command, Element, Settings, Text};

struct Goodbye;

impl Application for Goodbye {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Goodbye, Command<Self::Message>) {
        (Goodbye, Command::none())
    }

    fn title(&self) -> String {
        String::from("Hello, World!")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Goodbye, World!").into()
    }
}

fn main() -> iced::Result {
    Goodbye::run(Settings::default())
}
