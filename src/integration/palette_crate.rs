//! Palette crate integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`palette::Srgb<u8>`](::palette::Srgb).

use crate::impl_into_framework_color;
use crate::Color;

impl From<Color> for ::palette::Srgb<u8> {
    fn from(c: Color) -> Self {
        ::palette::Srgb::new(c.r, c.g, c.b)
    }
}

impl From<::palette::Srgb<u8>> for Color {
    fn from(c: ::palette::Srgb<u8>) -> Self {
        Color::new(c.red, c.green, c.blue)
    }
}

impl_into_framework_color!(::palette::Srgb<u8>);
