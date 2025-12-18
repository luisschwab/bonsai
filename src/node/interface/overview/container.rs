use iced::border::Radius;
use iced::widget::container::Style as ContainerStyle;
use iced::{Border, Theme};

use crate::common::interface::container::common::{BORDER_RADIUS, BORDER_WIDTH, SHADOW};
use crate::common::interface::color::OFF_WHITE;

/// Container for displaying logs through a `tracing_subscriber`.
pub(crate) fn log_container() -> impl Fn(&Theme) -> ContainerStyle {
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
