use iced::border::Radius;
use iced::widget::button::Status as ButtonStatus;
use iced::widget::button::Style as ButtonStyle;
use iced::widget::container::Style as ContainerStyle;
use iced::{Border, Theme};

use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::container::common::SHADOW;
use crate::common::interface::container::common::{BORDER_RADIUS, BORDER_WIDTH};

pub(crate) const CONTENT_PADDING: f32 = 10.0;
pub(crate) const CONTENT_SPACING: f32 = 8.0;

pub(crate) fn content_container() -> impl Fn(&Theme) -> ContainerStyle {
    |_theme| ContainerStyle {
        border: Border {
            color: OFF_WHITE,
            width: BORDER_WIDTH,
            radius: Radius::new(BORDER_RADIUS),
        },
        ..Default::default()
    }
}

pub(crate) fn button_container() -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    |_theme, status| {
        let text_color = match status {
            ButtonStatus::Hovered => OFF_WHITE.scale_alpha(0.7),
            ButtonStatus::Pressed => OFF_WHITE.scale_alpha(0.5),
            _ => OFF_WHITE,
        };

        ButtonStyle {
            border: Border {
                color: OFF_WHITE,
                width: BORDER_WIDTH,
                radius: Radius::new(BORDER_RADIUS),
            },
            text_color,
            shadow: SHADOW,
            ..Default::default()
        }
    }
}
