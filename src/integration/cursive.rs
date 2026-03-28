//! Cursive integration.
//!
//! Provides conversion between [`Color`](crate::Color) and
//! [`cursive_core::theme::Color`].
//!
//! Forward conversion always produces [`Color::Rgb`](cursive_core::theme::Color::Rgb).
//! Reverse conversion uses [`TryFrom`] since only the `Rgb` variant can be converted.

use crate::impl_into_framework_color;
use crate::{Color, ConversionError};

impl From<Color> for ::cursive_core::theme::Color {
    fn from(c: Color) -> Self {
        ::cursive_core::theme::Color::Rgb(c.r, c.g, c.b)
    }
}

impl TryFrom<::cursive_core::theme::Color> for Color {
    type Error = ConversionError;

    fn try_from(c: ::cursive_core::theme::Color) -> Result<Self, Self::Error> {
        match c {
            ::cursive_core::theme::Color::Rgb(r, g, b) => Ok(Color::new(r, g, b)),
            _ => Err(ConversionError {
                message: "only cursive_core::theme::Color::Rgb can be converted to prismatica::Color",
            }),
        }
    }
}

impl_into_framework_color!(::cursive_core::theme::Color);
