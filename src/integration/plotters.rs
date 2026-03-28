//! Plotters framework integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`plotters::style::RGBColor`].
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let rgb: plotters::style::RGBColor = color.into();
//! let back: prismatica::Color = rgb.into();
//! assert_eq!(color, back);
//! ```

use crate::impl_into_framework_color;
use crate::Color;

/// Convert a prismatica [`Color`] to a [`plotters::style::RGBColor`].
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(255, 128, 0);
/// let rgb: plotters::style::RGBColor = color.into();
/// ```
impl From<Color> for ::plotters::style::RGBColor {
    fn from(c: Color) -> Self {
        ::plotters::style::RGBColor(c.r, c.g, c.b)
    }
}

/// Convert a [`plotters::style::RGBColor`] to a prismatica [`Color`].
///
/// # Examples
///
/// ```ignore
/// let rgb = plotters::style::RGBColor(128, 64, 32);
/// let color: prismatica::Color = rgb.into();
/// assert_eq!(color.r, 128);
/// ```
impl From<::plotters::style::RGBColor> for Color {
    fn from(c: ::plotters::style::RGBColor) -> Self {
        Color::new(c.0, c.1, c.2)
    }
}

impl_into_framework_color!(::plotters::style::RGBColor);
