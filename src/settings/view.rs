use iced::Element;
use iced::Length;
use iced::widget::Container;
use iced::widget::column;
use iced::widget::container;
use iced::widget::row;
use iced::widget::text;

use crate::BonsaiMessage;
use crate::node::interface::common::TITLE_PADDING;
use crate::node::interface::common::title_container;
use crate::settings::bonsai_settings::BonsaiSettingsMessage;

pub(crate) fn view_settings<'a>() -> Element<'a, BonsaiSettingsMessage> {
    let title: Container<'_, BonsaiSettingsMessage> = container(text("SETTINGS").size(25))
        .style(title_container())
        .padding(TITLE_PADDING);

    let left = column![title].spacing(20).width(Length::FillPortion(1));

    let right = column![].spacing(20).width(Length::FillPortion(1));

    row![left, right].spacing(20).into()
}
