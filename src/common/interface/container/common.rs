use iced::Length;
use iced::Shadow;
use iced::Theme;
use iced::widget::container::Style as ContainerStyle;

pub(crate) const BORDER_WIDTH: f32 = 1.5;
pub(crate) const BORDER_RADIUS: f32 = 0.0;

pub(crate) const CELL_HEIGHT: Length = Length::Fixed(35.0);
pub(crate) const CELL_HEIGHT_2X: Length = Length::Fixed(70.0);
pub(crate) const TABLE_HEADER_FONT_SIZE: u32 = 16;
pub(crate) const TABLE_CELL_FONT_SIZE: u32 = 12;
pub(crate) const TABLE_CELL_ICON_SIZE: u32 = 24;

use crate::common::interface::color::OFF_WHITE;

pub(crate) const SHADOW: Shadow = Shadow {
    color: OFF_WHITE,
    offset: iced::Vector::new(3.0, 3.0),
    blur_radius: 2.0,
};

pub fn shadow_container() -> impl Fn(&Theme) -> ContainerStyle {
    |_theme: &Theme| ContainerStyle {
        shadow: SHADOW,
        ..Default::default()
    }
}
