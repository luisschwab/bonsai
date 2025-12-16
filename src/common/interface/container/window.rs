use iced::widget::container;
use iced::{Border, Theme};

use crate::common::interface::color::OFF_WHITE;

pub(crate) const WINDOW_PADDING: f32 = 10.0;

pub(crate) fn window_container() -> impl Fn(&Theme) -> container::Style {
    |_theme| container::Style {
        border: Border {
            color: OFF_WHITE,
            width: 2.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}
