//! Comfy-table integration.
//!
//! Provides conversion between [`Color`](crate::Color) and [`comfy_table::Color`].
//!
//! Forward conversion always produces [`Color::Rgb`](comfy_table::Color::Rgb).
//! Reverse conversion uses [`TryFrom`] since only the `Rgb` variant can be converted.
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let ct_color: comfy_table::Color = color.into();
//!
//! // Reverse: only Rgb variant converts
//! let back = prismatica::Color::try_from(ct_color).unwrap();
//! assert_eq!(color, back);
//! ```

use crate::impl_into_framework_color;
use crate::{Color, ConversionError};

/// Convert a prismatica [`Color`] to a [`comfy_table::Color::Rgb`].
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(128, 64, 32);
/// let ct: comfy_table::Color = color.into();
/// ```
impl From<Color> for ::comfy_table::Color {
    fn from(c: Color) -> Self {
        ::comfy_table::Color::Rgb { r: c.r, g: c.g, b: c.b }
    }
}

/// Try to convert a [`comfy_table::Color`] to a prismatica [`Color`].
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
/// let rgb = comfy_table::Color::Rgb { r: 128, g: 64, b: 32 };
/// assert!(Color::try_from(rgb).is_ok());
///
/// let named = comfy_table::Color::Red;
/// assert!(Color::try_from(named).is_err());
/// ```
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
