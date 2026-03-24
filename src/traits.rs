//! Framework conversion traits.
//!
//! This module will provide feature-gated conversions between prismatica
//! types and popular Rust visualization frameworks:
//!
//! - `plotters-integration`: `Color` to `plotters::style::RGBColor`
//! - `egui-integration`: `Color` to `egui::Color32`
//! - `image-integration`: `Color` to `image::Rgb<u8>`
//! - `serde-support`: Serialization for `ColormapMeta`
