//! wgpu integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`wgpu_types::Color`].
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
//! let wc: wgpu_types::Color = color.into();
//! let back: prismatica::Color = wc.into();
//! assert_eq!(color, back);
//! ```

use super::{f64_to_u8, u8_to_f64};
use crate::impl_into_framework_color;
use crate::Color;

/// Convert a prismatica [`Color`] to a [`wgpu_types::Color`].
///
/// The alpha channel is set to 1.0 (fully opaque).
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(255, 128, 0);
/// let wc: wgpu_types::Color = color.into();
/// assert_eq!(wc.a, 1.0);
/// ```
impl From<Color> for ::wgpu_types::Color {
    fn from(c: Color) -> Self {
        ::wgpu_types::Color {
            r: u8_to_f64(c.r),
            g: u8_to_f64(c.g),
            b: u8_to_f64(c.b),
            a: 1.0,
        }
    }
}

/// Convert a [`wgpu_types::Color`] to a prismatica [`Color`].
///
/// The alpha channel is discarded.
///
/// # Examples
///
/// ```ignore
/// let wc = wgpu_types::Color { r: 0.5, g: 0.25, b: 0.125, a: 0.8 };
/// let color: prismatica::Color = wc.into();
/// ```
impl From<::wgpu_types::Color> for Color {
    fn from(c: ::wgpu_types::Color) -> Self {
        Color::new(f64_to_u8(c.r), f64_to_u8(c.g), f64_to_u8(c.b))
    }
}

impl_into_framework_color!(::wgpu_types::Color);
