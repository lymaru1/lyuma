use iced::widget::{button, column, image, row, text, container};
use iced::{Alignment, Element, Length};

// 메뉴에 표시할 화면들을 정의
#[derive(Debug, Clone)]
pub enum Screen {
    Home,
    Settings,
    About,
    Profile,
}

pub struct MenuItem {
    pub label: &'static str,
    pub screen: Screen,
    pub icon_path: &'static str,
}

pub fn get_menu_items() -> Vec<MenuItem> {
    vec![
        MenuItem { label: "Home", screen: Screen::Home, icon_path: "images/menu_setting.png" },
        MenuItem { label: "Settings", screen: Screen::Settings, icon_path: "images/menu_setting.png"},
        MenuItem { label: "About", screen: Screen::About, icon_path: "images/menu_setting.png" },
        MenuItem { label: "Profile", screen: Screen::Profile, icon_path: "images/menu_setting.png" },
    ]
}

pub fn view(menu_items: Vec<MenuItem>) -> Element<'static, crate::Message> {
    let buttons: Vec<Element<_>> = menu_items.into_iter().map(|item| {
        let icon = image(item.icon_path).width(Length::Fixed(25.0)).height(Length::Fixed(25.0));
        let label = text(item.label);

        let content = row![
            icon,
            container(label).width(Length::Fill).center_x()
        ]
        .spacing(5)
        .align_items(Alignment::Center);

        button(content)
            .width(Length::Fill)
            .on_press(crate::Message::MenuClicked(item.screen))
            .into()
    }).collect();

    column(buttons).spacing(10).into()
}