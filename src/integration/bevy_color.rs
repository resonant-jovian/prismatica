//! Bevy color integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`bevy_color::Srgba`].
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
//! let srgba: bevy_color::Srgba = color.into();
//! let back: prismatica::Color = srgba.into();
//! assert_eq!(color, back);
//! ```

use super::{f32_to_u8, u8_to_f32};
use crate::impl_into_framework_color;
use crate::Color;

/// Convert a prismatica [`Color`] to a [`bevy_color::Srgba`].
///
/// The alpha channel is set to 1.0 (fully opaque).
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(255, 128, 0);
/// let srgba: bevy_color::Srgba = color.into();
/// assert_eq!(srgba.alpha, 1.0);
/// ```
impl From<Color> for ::bevy_color::Srgba {
    fn from(c: Color) -> Self {
        ::bevy_color::Srgba::new(u8_to_f32(c.r), u8_to_f32(c.g), u8_to_f32(c.b), 1.0)
    }
}

/// Convert a [`bevy_color::Srgba`] to a prismatica [`Color`].
///
/// The alpha channel is discarded.
///
/// # Examples
///
/// ```ignore
/// let srgba = bevy_color::Srgba::new(0.5, 0.25, 0.125, 0.8);
/// let color: prismatica::Color = srgba.into();
/// ```
impl From<::bevy_color::Srgba> for Color {
    fn from(c: ::bevy_color::Srgba) -> Self {
        Color::new(f32_to_u8(c.red), f32_to_u8(c.green), f32_to_u8(c.blue))
    }
}

impl_into_framework_color!(::bevy_color::Srgba);
