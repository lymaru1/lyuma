use iced::widget::{button, column};
use iced::Element;

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
}

pub fn get_menu_items() -> Vec<MenuItem> {
    vec![
        MenuItem { label: "Home", screen: Screen::Home },
        MenuItem { label: "Settings", screen: Screen::Settings },
        MenuItem { label: "About", screen: Screen::About },
        MenuItem { label: "Profile", screen: Screen::Profile },
    ]
}

pub fn view(menu_items: Vec<MenuItem>) -> Element<'static, crate::Message> {
    let buttons: Vec<Element<_>> = menu_items.into_iter().map(|item| {
        button(item.label).on_press(crate::Message::MenuClicked(item.screen)).into()
    }).collect();

    column(buttons).spacing(10).into()
}