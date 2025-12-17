use iced::border::Radius;
use iced::widget::container::Style;
use iced::widget::container::Style as ContainerStyle;
use iced::{Border, Padding, Theme};

use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::container::common::{BORDER_RADIUS, BORDER_WIDTH, SHADOW};

pub const TITLE_PADDING: Padding = Padding {
    top: 5.0,
    right: 10.0,
    bottom: 5.0,
    left: 10.0,
};

pub(crate) fn title_container() -> impl Fn(&Theme) -> ContainerStyle {
    |_theme| ContainerStyle {
        border: Border {
            color: OFF_WHITE,
            width: BORDER_WIDTH,
            radius: Radius::new(BORDER_RADIUS),
        },
        shadow: SHADOW,
        ..Default::default()
    }
}

pub(crate) fn table_cell() -> impl Fn(&Theme) -> Style {
    move |_theme| {
        let border = Border {
            color: OFF_WHITE,
            width: BORDER_WIDTH,
            radius: Radius::new(BORDER_RADIUS),
        };

        Style {
            border,
            ..Default::default()
        }
    }
}
