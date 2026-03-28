//! wgpu integration.
//!
//! Provides bidirectional conversion between [`Color`](crate::Color)
//! and [`wgpu_types::Color`].
//!
//! Forward conversion sets alpha to 1.0 (fully opaque).
//! Reverse conversion discards the alpha channel.

use super::{f64_to_u8, u8_to_f64};
use crate::impl_into_framework_color;
use crate::Color;

impl From<Color> for ::wgpu_types::Color {
    fn from(c: Color) -> Self {
        ::wgpu_types::Color {
            r: u8_to_f64(c.r),
            g: u8_to_f64(c.g),
            b: u8_to_f64(c.b),
            a: 1.0,
        }
    }
}

impl From<::wgpu_types::Color> for Color {
    fn from(c: ::wgpu_types::Color) -> Self {
        Color::new(f64_to_u8(c.r), f64_to_u8(c.g), f64_to_u8(c.b))
    }
}

impl_into_framework_color!(::wgpu_types::Color);
