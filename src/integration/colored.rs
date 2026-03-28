//! Colored crate integration.
//!
//! Provides conversion between [`Color`](crate::Color) and [`colored::Color`].
//!
//! Forward conversion always produces [`Color::TrueColor`](colored::Color::TrueColor).
//! Reverse conversion uses [`TryFrom`] since only the `TrueColor` variant can be converted.

use crate::impl_into_framework_color;
use crate::{Color, ConversionError};

impl From<Color> for ::colored::Color {
    fn from(c: Color) -> Self {
        ::colored::Color::TrueColor { r: c.r, g: c.g, b: c.b }
    }
}

impl TryFrom<::colored::Color> for Color {
    type Error = ConversionError;

    fn try_from(c: ::colored::Color) -> Result<Self, Self::Error> {
        match c {
            ::colored::Color::TrueColor { r, g, b } => Ok(Color::new(r, g, b)),
            _ => Err(ConversionError {
                message: "only colored::Color::TrueColor can be converted to prismatica::Color",
            }),
        }
    }
}

impl_into_framework_color!(::colored::Color);
