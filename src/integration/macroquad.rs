//! Macroquad integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`macroquad::color::Color`].
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
//! let mc: macroquad::color::Color = color.into();
//! let back: prismatica::Color = mc.into();
//! assert_eq!(color, back);
//! ```

use super::{f32_to_u8, u8_to_f32};
use crate::impl_into_framework_color;
use crate::Color;

/// Convert a prismatica [`Color`] to a [`macroquad::color::Color`].
///
/// The alpha channel is set to 1.0 (fully opaque).
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(255, 128, 0);
/// let mc: macroquad::color::Color = color.into();
/// assert_eq!(mc.a, 1.0);
/// ```
impl From<Color> for ::macroquad::color::Color {
    fn from(c: Color) -> Self {
        ::macroquad::color::Color::new(u8_to_f32(c.r), u8_to_f32(c.g), u8_to_f32(c.b), 1.0)
    }
}

/// Convert a [`macroquad::color::Color`] to a prismatica [`Color`].
///
/// The alpha channel is discarded.
///
/// # Examples
///
/// ```ignore
/// let mc = macroquad::color::Color::new(0.5, 0.25, 0.125, 0.8);
/// let color: prismatica::Color = mc.into();
/// ```
impl From<::macroquad::color::Color> for Color {
    fn from(c: ::macroquad::color::Color) -> Self {
        Color::new(f32_to_u8(c.r), f32_to_u8(c.g), f32_to_u8(c.b))
    }
}

impl_into_framework_color!(::macroquad::color::Color);
