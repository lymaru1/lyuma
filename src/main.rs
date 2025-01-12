use iced::executor;
use iced::widget::{button, column, container, row, text};
use iced::{Application, Command, Element, Length, Settings, Theme};

pub fn main() -> iced::Result {
    Lyuma::run(Settings::default())
}

struct Lyuma {
    screen: Screen,
}

#[derive(Debug, Clone)]
enum Message {
    MenuClicked(Screen),
}
#[derive(Debug, Clone)]
enum Screen {
    Home,
    Settings,
    About,
}
impl Application for Lyuma {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Lyuma, Command<Self::Message>) {
        (Lyuma { screen: Screen::Home}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Lyuma")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::MenuClicked(screen) => {
                self.screen = screen;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let menu = column![
            button("Home").on_press(Message::MenuClicked(Screen::Home)),
            button("Settings").on_press(Message::MenuClicked(Screen::Settings)),
            button("About").on_press(Message::MenuClicked(Screen::About)),
        ]
            .spacing(10);

        let content: Element<_> = match self.screen {
            Screen::Home => text("This is the Home screen").into(),
            Screen::Settings => text("This is the Settings screen").into(),
            Screen::About => text("This is the About screen").into(),
        };

        row![
            container(menu).width(Length::FillPortion(2)),
            container(content).width(Length::FillPortion(8)),
        ]
        .into()
    }
}