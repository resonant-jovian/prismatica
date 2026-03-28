//! Ratatui integration.
//!
//! Provides conversion between [`Color`](crate::Color) and [`ratatui::style::Color`].
//!
//! Forward conversion always produces [`Color::Rgb`](ratatui::style::Color::Rgb).
//! Reverse conversion uses [`TryFrom`] since only the `Rgb` variant can be converted.
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let rat_color: ratatui::style::Color = color.into();
//!
//! // Reverse: only Rgb variant converts
//! let back = prismatica::Color::try_from(rat_color).unwrap();
//! assert_eq!(color, back);
//! ```

use crate::impl_into_framework_color;
use crate::{Color, ConversionError};

/// Convert a prismatica [`Color`] to a [`ratatui::style::Color::Rgb`].
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(128, 64, 32);
/// let rat: ratatui::style::Color = color.into();
/// ```
impl From<Color> for ::ratatui::style::Color {
    fn from(c: Color) -> Self {
        ::ratatui::style::Color::Rgb(c.r, c.g, c.b)
    }
}

/// Try to convert a [`ratatui::style::Color`] to a prismatica [`Color`].
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
/// let rgb = ratatui::style::Color::Rgb(128, 64, 32);
/// assert!(Color::try_from(rgb).is_ok());
///
/// let named = ratatui::style::Color::Red;
/// assert!(Color::try_from(named).is_err());
/// ```
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
