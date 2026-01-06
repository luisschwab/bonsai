use iced::Border;
use iced::Theme;
use iced::border::Radius;
use iced::widget::button::Status as ButtonStatus;
use iced::widget::button::Style as ButtonStyle;
use iced::widget::container::Style as ContainerStyle;

use crate::common::interface::color::BLACK;
use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::color::ORANGE;
use crate::common::interface::color::RED;
use crate::common::interface::container::common::BORDER_RADIUS;
use crate::common::interface::container::common::BORDER_WIDTH;
use crate::common::interface::container::common::SHADOW;

/// Container for displaying a table with peer info.
pub(crate) fn peer_info_table_container() -> impl Fn(&Theme) -> ContainerStyle {
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

pub fn disconnect_button() -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    |_theme, status| ButtonStyle {
        background: Some(iced::Background::Color(
            if status == ButtonStatus::Hovered {
                ORANGE.scale_alpha(0.8)
            } else {
                ORANGE
            },
        )),
        text_color: BLACK,
        border: iced::Border {
            color: iced::Color::BLACK,
            width: 1.0,
            radius: Radius::new(0),
        },
        ..Default::default()
    }
}

pub fn ban_button() -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    |_theme, status| ButtonStyle {
        background: Some(iced::Background::Color(
            if status == ButtonStatus::Hovered {
                RED.scale_alpha(0.8)
            } else {
                RED
            },
        )),
        text_color: BLACK,
        border: iced::Border {
            color: iced::Color::BLACK,
            width: 1.0,
            radius: Radius::new(0),
        },
        ..Default::default()
    }
}
