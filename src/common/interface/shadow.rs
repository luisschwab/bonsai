use iced::Shadow;
use iced::Vector;

use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::color::RED;

pub(crate) const SHADOW_GRAY: Shadow = Shadow {
    color: OFF_WHITE,
    offset: Vector::new(3.0, 3.0),
    blur_radius: 2.0,
};

pub(crate) const SHADOW_RED: Shadow = Shadow {
    color: RED,
    offset: Vector::new(3.0, 3.0),
    blur_radius: 2.0,
};
