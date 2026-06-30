use iced::Element;

fn update(_state: &mut (), _message: ()) {}

fn view(_state: &()) -> Element<'_, ()> {
    "Goodbye, World!".into()
}

fn main() -> iced::Result {
    iced::application(|| (), update, view)
        .title("Hello, World!")
        .run()
}
