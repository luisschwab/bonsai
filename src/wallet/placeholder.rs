use iced::Element;
use iced::widget::{column, text};

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
            text("Wallet Tab").size(24),
            text("BDK-based wallet goes here"),
        ]
        .spacing(20)
        .into()
    }
}
