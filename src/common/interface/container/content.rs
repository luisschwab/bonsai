use iced::border::Radius;
use iced::widget::container;
use iced::{Border, Theme};

use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::container::common::{BORDER_RADIUS, BORDER_WIDTH, SHADOW};

pub(crate) const CONTENT_PADDING: f32 = 10.0;
pub(crate) const CONTENT_SPACING: f32 = 5.0;

pub(crate) fn content_container() -> impl Fn(&Theme) -> container::Style {
    |_theme| container::Style {
        border: Border {
            color: OFF_WHITE,
            width: BORDER_WIDTH,
            radius: Radius::new(BORDER_RADIUS),
        },
        shadow: SHADOW,
        ..Default::default()
    }
}
