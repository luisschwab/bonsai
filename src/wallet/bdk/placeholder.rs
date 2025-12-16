use iced::Element;
use iced::widget::{column, text};

#[derive(Default)]
pub struct BDKWallet {}

#[derive(Debug, Clone)]
pub enum BDKWalletMessage {}

impl BDKWallet {
    pub fn update(&mut self, message: BDKWalletMessage) {
        match message {}
    }

    pub fn view(&self) -> Element<'_, BDKWalletMessage> {
        column![text("BDK Wallet").size(24), text("TODO, coming soon™"),]
            .spacing(20)
            .into()
    }
}
