//! Egui framework integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color) and [`egui::Color32`].

use crate::impl_into_framework_color;
use crate::Color;

impl From<Color> for ::egui::Color32 {
    fn from(c: Color) -> Self {
        ::egui::Color32::from_rgb(c.r, c.g, c.b)
    }
}

impl From<::egui::Color32> for Color {
    fn from(c: ::egui::Color32) -> Self {
        Color::new(c.r(), c.g(), c.b())
    }
}

impl_into_framework_color!(::egui::Color32);
