//! Ratatui integration.
//!
//! Provides conversion between [`Color`](crate::Color) and [`ratatui::style::Color`].
//!
//! Forward conversion always produces [`Color::Rgb`](ratatui::style::Color::Rgb).
//! Reverse conversion uses [`TryFrom`] since only the `Rgb` variant can be converted.

use crate::impl_into_framework_color;
use crate::{Color, ConversionError};

impl From<Color> for ::ratatui::style::Color {
    fn from(c: Color) -> Self {
        ::ratatui::style::Color::Rgb(c.r, c.g, c.b)
    }
}

impl TryFrom<::ratatui::style::Color> for Color {
    type Error = ConversionError;

    fn try_from(c: ::ratatui::style::Color) -> Result<Self, Self::Error> {
        match c {
            ::ratatui::style::Color::Rgb(r, g, b) => Ok(Color::new(r, g, b)),
            _ => Err(ConversionError {
                message: "only ratatui::style::Color::Rgb can be converted to prismatica::Color",
            }),
        }
    }
}

impl_into_framework_color!(::ratatui::style::Color);
