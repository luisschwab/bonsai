use iced::widget::container::Style;
use iced::{Border, Theme};

use crate::common::interface::color::OFF_WHITE;

pub(crate) const SIDEBAR_WIDTH: f32 = 150.0;
pub(crate) const SIDEBAR_PADDING: f32 = 10.0;
pub(crate) const SIDEBAR_BUTTON_SPACING: u32 = 5;

pub(crate) fn sidebar_container() -> impl Fn(&Theme) -> Style {
    |_theme| Style {
        border: Border {
            color: OFF_WHITE,
            width: 2.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub(crate) fn sidebar_element() -> impl Fn(&Theme) -> Style {
    |_theme| Style {
        border: Border {
            color: OFF_WHITE,
            width: 2.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}
