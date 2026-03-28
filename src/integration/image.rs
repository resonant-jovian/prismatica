//! Image crate integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`image::Rgb`].

use crate::impl_into_framework_color;
use crate::Color;

impl From<Color> for ::image::Rgb<u8> {
    fn from(c: Color) -> Self {
        ::image::Rgb([c.r, c.g, c.b])
    }
}

impl From<::image::Rgb<u8>> for Color {
    fn from(c: ::image::Rgb<u8>) -> Self {
        Color::new(c.0[0], c.0[1], c.0[2])
    }
}

impl_into_framework_color!(::image::Rgb<u8>);
