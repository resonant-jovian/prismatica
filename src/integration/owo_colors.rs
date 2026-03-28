//! owo-colors integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`owo_colors::Rgb`].
//!
//! Both types store u8 RGB channels, so the conversion is lossless.
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let rgb: owo_colors::Rgb = color.into();
//! let back: prismatica::Color = rgb.into();
//! assert_eq!(color, back); // lossless u8 roundtrip
//! ```

use crate::Color;
use crate::impl_into_framework_color;

/// Convert a prismatica [`Color`] to an [`owo_colors::Rgb`].
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(255, 128, 0);
/// let rgb: owo_colors::Rgb = color.into();
/// assert_eq!(rgb.0, 255);
/// ```
impl From<Color> for ::owo_colors::Rgb {
    fn from(c: Color) -> Self {
        ::owo_colors::Rgb(c.r, c.g, c.b)
    }
}

/// Convert an [`owo_colors::Rgb`] to a prismatica [`Color`].
///
/// # Examples
///
/// ```ignore
/// let rgb = owo_colors::Rgb(128, 64, 32);
/// let color: prismatica::Color = rgb.into();
/// assert_eq!(color.r, 128);
/// ```
impl From<::owo_colors::Rgb> for Color {
    fn from(c: ::owo_colors::Rgb) -> Self {
        Color::new(c.0, c.1, c.2)
    }
}

impl_into_framework_color!(::owo_colors::Rgb);
