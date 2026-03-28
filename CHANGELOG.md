# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.3.0] - 2026-03-28

### Added

- 15 new framework integrations: palette, bevy_color, ratatui, crossterm, iced,
  macroquad, tiny-skia, wgpu, colored, owo-colors, termion, cursive, comfy-table,
  slint, syntect
- Bidirectional conversion (reverse `From`/`TryFrom`) for all integrations
- `ConversionError` type for fallible reverse conversions on enum-based color types
- `all-integrations` meta-feature flag
- `impl_into_framework_color!` internal macro for integration boilerplate
- Conversion helpers: `u8_to_f32`, `f32_to_u8`, `u8_to_f64`, `f64_to_u8`
- 22 examples: 18 integration examples + 4 general-purpose examples
- Full inline documentation with examples for all integration modules

### Changed

- Existing egui, plotters, and image integrations are now bidirectional

## [0.2.0] - 2025-05-01

- 308 scientific colormaps from 10 collections (CET, CMOcean, ColorBrewer,
  CMasher, NCAR, CartoColors, Moreland, d3 added to core matplotlib + Crameri)
- 70 discrete palettes (ColorBrewer, CartoColors, d3)
- Framework integrations: egui, plotters, image, serde
- Registry API: `find_by_name()`, `all_colormaps()`, `filter_by_kind()`, `filter_by_collection()`

## [0.1.0] - 2025-04-01

Initial release.

- Core types: `Color`, `Colormap`, `ColormapMeta`, `ColormapKind`, `DiscretePalette`
- matplotlib (8 maps) and Crameri (40 maps) collections
- `#![no_std]` support with optional `alloc`/`std` features
