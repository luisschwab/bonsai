#![allow(unused)]

use iced::font::{Family, Font, Weight};

pub(crate) const BERKELEY_MONO_REGULAR: Font = Font {
    family: Family::Name("Berkeley Mono"),
    weight: Weight::Normal,
    ..Font::DEFAULT
};

pub(crate) const BERKELEY_MONO_BOLD: Font = Font {
    family: Family::Name("Berkeley Mono"),
    weight: Weight::Bold,
    ..Font::DEFAULT
};
