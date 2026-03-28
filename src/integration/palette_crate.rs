//! Palette crate integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`palette::Srgb<u8>`](::palette::Srgb).
//!
//! Both types store sRGB u8 channels, so the conversion is lossless.
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let srgb: palette::Srgb<u8> = color.into();
//! let back: prismatica::Color = srgb.into();
//! assert_eq!(color, back); // lossless u8 roundtrip
//! ```

use crate::Color;
use crate::impl_into_framework_color;

/// Convert a prismatica [`Color`] to a [`palette::Srgb<u8>`](::palette::Srgb).
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(128, 64, 32);
/// let srgb: palette::Srgb<u8> = color.into();
/// assert_eq!(srgb.red, 128);
/// ```
impl From<Color> for ::palette::Srgb<u8> {
    fn from(c: Color) -> Self {
        ::palette::Srgb::new(c.r, c.g, c.b)
    }
}

/// Convert a [`palette::Srgb<u8>`](::palette::Srgb) to a prismatica [`Color`].
///
/// # Examples
///
/// ```ignore
/// let srgb = palette::Srgb::new(128u8, 64, 32);
/// let color: prismatica::Color = srgb.into();
/// assert_eq!(color.r, 128);
/// ```
impl From<::palette::Srgb<u8>> for Color {
    fn from(c: ::palette::Srgb<u8>) -> Self {
        Color::new(c.red, c.green, c.blue)
    }
}

impl_into_framework_color!(::palette::Srgb<u8>);
