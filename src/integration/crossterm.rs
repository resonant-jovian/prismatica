//! Crossterm integration.
//!
//! Provides conversion between [`Color`](crate::Color) and [`crossterm::style::Color`].
//!
//! Forward conversion always produces [`Color::Rgb`](crossterm::style::Color::Rgb).
//! Reverse conversion uses [`TryFrom`] since only the `Rgb` variant can be converted.

use crate::impl_into_framework_color;
use crate::{Color, ConversionError};

impl From<Color> for ::crossterm::style::Color {
    fn from(c: Color) -> Self {
        ::crossterm::style::Color::Rgb { r: c.r, g: c.g, b: c.b }
    }
}

impl TryFrom<::crossterm::style::Color> for Color {
    type Error = ConversionError;

    fn try_from(c: ::crossterm::style::Color) -> Result<Self, Self::Error> {
        match c {
            ::crossterm::style::Color::Rgb { r, g, b } => Ok(Color::new(r, g, b)),
            _ => Err(ConversionError {
                message: "only crossterm::style::Color::Rgb can be converted to prismatica::Color",
            }),
        }
    }
}

impl_into_framework_color!(::crossterm::style::Color);
