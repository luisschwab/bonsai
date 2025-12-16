#![allow(unused)]

use iced::Element;
use iced::widget::{column, text};

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
            text("Phoenixd LN Wallet").size(24),
            text("TODO, coming soon™"),
        ]
        .spacing(20)
        .into()
    }
}
