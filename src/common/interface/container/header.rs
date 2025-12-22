use iced::Border;
use iced::border::Radius;
use iced::widget::container;

use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::container::common::BORDER_RADIUS;
use crate::common::interface::container::common::BORDER_WIDTH;

pub(crate) const HEADER_HEIGHT: f32 = 90.0;
pub(crate) const HEADER_PADDING: f32 = 10.0;

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
