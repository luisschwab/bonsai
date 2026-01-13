use iced::Border;
use iced::Theme;
use iced::border::Radius;
use iced::widget::button::Status as ButtonStatus;
use iced::widget::button::Style as ButtonStyle;
use iced::widget::container::Style as ContainerStyle;

use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::constants::BORDER_RADIUS;
use crate::common::interface::constants::BORDER_WIDTH;
use crate::common::interface::shadow::SHADOW_GRAY;

/// Container for displaying a table with peer info.
pub(crate) fn peer_info_table_container() -> impl Fn(&Theme) -> ContainerStyle {
    |_theme| ContainerStyle {
        border: Border {
            color: OFF_WHITE,
            width: BORDER_WIDTH,
            radius: Radius::new(BORDER_RADIUS),
        },
        shadow: SHADOW_GRAY,
        ..Default::default()
    }
}

pub fn network_button() -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    |_theme, status| ButtonStyle {
        background: None,
        text_color: if status == ButtonStatus::Hovered {
            OFF_WHITE.scale_alpha(0.8)
        } else {
            OFF_WHITE
        },
        border: Border {
            color: if status == ButtonStatus::Hovered {
                OFF_WHITE.scale_alpha(0.8)
            } else {
                OFF_WHITE
            },
            width: 1.0,
            radius: Radius::new(0),
        },
        ..Default::default()
    }
}
