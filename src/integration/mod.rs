//! Framework integration modules.
//!
//! Each sub-module provides [`From<Color>`](crate::Color) conversions for a
//! specific visualization framework, gated behind an optional feature flag.

#[cfg(feature = "egui-integration")]
mod egui;

#[cfg(feature = "plotters-integration")]
mod plotters;

#[cfg(feature = "image-integration")]
mod image;

#[cfg(feature = "serde-support")]
mod serde_support;
