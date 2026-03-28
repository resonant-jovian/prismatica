//! Comfy-table integration.
//!
//! Provides conversion between [`Color`](crate::Color) and [`comfy_table::Color`].
//!
//! Forward conversion always produces [`Color::Rgb`](comfy_table::Color::Rgb).
//! Reverse conversion uses [`TryFrom`] since only the `Rgb` variant can be converted.

use crate::impl_into_framework_color;
use crate::{Color, ConversionError};

impl From<Color> for ::comfy_table::Color {
    fn from(c: Color) -> Self {
        ::comfy_table::Color::Rgb { r: c.r, g: c.g, b: c.b }
    }
}

impl TryFrom<::comfy_table::Color> for Color {
    type Error = ConversionError;

    fn try_from(c: ::comfy_table::Color) -> Result<Self, Self::Error> {
        match c {
            ::comfy_table::Color::Rgb { r, g, b } => Ok(Color::new(r, g, b)),
            _ => Err(ConversionError {
                message: "only comfy_table::Color::Rgb can be converted to prismatica::Color",
            }),
        }
    }
}

impl_into_framework_color!(::comfy_table::Color);
