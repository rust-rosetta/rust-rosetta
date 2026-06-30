use std::time::Duration;

use iced::{
    Element,
    widget::{button, text},
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

fn update(state: &mut Animation, message: Message) {
    match message {
        Message::Tick => {
            if state.reverse {
                let begin = state.text.split_off(1);
                state.text.insert_str(0, &begin);
            } else {
                let end = state.text.split_off(state.text.len() - 1);
                state.text.insert_str(0, &end);
            }
        }
        Message::Reverse => state.reverse = !state.reverse,
    }
}

fn view(state: &Animation) -> Element<'_, Message> {
    button(text(&state.text)).on_press(Message::Reverse).into()
}

fn main() -> iced::Result {
    iced::application(Animation::default, update, view)
        .title("Animation")
        .subscription(|_| iced::time::every(Duration::from_millis(100)).map(|_| Message::Tick))
        .run()
}
