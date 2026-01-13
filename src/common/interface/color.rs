#![allow(unused)]

use std::f32::consts::PI;

use bitcoin::Network;
use iced::Color;

/// Get the corresponding [`Color`] to a [`Network`];
pub(crate) fn network_color(network: Network) -> Color {
    match network {
        Network::Bitcoin => ORANGE,
        Network::Signet => PURPLE,
        Network::Testnet | Network::Testnet4 => BLUE,
        Network::Regtest => OFF_WHITE,
    }
}

/// Pulse a [`Color`] in a sinusoidal manner according to a timer;
pub(crate) fn pulse_color(base_color: Color, app_clock: usize) -> Color {
    let time = (app_clock as f32) * 32.0;
    let pulse = ((time / 1000.0) * PI * 2.0).sin();
    let alpha = 0.7 + ((pulse + 1.0) / 2.0) * 0.3;
    base_color.scale_alpha(alpha)
}

/// Light Grey.
pub(crate) const LIGHT_GREY: Color = Color::from_rgb(
    0x1d as f32 / 255.0,
    0x20 as f32 / 255.0,
    0x21 as f32 / 255.0,
);

/// Dark Grey.
pub(crate) const DARK_GREY: Color = Color::from_rgb(
    0x14 as f32 / 255.0,
    0x14 as f32 / 255.0,
    0x14 as f32 / 255.0,
);

/// Off White.
pub(crate) const OFF_WHITE: Color = Color::from_rgb(
    0xeb as f32 / 255.0,
    0xdb as f32 / 255.0,
    0xb2 as f32 / 255.0,
);

/// Snow White.
pub(crate) const WHITE: Color = Color::from_rgb(
    0xff as f32 / 255.0,
    0xff as f32 / 255.0,
    0xff as f32 / 255.0,
);

/// Black.
pub(crate) const BLACK: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
);

/// Gold.
pub(crate) const YELLOW: Color = Color::from_rgb(
    0xff as f32 / 255.0,
    0xd7 as f32 / 255.0,
    0x00 as f32 / 255.0,
);

/// Bitcoin Orange.
pub(crate) const ORANGE: Color = Color::from_rgb(
    0xff as f32 / 255.0,
    0x99 as f32 / 255.0,
    0x00 as f32 / 255.0,
);

/// Bloody Red.
pub(crate) const RED: Color = Color::from_rgb(
    0xfb as f32 / 255.0,
    0x49 as f32 / 255.0,
    0x34 as f32 / 255.0,
);

/// Floresta Green.
pub(crate) const GREEN_SHAMROCK: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x9e as f32 / 255.0,
    0x60 as f32 / 255.0,
);

/// Lapis Lazuli Blue.
pub(crate) const BLUE: Color = Color::from_rgb(
    0x26 as f32 / 255.0,
    0x61 as f32 / 255.0,
    0x9c as f32 / 255.0,
);

/// Bright Purple.
pub(crate) const PURPLE: Color = Color::from_rgb(
    0xBF as f32 / 255.0,
    0x40 as f32 / 255.0,
    0xBF as f32 / 255.0,
);
