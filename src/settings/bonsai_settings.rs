use iced::Element;
use iced::Task;

use crate::BonsaiMessage;

#[derive(Default, Debug, Clone)]
pub(crate) enum BonsaiSettingsMessage {
    #[default]
    Placeholder,
}

#[derive(Default)]
pub(crate) struct BonsaiSettings {}

impl BonsaiSettings {
    pub(crate) fn view(&self) -> Element<'_, BonsaiSettingsMessage> {
        use crate::settings::view::view_settings;
        view_settings()
    }

    pub(crate) fn update(&mut self, message: BonsaiSettingsMessage) -> Task<BonsaiSettingsMessage> {
        Task::none()
    }
}
