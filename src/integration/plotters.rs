//! Plotters framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`plotters::style::RGBColor`].

use crate::{Color, IntoFrameworkColor};

/// Convert a prismatica Color to a plotters RGBColor.
impl From<Color> for ::plotters::style::RGBColor {
    fn from(c: Color) -> Self {
        ::plotters::style::RGBColor(c.r, c.g, c.b)
    }
}

impl IntoFrameworkColor<::plotters::style::RGBColor> for Color {
    fn into_framework_color(self) -> ::plotters::style::RGBColor {
        self.into()
    }
}
