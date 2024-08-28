use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

pub fn main() -> iced::Result {
    Lyuma::run(Settings::default())
}

struct Lyuma;

impl Application for Lyuma {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Lyuma, Command<Self::Message>) {
        (Lyuma, Command::none())
    }

    fn title(&self) -> String {
        String::from("Lyuma")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        "Hello, world!".into()
    }
}