//! Slint framework integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`slint::Color`].

use crate::impl_into_framework_color;
use crate::Color;

impl From<Color> for ::slint::Color {
    fn from(c: Color) -> Self {
        ::slint::Color::from_rgb_u8(c.r, c.g, c.b)
    }
}

impl From<::slint::Color> for Color {
    fn from(c: ::slint::Color) -> Self {
        Color::new(c.red(), c.green(), c.blue())
    }
}

impl_into_framework_color!(::slint::Color);
