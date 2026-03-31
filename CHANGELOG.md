# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [1.0.0] — 2026-03-31

### Added

- `Ord` and `PartialOrd` derives on `Color` for sorting support
- 3-digit CSS hex shorthand support in `from_css_hex()` (e.g., `#FFF` -> `#FFFFFF`)
- `from_css_hex()` is now `const fn`
- `FromStr` impl for `Color` (enables `"#ff8800".parse::<Color>()`)
- `From<u32>` impl for `Color` (enables `Color::from(0xFF8800)`)
- `ParseColorError` type for `FromStr` failures
- WASM compatibility CI check (`wasm32-unknown-unknown`)
- `SECURITY.md`, `CODE_OF_CONDUCT.md`, issue templates, PR template
- `.rustfmt.toml`, `.clippy.toml`, `deny.toml` configuration files
- Edge case tests for `eval_rational()` with n=0 and n=1
- `cargo-semver-checks` and `cargo-deny` CI jobs

### Changed

- API declared stable under [Semantic Versioning](https://semver.org/)
- Contrast ratio documentation clarified as `[1.0, 21.0]` per WCAG standard
- `from_f32()` now documents NaN/infinity behavior
- `find_by_name()` and `find_palette_by_name()` now document case-sensitivity
  and original casing conventions
- `CONTRIBUTING.md` expanded with Code of Conduct, versioning policy, and
  contact information

### Summary

Prismatica 1.0.0 provides 308 scientific colormaps and 70 discrete palettes
from 10 established collections as compile-time Rust constants, with
bidirectional conversion for 19 framework integrations. The API is now
considered stable.

## [0.3.1] — 2026-03-28

### Changed

- Version patch bump.

## [0.3.0] — 2026-03-28

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

## [0.2.0] — 2025-05-01

### Added

- 308 scientific colormaps from 10 collections (CET, CMOcean, ColorBrewer,
  CMasher, NCAR, CartoColors, Moreland, d3 added to core matplotlib + Crameri)
- 70 discrete palettes (ColorBrewer, CartoColors, d3)
- Framework integrations: egui, plotters, image, serde
- Registry API: `find_by_name()`, `all_colormaps()`, `filter_by_kind()`, `filter_by_collection()`

## [0.1.0] — 2025-04-01

### Added

- Core types: `Color`, `Colormap`, `ColormapMeta`, `ColormapKind`, `DiscretePalette`
- matplotlib (8 maps) and Crameri (40 maps) collections
- `#![no_std]` support with optional `alloc`/`std` features

[1.0.0]: https://github.com/resonant-jovian/prismatica/compare/0.3.1...1.0.0
[0.3.1]: https://github.com/resonant-jovian/prismatica/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/resonant-jovian/prismatica/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/resonant-jovian/prismatica/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/resonant-jovian/prismatica/releases/tag/0.1.0
