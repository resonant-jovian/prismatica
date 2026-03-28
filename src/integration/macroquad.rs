//! Macroquad integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`macroquad::color::Color`].
//!
//! Forward conversion sets alpha to 1.0 (fully opaque).
//! Reverse conversion discards the alpha channel.

use super::{f32_to_u8, u8_to_f32};
use crate::impl_into_framework_color;
use crate::Color;

impl From<Color> for ::macroquad::color::Color {
    fn from(c: Color) -> Self {
        ::macroquad::color::Color::new(u8_to_f32(c.r), u8_to_f32(c.g), u8_to_f32(c.b), 1.0)
    }
}

impl From<::macroquad::color::Color> for Color {
    fn from(c: ::macroquad::color::Color) -> Self {
        Color::new(f32_to_u8(c.r), f32_to_u8(c.g), f32_to_u8(c.b))
    }
}

impl_into_framework_color!(::macroquad::color::Color);
