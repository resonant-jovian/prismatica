//! Syntect integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`syntect::highlighting::Color`].
//!
//! Forward conversion sets alpha to 255 (fully opaque).
//! Reverse conversion discards the alpha channel.

use crate::impl_into_framework_color;
use crate::Color;

impl From<Color> for ::syntect::highlighting::Color {
    fn from(c: Color) -> Self {
        ::syntect::highlighting::Color {
            r: c.r,
            g: c.g,
            b: c.b,
            a: 255,
        }
    }
}

impl From<::syntect::highlighting::Color> for Color {
    fn from(c: ::syntect::highlighting::Color) -> Self {
        Color::new(c.r, c.g, c.b)
    }
}

impl_into_framework_color!(::syntect::highlighting::Color);
