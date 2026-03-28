//! owo-colors integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`owo_colors::Rgb`].

use crate::impl_into_framework_color;
use crate::Color;

impl From<Color> for ::owo_colors::Rgb {
    fn from(c: Color) -> Self {
        ::owo_colors::Rgb(c.r, c.g, c.b)
    }
}

impl From<::owo_colors::Rgb> for Color {
    fn from(c: ::owo_colors::Rgb) -> Self {
        Color::new(c.0, c.1, c.2)
    }
}

impl_into_framework_color!(::owo_colors::Rgb);
