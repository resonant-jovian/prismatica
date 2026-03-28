//! Colored crate integration.
//!
//! Provides conversion between [`Color`](crate::Color) and [`colored::Color`].
//!
//! Forward conversion always produces [`Color::TrueColor`](colored::Color::TrueColor).
//! Reverse conversion uses [`TryFrom`] since only the `TrueColor` variant can be converted.
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let col_color: colored::Color = color.into();
//!
//! // Reverse: only TrueColor variant converts
//! let back = prismatica::Color::try_from(col_color).unwrap();
//! assert_eq!(color, back);
//! ```

use crate::impl_into_framework_color;
use crate::{Color, ConversionError};

/// Convert a prismatica [`Color`] to a [`colored::Color::TrueColor`].
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(128, 64, 32);
/// let col: colored::Color = color.into();
/// ```
impl From<Color> for ::colored::Color {
    fn from(c: Color) -> Self {
        ::colored::Color::TrueColor { r: c.r, g: c.g, b: c.b }
    }
}

/// Try to convert a [`colored::Color`] to a prismatica [`Color`].
///
/// Only the `TrueColor` variant can be converted. Named colors (e.g., `Red`, `Blue`)
/// and other variants return an error.
///
/// # Errors
///
/// Returns [`ConversionError`](crate::ConversionError) if the color is not
/// a `Color::TrueColor` variant.
///
/// # Examples
///
/// ```ignore
/// use prismatica::Color;
///
/// let rgb = colored::Color::TrueColor { r: 128, g: 64, b: 32 };
/// assert!(Color::try_from(rgb).is_ok());
///
/// let named = colored::Color::Red;
/// assert!(Color::try_from(named).is_err());
/// ```
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
