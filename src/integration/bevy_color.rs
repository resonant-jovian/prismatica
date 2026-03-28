//! Bevy color integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`bevy_color::Srgba`].
//!
//! Forward conversion sets alpha to 1.0 (fully opaque).
//! Reverse conversion discards the alpha channel.

use super::{f32_to_u8, u8_to_f32};
use crate::impl_into_framework_color;
use crate::Color;

impl From<Color> for ::bevy_color::Srgba {
    fn from(c: Color) -> Self {
        ::bevy_color::Srgba::new(u8_to_f32(c.r), u8_to_f32(c.g), u8_to_f32(c.b), 1.0)
    }
}

impl From<::bevy_color::Srgba> for Color {
    fn from(c: ::bevy_color::Srgba) -> Self {
        Color::new(f32_to_u8(c.red), f32_to_u8(c.green), f32_to_u8(c.blue))
    }
}

impl_into_framework_color!(::bevy_color::Srgba);
