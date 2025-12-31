use iced::Element;
use iced::widget::column;
use iced::widget::text;

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
            text("WALLET [TODO]").size(24),
            text("Powered by BDK").size(16)
        ]
        .into()
    }
}
