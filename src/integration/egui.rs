//! Egui framework integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color) and [`egui::Color32`].
//!
//! # Examples
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//!
//! let color = BATLOW.eval(0.5);
//! let c32: egui::Color32 = color.into();
//! let back: prismatica::Color = c32.into();
//! assert_eq!(color, back);
//! ```

use crate::Color;
use crate::impl_into_framework_color;

/// Convert a prismatica [`Color`] to an [`egui::Color32`].
///
/// # Examples
///
/// ```ignore
/// let color = prismatica::Color::new(255, 128, 0);
/// let c32: egui::Color32 = color.into();
/// assert_eq!(c32.r(), 255);
/// ```
impl From<Color> for ::egui::Color32 {
    fn from(c: Color) -> Self {
        ::egui::Color32::from_rgb(c.r, c.g, c.b)
    }
}

/// Convert an [`egui::Color32`] to a prismatica [`Color`].
///
/// The alpha channel is discarded.
///
/// # Examples
///
/// ```ignore
/// let c32 = egui::Color32::from_rgb(128, 64, 32);
/// let color: prismatica::Color = c32.into();
/// assert_eq!(color.r, 128);
/// ```
impl From<::egui::Color32> for Color {
    fn from(c: ::egui::Color32) -> Self {
        Color::new(c.r(), c.g(), c.b())
    }
}

impl_into_framework_color!(::egui::Color32);
