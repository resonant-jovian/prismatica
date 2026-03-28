//! tiny-skia integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`tiny_skia::Color`].
//!
//! Forward conversion sets alpha to 255 (fully opaque).
//! Reverse conversion discards the alpha channel.

use super::f32_to_u8;
use crate::impl_into_framework_color;
use crate::Color;

impl From<Color> for ::tiny_skia::Color {
    fn from(c: Color) -> Self {
        ::tiny_skia::Color::from_rgba8(c.r, c.g, c.b, 255)
    }
}

impl From<::tiny_skia::Color> for Color {
    fn from(c: ::tiny_skia::Color) -> Self {
        Color::new(f32_to_u8(c.red()), f32_to_u8(c.green()), f32_to_u8(c.blue()))
    }
}

impl_into_framework_color!(::tiny_skia::Color);
