//! tiny-skia integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`tiny_skia::Color`].
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
//! let tc: tiny_skia::Color = color.into();
//! let back: prismatica::Color = tc.into();
//! assert_eq!(color, back);
//! ```

use super::f32_to_u8;
use crate::impl_into_framework_color;
use crate::Color;

/// Convert a prismatica [`Color`] to a [`tiny_skia::Color`].
///
/// The alpha channel is set to 255 (fully opaque).
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(255, 128, 0);
/// let tc: tiny_skia::Color = color.into();
/// ```
impl From<Color> for ::tiny_skia::Color {
    fn from(c: Color) -> Self {
        ::tiny_skia::Color::from_rgba8(c.r, c.g, c.b, 255)
    }
}

/// Convert a [`tiny_skia::Color`] to a prismatica [`Color`].
///
/// The alpha channel is discarded.
///
/// # Examples
///
/// ```ignore
/// let tc = tiny_skia::Color::from_rgba8(128, 64, 32, 200);
/// let color: prismatica::Color = tc.into();
/// assert_eq!(color.r, 128);
/// ```
impl From<::tiny_skia::Color> for Color {
    fn from(c: ::tiny_skia::Color) -> Self {
        Color::new(f32_to_u8(c.red()), f32_to_u8(c.green()), f32_to_u8(c.blue()))
    }
}

impl_into_framework_color!(::tiny_skia::Color);
