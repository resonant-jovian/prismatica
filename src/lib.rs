//! # Prismatica -- Scientific Colormaps for Rust
//!
//! 260+ perceptually uniform, colorblind-safe colormaps from:
//! matplotlib, Crameri, CET, CMOcean, ColorBrewer, CMasher, NCAR, and more.
//!
//! ```ignore
//! use prismatica::crameri::BATLOW;
//! let color = BATLOW.eval(0.5);
//! println!("RGB: ({}, {}, {})", color.r, color.g, color.b);
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Colormaps | Description |
//! |---------|-----------|-------------|
//! | `core` (default) | ~48 | matplotlib + Crameri |
//! | `cet` | +60 | Peter Kovesi's perceptually uniform maps |
//! | `cmocean` | +22 | Oceanographic colormaps |
//! | `colorbrewer` | +35 | Cynthia Brewer's cartographic palettes |
//! | `cmasher` | +30 | Astrophysics colormaps |
//! | `ncar` | +40 | NCAR NCL geoscience maps |
//! | `cartocolors` | +15 | CARTO cartographic maps |
//! | `moreland` | +6 | Cool-warm, black body, Kindlmann |
//! | `d3` | varies | d3-scale-chromatic maps |
//! | `all` | ~260+ | Everything |
//!
//! ## Choosing a Colormap
//!
//! - **Sequential data** (temperature, elevation, concentration):
//!   `batlow`, `viridis`, `oslo`, `thermal`
//! - **Diverging data** (anomalies, residuals):
//!   `berlin`, `vik`, `balance`, `cool_warm`
//! - **Cyclic data** (phase, direction, time-of-day):
//!   `romaO`, `phase`, `twilight`
//! - **Categorical data** (labels, classes):
//!   ColorBrewer qualitative palettes: `SET2`, `DARK2`, `PAIRED`

#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod types;
pub use types::*;

mod traits;
pub use traits::*;

mod registry;

#[cfg(any(
    feature = "egui-integration",
    feature = "plotters-integration",
    feature = "image-integration",
    feature = "serde-support",
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
