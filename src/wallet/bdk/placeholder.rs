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
        column![
            text("ONCHAIN WALLET [TODO]").size(24),
            text("Powered by BDK").size(12)
        ]
        .into()
    }
}
