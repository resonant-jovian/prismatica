//! Iced framework integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`iced_core::Color`].
//!
//! Forward conversion sets alpha to 1.0 (fully opaque).
//! Reverse conversion discards the alpha channel.

use super::{f32_to_u8, u8_to_f32};
use crate::impl_into_framework_color;
use crate::Color;

impl From<Color> for ::iced_core::Color {
    fn from(c: Color) -> Self {
        ::iced_core::Color {
            r: u8_to_f32(c.r),
            g: u8_to_f32(c.g),
            b: u8_to_f32(c.b),
            a: 1.0,
        }
    }
}

impl From<::iced_core::Color> for Color {
    fn from(c: ::iced_core::Color) -> Self {
        Color::new(f32_to_u8(c.r), f32_to_u8(c.g), f32_to_u8(c.b))
    }
}

impl_into_framework_color!(::iced_core::Color);
