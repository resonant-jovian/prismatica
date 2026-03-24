//! Framework conversion traits.
//!
//! Provides generic traits for converting prismatica types into
//! framework-specific color types, gated behind optional features.

/// Trait for converting a prismatica [`Color`](crate::Color) to a framework-specific color type.
pub trait IntoFrameworkColor<T> {
    /// Convert this color into the framework's color type.
    fn into_framework_color(self) -> T;
}
