#![allow(unused)]

use iced::Element;
use iced::widget::column;
use iced::widget::text;

#[derive(Default)]
pub struct Phoenixd {}

#[derive(Debug, Clone)]
pub enum PhoenixdMessage {}

impl Phoenixd {
    pub fn update(&mut self, message: PhoenixdMessage) {
        match message {}
    }

    pub fn view(&self) -> Element<'_, PhoenixdMessage> {
        column![
            text("LIGHTNING WALLET [TODO]").size(24),
            text("Powered by phoenixd").size(12)
        ]
        .into()
    }
}
