use iced::border::Radius;
use iced::widget::container::Style as ContainerStyle;
use iced::widget::text_input::Style as TextInputStyle;
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

pub(crate) fn table_cell() -> impl Fn(&Theme) -> ContainerStyle {
    move |_theme| {
        let border = Border {
            color: OFF_WHITE,
            width: BORDER_WIDTH,
            radius: Radius::new(BORDER_RADIUS),
        };

        ContainerStyle {
            border,
            ..Default::default()
        }
    }
}

pub(crate) fn input_field() -> impl Fn(&Theme, iced::widget::text_input::Status) -> TextInputStyle {
    move |_theme, _status| TextInputStyle {
        background: iced::Background::Color(iced::Color::TRANSPARENT),
        border: Border {
            color: OFF_WHITE,
            width: BORDER_WIDTH,
            radius: Radius::new(0.0),
        },
        icon: OFF_WHITE,
        placeholder: OFF_WHITE.scale_alpha(0.5),
        value: OFF_WHITE,
        selection: OFF_WHITE,
    }
}
