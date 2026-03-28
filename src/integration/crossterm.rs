//! Crossterm integration.
//!
//! Provides conversion between [`Color`](crate::Color) and [`crossterm::style::Color`].
//!
//! Forward conversion always produces [`Color::Rgb`](crossterm::style::Color::Rgb).
//! Reverse conversion uses [`TryFrom`] since only the `Rgb` variant can be converted.
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let ct_color: crossterm::style::Color = color.into();
//!
//! // Reverse: only Rgb variant converts
//! let back = prismatica::Color::try_from(ct_color).unwrap();
//! assert_eq!(color, back);
//! ```

use crate::impl_into_framework_color;
use crate::{Color, ConversionError};

/// Convert a prismatica [`Color`] to a [`crossterm::style::Color::Rgb`].
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(128, 64, 32);
/// let ct: crossterm::style::Color = color.into();
/// ```
impl From<Color> for ::crossterm::style::Color {
    fn from(c: Color) -> Self {
        ::crossterm::style::Color::Rgb {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }
}

/// Try to convert a [`crossterm::style::Color`] to a prismatica [`Color`].
///
/// Only the `Rgb` variant can be converted. Named colors (e.g., `Red`, `Blue`)
/// and other variants return an error.
///
/// # Errors
///
/// Returns [`ConversionError`](crate::ConversionError) if the color is not
/// a `Color::Rgb` variant.
///
/// # Examples
///
/// ```ignore
/// use prismatica::Color;
///
/// let rgb = crossterm::style::Color::Rgb { r: 128, g: 64, b: 32 };
/// assert!(Color::try_from(rgb).is_ok());
///
/// let named = crossterm::style::Color::Red;
/// assert!(Color::try_from(named).is_err());
/// ```
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
