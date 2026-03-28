//! Plotters framework integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`plotters::style::RGBColor`].

use crate::impl_into_framework_color;
use crate::Color;

impl From<Color> for ::plotters::style::RGBColor {
    fn from(c: Color) -> Self {
        ::plotters::style::RGBColor(c.r, c.g, c.b)
    }
}

impl From<::plotters::style::RGBColor> for Color {
    fn from(c: ::plotters::style::RGBColor) -> Self {
        Color::new(c.0, c.1, c.2)
    }
}

impl_into_framework_color!(::plotters::style::RGBColor);
