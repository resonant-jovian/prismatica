//! Termion integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`termion::color::Rgb`].
//!
//! **Note:** termion only supports Unix platforms.

use crate::impl_into_framework_color;
use crate::Color;

impl From<Color> for ::termion::color::Rgb {
    fn from(c: Color) -> Self {
        ::termion::color::Rgb(c.r, c.g, c.b)
    }
}

impl From<::termion::color::Rgb> for Color {
    fn from(c: ::termion::color::Rgb) -> Self {
        Color::new(c.0, c.1, c.2)
    }
}

impl_into_framework_color!(::termion::color::Rgb);
