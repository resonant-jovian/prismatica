//! Image crate integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`image::Rgb`].
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let pixel: image::Rgb<u8> = color.into();
//! let back: prismatica::Color = pixel.into();
//! assert_eq!(color, back);
//! ```

use crate::Color;
use crate::impl_into_framework_color;

/// Convert a prismatica [`Color`] to an [`image::Rgb<u8>`] pixel.
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(255, 128, 0);
/// let pixel: image::Rgb<u8> = color.into();
/// assert_eq!(pixel.0, [255, 128, 0]);
/// ```
impl From<Color> for ::image::Rgb<u8> {
    fn from(c: Color) -> Self {
        ::image::Rgb([c.r, c.g, c.b])
    }
}

/// Convert an [`image::Rgb<u8>`] pixel to a prismatica [`Color`].
///
/// # Examples
///
/// ```ignore
/// let pixel = image::Rgb([128, 64, 32]);
/// let color: prismatica::Color = pixel.into();
/// assert_eq!(color.r, 128);
/// ```
impl From<::image::Rgb<u8>> for Color {
    fn from(c: ::image::Rgb<u8>) -> Self {
        Color::new(c.0[0], c.0[1], c.0[2])
    }
}

impl_into_framework_color!(::image::Rgb<u8>);
