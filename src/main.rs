use iced::executor;
use iced::widget::{container, row, text};
use iced::{Application, Command, Element, Length, Settings, Theme};

mod menu;
use menu::Screen;
mod logger;
use logger::{Loggable, log_info};

pub fn main() -> iced::Result {
    logger::init_logger();
    Lyuma::run(Settings::default())
}

struct Lyuma {
    screen: Screen,
}

#[derive(Debug, Clone)]
enum Message {
    MenuClicked(Screen),
}

impl Application for Lyuma {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Lyuma, Command<Self::Message>) {
        (Lyuma { screen: Screen::Home }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Lyuma")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::MenuClicked(screen) => {
                self.screen = screen;
                log_info(&format!("Menu clicked: {:?}", self.screen));
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let menu_items = menu::get_menu_items();
        let menu = menu::view(menu_items);

        let content: Element<_> = match self.screen {
            Screen::Home => text("This is the Home screen").into(),
            Screen::Settings => text("This is the Settings screen").into(),
            Screen::About => text("This is the About screen").into(),
            Screen::Profile => text("This is the Profile screen").into(),
        };

        row![
            container(menu).width(Length::FillPortion(2)),
            container(content).width(Length::FillPortion(8)),
        ]
        .into()
    }
}