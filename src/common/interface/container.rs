use iced::Border;
use iced::Theme;
use iced::border::Radius;
use iced::widget::button::Status as ButtonStatus;
use iced::widget::button::Style as ButtonStyle;
use iced::widget::container;
use iced::widget::container::Style as ContainerStyle;

use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::constants::BORDER_RADIUS;
use crate::common::interface::constants::BORDER_WIDTH;
use crate::common::interface::shadow::SHADOW_GRAY;

pub fn shadow_container() -> impl Fn(&Theme) -> ContainerStyle {
    |_theme: &Theme| ContainerStyle {
        shadow: SHADOW_GRAY,
        ..Default::default()
    }
}

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
            shadow: SHADOW_GRAY,
            ..Default::default()
        }
    }
}

pub(crate) fn sidebar_container() -> impl Fn(&Theme) -> ContainerStyle {
    |_theme| ContainerStyle {
        border: Border {
            color: OFF_WHITE,
            width: BORDER_WIDTH,
            radius: Radius::new(BORDER_RADIUS),
        },
        ..Default::default()
    }
}

pub fn header_container() -> impl Fn(&iced::Theme) -> container::Style {
    |_theme| container::Style {
        border: Border {
            color: OFF_WHITE,
            width: BORDER_WIDTH,
            radius: Radius::new(BORDER_RADIUS),
        },
        ..Default::default()
    }
}
