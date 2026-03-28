//! Iced framework integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`iced_core::Color`].
//!
//! Forward conversion sets alpha to 1.0 (fully opaque).
//! Reverse conversion discards the alpha channel.
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let ic: iced_core::Color = color.into();
//! let back: prismatica::Color = ic.into();
//! assert_eq!(color, back);
//! ```

use super::{f32_to_u8, u8_to_f32};
use crate::impl_into_framework_color;
use crate::Color;

/// Convert a prismatica [`Color`] to an [`iced_core::Color`].
///
/// The alpha channel is set to 1.0 (fully opaque).
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(255, 128, 0);
/// let ic: iced_core::Color = color.into();
/// assert_eq!(ic.a, 1.0);
/// ```
impl From<Color> for ::iced_core::Color {
    fn from(c: Color) -> Self {
        ::iced_core::Color {
            r: u8_to_f32(c.r),
            g: u8_to_f32(c.g),
            b: u8_to_f32(c.b),
            a: 1.0,
        }
    }
}

/// Convert an [`iced_core::Color`] to a prismatica [`Color`].
///
/// The alpha channel is discarded.
///
/// # Examples
///
/// ```ignore
/// let ic = iced_core::Color { r: 0.5, g: 0.25, b: 0.125, a: 0.8 };
/// let color: prismatica::Color = ic.into();
/// ```
impl From<::iced_core::Color> for Color {
    fn from(c: ::iced_core::Color) -> Self {
        Color::new(f32_to_u8(c.r), f32_to_u8(c.g), f32_to_u8(c.b))
    }
}

impl_into_framework_color!(::iced_core::Color);
