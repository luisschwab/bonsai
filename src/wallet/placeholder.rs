use iced::widget::{column, text};
use iced::{Element, Font};

#[derive(Default)]
pub struct Wallet {}

#[derive(Debug, Clone)]
pub enum WalletMessage {}

impl Wallet {
    pub fn update(&mut self, message: WalletMessage) {
        match message {}
    }

    pub fn view(&self) -> Element<'_, WalletMessage> {
        column![
            text("Wallet Tab").size(24).font(Font::MONOSPACE),
            text("BDK-based wallet goes here").font(Font::MONOSPACE),
        ]
        .spacing(20)
        .into()
    }
}
