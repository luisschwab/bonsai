use iced::Shadow;

pub(crate) const BORDER_WIDTH: f32 = 1.5;
pub(crate) const BORDER_RADIUS: f32 = 0.0;

use crate::common::interface::color::OFF_WHITE;

pub(crate) const SHADOW: Shadow = Shadow {
    color: OFF_WHITE,
    offset: iced::Vector::new(3.0, 3.0),
    blur_radius: 2.0,
};
