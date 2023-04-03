use iced::{
    executor,
    widget::{button, text},
    Application, Command, Element, Settings, Subscription, Theme,
};

struct Animation {
    text: String,
    reverse: bool,
}

impl Default for Animation {
    fn default() -> Self {
        Animation {
            text: String::from("Hello, world! "),
            reverse: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
    Reverse,
}

impl Application for Animation {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Animation::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Animation")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                if self.reverse {
                    let begin = self.text.split_off(1);
                    self.text.insert_str(0, &begin);
                } else {
                    let end = self.text.split_off(self.text.len() - 1);
                    self.text.insert_str(0, &end);
                }
            }
            Message::Reverse => self.reverse = !self.reverse,
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(100)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        button(text(&self.text)).on_press(Message::Reverse).into()
    }
}

fn main() -> iced::Result {
    Animation::run(Settings::default())
}
