//! Slint framework integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`slint::Color`].
//!
//! Both types store u8 RGB channels, so the conversion is lossless.
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let sc: slint::Color = color.into();
//! let back: prismatica::Color = sc.into();
//! assert_eq!(color, back); // lossless u8 roundtrip
//! ```

use crate::Color;
use crate::impl_into_framework_color;

/// Convert a prismatica [`Color`] to a [`slint::Color`].
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(255, 128, 0);
/// let sc: slint::Color = color.into();
/// assert_eq!(sc.red(), 255);
/// ```
impl From<Color> for ::slint::Color {
    fn from(c: Color) -> Self {
        ::slint::Color::from_rgb_u8(c.r, c.g, c.b)
    }
}

/// Convert a [`slint::Color`] to a prismatica [`Color`].
///
/// # Examples
///
/// ```ignore
/// let sc = slint::Color::from_rgb_u8(128, 64, 32);
/// let color: prismatica::Color = sc.into();
/// assert_eq!(color.r, 128);
/// ```
impl From<::slint::Color> for Color {
    fn from(c: ::slint::Color) -> Self {
        Color::new(c.red(), c.green(), c.blue())
    }
}

impl_into_framework_color!(::slint::Color);
