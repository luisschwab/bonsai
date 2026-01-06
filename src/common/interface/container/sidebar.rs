use iced::Border;
use iced::Color;
use iced::Shadow;
use iced::Theme;
use iced::border::Radius;
use iced::widget::button::Status as ButtonStatus;
use iced::widget::button::Status::Active;
use iced::widget::button::Status::Disabled;
use iced::widget::button::Status::Hovered;
use iced::widget::button::Status::Pressed;
use iced::widget::button::Style as ButtonStyle;
use iced::widget::container::Style as ContainerStyle;

use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::container::common::BORDER_RADIUS;
use crate::common::interface::container::common::BORDER_WIDTH;

pub(crate) const SIDEBAR_WIDTH: f32 = 200.0;
pub(crate) const SIDEBAR_PADDING: f32 = 10.0;
pub(crate) const SIDEBAR_BUTTON_HEIGHT: f32 = 45.0;
pub(crate) const SIDEBAR_BUTTON_SPACING: f32 = 10.0;

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

pub fn sidebar_button(
    is_active: bool,
    hover_color: Color,
) -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    move |_theme, button_status| {
        let border = Border {
            color: OFF_WHITE,
            width: BORDER_WIDTH,
            radius: Radius::new(BORDER_RADIUS),
        };

        let background = None;

        let shadow = Shadow {
            color: OFF_WHITE,
            offset: iced::Vector::new(3.0, 3.0),
            blur_radius: 2.0,
        };

        let text_color = if is_active { hover_color } else { OFF_WHITE };

        match button_status {
            Active => ButtonStyle {
                border,
                background,
                text_color,
                shadow,
                ..Default::default()
            },
            Disabled => ButtonStyle {
                border,
                background,
                text_color,
                shadow,
                ..Default::default()
            },
            Hovered => ButtonStyle {
                border,
                background,
                text_color: hover_color,
                shadow,
                ..Default::default()
            },
            Pressed => ButtonStyle {
                border,
                background,
                text_color,
                shadow,
                ..Default::default()
            },
        }
    }
}
