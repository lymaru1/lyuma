use iced::widget::{button, Column, Row, Text};
use iced::{Element, Command};
use iced::widget::text::Shaping;
use btleplug::api::{Central, Manager as _, Peripheral as _};
use btleplug::platform::Manager;
use futures::stream::StreamExt;

#[derive(Debug, Clone)]
pub enum Message {
    Search,
    DevicesFound(Vec<String>),
}

pub struct Home {
    search_button: iced::widget::button::State,
    devices: Vec<String>,
    searching: bool,
}

impl Home {
    pub fn new() -> Self {
        Self {
            search_button: iced::widget::button::State::new(),
            devices: Vec::new(),
            searching: false,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Search => {
                self.searching = true;
                Command::perform(scan_ble_devices(), Message::DevicesFound)
            }
            Message::DevicesFound(devices) => {
                self.devices = devices;
                self.searching = false;
                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let search_button = button(
            if self.searching {
                Text::new("검색 중...").shaping(Shaping::Advanced)
            } else {
                Text::new("블루투스 검색").shaping(Shaping::Advanced)
            }
        ).on_press(Message::Search);

        let device_list = self.devices.iter().fold(Column::new(), |col, name| {
            col.push(Text::new(name))
        });

        Column::new()
            .push(Row::new().push(search_button))
            .push(device_list)
            .into()
    }
}

async fn scan_ble_devices() -> Vec<String> {
    let manager = match Manager::new().await.ok() {
        Some(m) => m,
        None => return vec![],
    };
    let adapters = match manager.adapters().await.ok() {
        Some(a) => a,
        None => return vec![],
    };
    let central = match adapters.into_iter().next() {
        Some(c) => c,
        None => return vec![],
    };
    let mut devices = Vec::new();

    let mut events = match central.events().await.ok() {
        Some(e) => e,
        None => return vec![],
    };
    if central.start_scan(Default::default()).await.ok().is_none() {
        return vec![];
    }

    let timeout = std::time::Duration::from_secs(3);
    let start = std::time::Instant::now();
    while let Some(event) = events.next().await {
        if start.elapsed() > timeout {
            break;
        }
        if let btleplug::api::CentralEvent::DeviceDiscovered(id) = event {
            if let Some(peripheral) = central.peripheral(&id).await.ok() {
                if let Ok(Some(properties)) = peripheral.properties().await {
                    if let Some(name) = properties.local_name {
                        devices.push(name);
                    }
                }
            }
        }
    }
    devices
}