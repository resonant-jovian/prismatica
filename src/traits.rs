//! Framework conversion traits.
//!
//! Provides generic traits for converting prismatica types into
//! framework-specific color types, gated behind optional features.

/// Trait for converting a prismatica [`Color`](crate::Color) to a framework-specific color type.
///
/// This trait is implemented for every integration target alongside `From<Color>`.
/// Most users should prefer `.into()` directly; this trait exists for generic
/// contexts where the target type cannot be inferred.
///
/// # Examples
///
/// ```ignore
/// use prismatica::{Color, IntoFrameworkColor};
///
/// let color = Color::new(255, 128, 0);
/// let rgb: plotters::style::RGBColor = color.into_framework_color();
/// ```
pub trait IntoFrameworkColor<T> {
    /// Convert this color into the framework's color type.
    fn into_framework_color(self) -> T;
}

/// Internal macro to generate `IntoFrameworkColor<T>` impls that delegate to `From`.
#[cfg(any(
    feature = "egui-integration",
    feature = "plotters-integration",
    feature = "image-integration",
    feature = "palette-integration",
    feature = "ratatui-integration",
    feature = "crossterm-integration",
    feature = "colored-integration",
    feature = "owo-colors-integration",
    feature = "termion-integration",
    feature = "cursive-integration",
    feature = "comfy-table-integration",
    feature = "syntect-integration",
    feature = "bevy-color-integration",
    feature = "iced-integration",
    feature = "macroquad-integration",
    feature = "tiny-skia-integration",
    feature = "wgpu-integration",
    feature = "slint-integration",
))]
macro_rules! impl_into_framework_color {
    ($target:ty) => {
        impl $crate::IntoFrameworkColor<$target> for $crate::Color {
            fn into_framework_color(self) -> $target {
                self.into()
            }
        }
    };
}
#[cfg(any(
    feature = "egui-integration",
    feature = "plotters-integration",
    feature = "image-integration",
    feature = "palette-integration",
    feature = "ratatui-integration",
    feature = "crossterm-integration",
    feature = "colored-integration",
    feature = "owo-colors-integration",
    feature = "termion-integration",
    feature = "cursive-integration",
    feature = "comfy-table-integration",
    feature = "syntect-integration",
    feature = "bevy-color-integration",
    feature = "iced-integration",
    feature = "macroquad-integration",
    feature = "tiny-skia-integration",
    feature = "wgpu-integration",
    feature = "slint-integration",
))]
pub(crate) use impl_into_framework_color;
