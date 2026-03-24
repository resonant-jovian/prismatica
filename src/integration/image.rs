//! Image crate integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`image::Rgb`].

use crate::{Color, IntoFrameworkColor};

/// Convert a prismatica Color to an image::Rgb pixel.
impl From<Color> for ::image::Rgb<u8> {
    fn from(c: Color) -> Self {
        ::image::Rgb([c.r, c.g, c.b])
    }
}

impl IntoFrameworkColor<::image::Rgb<u8>> for Color {
    fn into_framework_color(self) -> ::image::Rgb<u8> {
        self.into()
    }
}
