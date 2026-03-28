//! Termion integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`termion::color::Rgb`].
//!
//! Both types store u8 RGB channels, so the conversion is lossless.
//!
//! **Note:** termion only supports Unix platforms.
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let rgb: termion::color::Rgb = color.into();
//! let back: prismatica::Color = rgb.into();
//! assert_eq!(color, back); // lossless u8 roundtrip
//! ```

use crate::Color;
use crate::impl_into_framework_color;

/// Convert a prismatica [`Color`] to a [`termion::color::Rgb`].
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(255, 128, 0);
/// let rgb: termion::color::Rgb = color.into();
/// assert_eq!(rgb.0, 255);
/// ```
impl From<Color> for ::termion::color::Rgb {
    fn from(c: Color) -> Self {
        ::termion::color::Rgb(c.r, c.g, c.b)
    }
}

/// Convert a [`termion::color::Rgb`] to a prismatica [`Color`].
///
/// # Examples
///
/// ```ignore
/// let rgb = termion::color::Rgb(128, 64, 32);
/// let color: prismatica::Color = rgb.into();
/// assert_eq!(color.r, 128);
/// ```
impl From<::termion::color::Rgb> for Color {
    fn from(c: ::termion::color::Rgb) -> Self {
        Color::new(c.0, c.1, c.2)
    }
}

impl_into_framework_color!(::termion::color::Rgb);
