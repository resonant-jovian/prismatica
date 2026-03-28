//! Framework integration modules.
//!
//! Each sub-module provides bidirectional conversions between [`Color`](crate::Color)
//! and a framework-specific color type, gated behind an optional feature flag.
//!
//! - Forward: `From<Color> for FrameworkType` (always infallible)
//! - Reverse: `From<FrameworkType> for Color` for struct types,
//!   `TryFrom<FrameworkType> for Color` for enum types

/// Convert a u8 channel to normalized f32 in `[0.0, 1.0]`.
#[inline]
pub(crate) fn u8_to_f32(v: u8) -> f32 {
    v as f32 / 255.0
}

/// Convert a normalized f32 in `[0.0, 1.0]` to a u8 channel with rounding.
#[inline]
pub(crate) fn f32_to_u8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}

/// Convert a u8 channel to normalized f64 in `[0.0, 1.0]`.
#[inline]
pub(crate) fn u8_to_f64(v: u8) -> f64 {
    v as f64 / 255.0
}

/// Convert a normalized f64 in `[0.0, 1.0]` to a u8 channel with rounding.
#[inline]
pub(crate) fn f64_to_u8(v: f64) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}

#[cfg(feature = "egui-integration")]
mod egui;

#[cfg(feature = "plotters-integration")]
mod plotters;

#[cfg(feature = "image-integration")]
mod image;

#[cfg(feature = "serde-support")]
mod serde_support;

#[cfg(feature = "palette-integration")]
mod palette_crate;

#[cfg(feature = "owo-colors-integration")]
mod owo_colors;

#[cfg(feature = "termion-integration")]
mod termion;

#[cfg(feature = "syntect-integration")]
mod syntect;

#[cfg(feature = "ratatui-integration")]
mod ratatui;

#[cfg(feature = "crossterm-integration")]
mod crossterm;

#[cfg(feature = "colored-integration")]
mod colored;

#[cfg(feature = "cursive-integration")]
mod cursive;

#[cfg(feature = "comfy-table-integration")]
mod comfy_table;

#[cfg(feature = "bevy-color-integration")]
mod bevy_color;

#[cfg(feature = "iced-integration")]
mod iced;

#[cfg(feature = "macroquad-integration")]
mod macroquad;

#[cfg(feature = "tiny-skia-integration")]
mod tiny_skia;

#[cfg(feature = "wgpu-integration")]
mod wgpu;

#[cfg(feature = "slint-integration")]
mod slint;
