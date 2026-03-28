//! Cursive integration.
//!
//! Provides conversion between [`Color`](crate::Color) and
//! [`cursive_core::theme::Color`].
//!
//! Forward conversion always produces [`Color::Rgb`](cursive_core::theme::Color::Rgb).
//! Reverse conversion uses [`TryFrom`] since only the `Rgb` variant can be converted.
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let cur_color: cursive_core::theme::Color = color.into();
//!
//! // Reverse: only Rgb variant converts
//! let back = prismatica::Color::try_from(cur_color).unwrap();
//! assert_eq!(color, back);
//! ```

use crate::impl_into_framework_color;
use crate::{Color, ConversionError};

/// Convert a prismatica [`Color`] to a [`cursive_core::theme::Color::Rgb`].
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(128, 64, 32);
/// let cur: cursive_core::theme::Color = color.into();
/// ```
impl From<Color> for ::cursive_core::theme::Color {
    fn from(c: Color) -> Self {
        ::cursive_core::theme::Color::Rgb(c.r, c.g, c.b)
    }
}

/// Try to convert a [`cursive_core::theme::Color`] to a prismatica [`Color`].
///
/// Only the `Rgb` variant can be converted. Named colors (e.g., `Dark`, `Light`)
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
/// let rgb = cursive_core::theme::Color::Rgb(128, 64, 32);
/// assert!(Color::try_from(rgb).is_ok());
///
/// let named = cursive_core::theme::Color::Dark(cursive_core::theme::BaseColor::Red);
/// assert!(Color::try_from(named).is_err());
/// ```
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
