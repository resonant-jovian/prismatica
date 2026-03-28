//! # Prismatica -- Scientific Colormaps for Rust
//!
//! 308 scientific colormaps + 70 discrete palettes from 10 collections:
//! matplotlib, Crameri, CET, CMOcean, ColorBrewer, CMasher, NCAR, CartoColors, Moreland, and d3.
//!
//! ```
//! use prismatica::crameri::BATLOW;
//! let color = BATLOW.eval(0.5);
//! println!("RGB: ({}, {}, {})", color.r, color.g, color.b);
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Colormaps | Description |
//! |---------|-----------|-------------|
//! | `core` (default) | 48 | matplotlib (8) + Crameri (40) |
//! | `cet` | +59 | Peter Kovesi's perceptually uniform maps |
//! | `cmocean` | +22 | Oceanographic colormaps |
//! | `colorbrewer` | +35 (+35 palettes) | Cynthia Brewer's cartographic palettes |
//! | `cmasher` | +53 | Astrophysics colormaps |
//! | `ncar` | +44 | NCAR NCL geoscience maps |
//! | `cartocolors` | +34 (+34 palettes) | CARTO cartographic maps |
//! | `moreland` | +6 | Cool-warm, black body, Kindlmann |
//! | `d3` | +7 (+1 palette) | Turbo, Rainbow, Sinebow, Cubehelix, Tableau10 |
//! | `all` | 308 (+70 palettes) | Everything |
//!
//! ## Framework Integrations
//!
//! All integrations provide bidirectional conversion via `From`/`Into`.
//! Enum-based types (ratatui, crossterm, colored, cursive, comfy-table) use
//! `TryFrom` for the reverse direction.
//!
//! | Feature | Framework | Type |
//! |---------|-----------|------|
//! | `egui-integration` | egui | `Color32` |
//! | `plotters-integration` | plotters | `RGBColor` |
//! | `image-integration` | image | `Rgb<u8>` |
//! | `palette-integration` | palette | `Srgb<u8>` |
//! | `bevy-color-integration` | bevy_color | `Srgba` |
//! | `iced-integration` | iced | `iced::Color` |
//! | `macroquad-integration` | macroquad | `macroquad::Color` |
//! | `tiny-skia-integration` | tiny-skia | `tiny_skia::Color` |
//! | `wgpu-integration` | wgpu | `wgpu::Color` |
//! | `slint-integration` | slint | `slint::Color` |
//! | `ratatui-integration` | ratatui | `ratatui::Color` |
//! | `crossterm-integration` | crossterm | `crossterm::Color` |
//! | `colored-integration` | colored | `colored::Color` |
//! | `owo-colors-integration` | owo-colors | `owo_colors::Rgb` |
//! | `termion-integration` | termion | `termion::color::Rgb` |
//! | `cursive-integration` | cursive | `cursive::Color` |
//! | `comfy-table-integration` | comfy-table | `comfy_table::Color` |
//! | `syntect-integration` | syntect | `syntect::Color` |
//! | `serde-support` | serde | Serialize/Deserialize |
//! | `all-integrations` | all of the above | |
//!
//! ## Choosing a Colormap
//!
//! - **Sequential data** (temperature, elevation, concentration):
//!   `batlow`, `viridis`, `oslo`, `thermal`
//! - **Diverging data** (anomalies, residuals):
//!   `berlin`, `vik`, `balance`, `smooth-cool-warm`
//! - **Cyclic data** (phase, direction, time-of-day):
//!   `romaO`, `phase`, `twilight`
//! - **Categorical data** (labels, classes):
//!   `SET2`, `DARK2`, `PAIRED`, `Tableau10`
//!
//! ## Minimum Supported Rust Version
//!
//! Prismatica requires Rust **1.85** or later (edition 2024).

#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod types;
pub use types::*;

mod traits;
pub use traits::*;

pub mod prelude;
mod registry;

#[cfg(any(
    feature = "egui-integration",
    feature = "plotters-integration",
    feature = "image-integration",
    feature = "serde-support",
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
mod integration;
pub use registry::*;

// -- Collection modules (feature-gated) --

#[cfg(feature = "matplotlib")]
pub mod matplotlib;

#[cfg(feature = "crameri")]
pub mod crameri;

#[cfg(feature = "cet")]
pub mod cet;

#[cfg(feature = "cmocean")]
pub mod cmocean;

#[cfg(feature = "colorbrewer")]
pub mod colorbrewer;

#[cfg(feature = "cmasher")]
pub mod cmasher;

#[cfg(feature = "ncar")]
pub mod ncar;

#[cfg(feature = "cartocolors")]
pub mod cartocolors;

#[cfg(feature = "moreland")]
pub mod moreland;

#[cfg(feature = "d3")]
pub mod d3;
