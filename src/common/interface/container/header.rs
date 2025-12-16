use iced::Border;
use iced::widget::container;

use crate::common::interface::color::OFF_WHITE;

pub(crate) const HEADER_HEIGHT: f32 = 60.0;
pub(crate) const HEADER_PADDING: f32 = 20.0;

pub fn header_container() -> impl Fn(&iced::Theme) -> container::Style {
    |_theme| container::Style {
        border: Border {
            color: OFF_WHITE,
            width: 2.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}
