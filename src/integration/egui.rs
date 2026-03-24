//! Egui framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`egui::Color32`].

use crate::{Color, IntoFrameworkColor};

/// Convert a prismatica Color to an egui Color32.
impl From<Color> for ::egui::Color32 {
    fn from(c: Color) -> Self {
        ::egui::Color32::from_rgb(c.r, c.g, c.b)
    }
}

impl IntoFrameworkColor<::egui::Color32> for Color {
    fn into_framework_color(self) -> ::egui::Color32 {
        self.into()
    }
}
