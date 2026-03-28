//! Syntect integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`syntect::highlighting::Color`].
//!
//! Forward conversion sets alpha to 255 (fully opaque).
//! Reverse conversion discards the alpha channel.
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let sc: syntect::highlighting::Color = color.into();
//! let back: prismatica::Color = sc.into();
//! assert_eq!(color, back);
//! ```

use crate::impl_into_framework_color;
use crate::Color;

/// Convert a prismatica [`Color`] to a [`syntect::highlighting::Color`].
///
/// The alpha channel is set to 255 (fully opaque).
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(255, 128, 0);
/// let sc: syntect::highlighting::Color = color.into();
/// assert_eq!(sc.r, 255);
/// assert_eq!(sc.a, 255);
/// ```
impl From<Color> for ::syntect::highlighting::Color {
    fn from(c: Color) -> Self {
        ::syntect::highlighting::Color {
            r: c.r,
            g: c.g,
            b: c.b,
            a: 255,
        }
    }
}

/// Convert a [`syntect::highlighting::Color`] to a prismatica [`Color`].
///
/// The alpha channel is discarded.
///
/// # Examples
///
/// ```ignore
/// let sc = syntect::highlighting::Color { r: 128, g: 64, b: 32, a: 200 };
/// let color: prismatica::Color = sc.into();
/// assert_eq!(color.r, 128);
/// ```
impl From<::syntect::highlighting::Color> for Color {
    fn from(c: ::syntect::highlighting::Color) -> Self {
        Color::new(c.r, c.g, c.b)
    }
}

impl_into_framework_color!(::syntect::highlighting::Color);
