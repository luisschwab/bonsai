use iced::widget::{column, text};
use iced::{Element, Font};

#[derive(Default)]
pub struct Node {
}

#[derive(Debug, Clone)]
pub enum Message {
}

impl Node {
    pub fn update(&mut self, message: Message) {
        match message {
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            text("Node Tab")
                .size(24)
                .font(Font::MONOSPACE),
            text("Node Management stats go here")
                .font(Font::MONOSPACE),
        ]
        .spacing(20)
        .into()
    }
}
