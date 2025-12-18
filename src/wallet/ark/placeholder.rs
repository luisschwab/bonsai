use iced::Element;
use iced::widget::{column, text};

#[derive(Default)]
pub struct ArkWallet {}

#[derive(Debug, Clone)]
pub enum ArkWalletMessage {}

impl ArkWallet {
    pub fn update(&mut self, message: ArkWalletMessage) {
        match message {}
    }

    pub fn view(&self) -> Element<'_, ArkWalletMessage> {
        column![
            text("ARK WALLET [TODO]").size(24),
            text("Powered by bark").size(12)
        ]
        .into()
    }
}
