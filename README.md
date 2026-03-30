# prismatica

**The universal compile-time scientific colormap library for Rust**

[![Crates.io](https://img.shields.io/crates/v/prismatica.svg)](https://crates.io/crates/prismatica)
[![docs.rs](https://docs.rs/prismatica/badge.svg)](https://docs.rs/prismatica)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Support on thanks.dev](https://img.shields.io/badge/Support-thanks.dev-green)](https://thanks.dev/u/gh/resonant-jovian)

[![CI](https://github.com/resonant-jovian/prismatica/actions/workflows/ci.yml/badge.svg)](https://github.com/resonant-jovian/prismatica/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.85-blue.svg)]()

> [!IMPORTANT]
> Pre-1.0.0 — the API may change between minor versions. The API will be considered stable at 1.0.0.

---

## Status

| Component | Status | Count |
|---|---|---|
| Core types (`Color`, `Colormap`, `ColormapMeta`, `DiscretePalette`) | Done | |
| Registry (discovery, filtering, palette lookup) | Done | |
| Prelude module | Done | |
| matplotlib | Done | 8 maps |
| Crameri | Done | 40 maps |
| CET | Done | 59 maps |
| CMOcean | Done | 22 maps |
| Moreland | Done | 6 maps |
| CMasher | Done | 53 maps |
| ColorBrewer | Done | 35 maps + 35 palettes |
| CartoColors | Done | 34 maps + 34 palettes |
| NCAR NCL | Done | 44 maps |
| d3 | Done | 7 maps + 1 palette |
| Framework integrations (19 crates) | Done | |
| **Total** | | **308 colormaps + 70 palettes** |

---

## Highlights

- **308 scientific colormaps** from 10 established collections
- **70 discrete palettes** for categorical data (ColorBrewer, CartoColors, d3)
- **Compile-time constants** -- zero runtime file I/O, zero parsing
- **`#![no_std]`** -- core functionality works without allocation
- **Continuous sampling** -- linear interpolation between 256-step LUT entries for any `t` in `[0, 1]`
- **Rich metadata** -- every colormap carries its kind, perceptual uniformity flag, CVD safety, and citation
- **Feature-gated collections** -- include only the maps you need; ~1 KB per colormap
- **Framework integrations** -- bidirectional conversion for 19 crates including plotters, egui, image, bevy, ratatui, iced, palette, and more
- **Sibling project** -- [chromata](https://github.com/resonant-jovian/chromata) provides editor color themes with the same Color type and integration pattern

---

## For Everyone

### What are scientific colormaps?

A colormap is a function that converts a scalar value (like temperature, elevation, or density) into a color. When you see a weather map where blue means cold and red means hot, that is a colormap at work.

**Perceptual uniformity** means that equal steps in data produce equal steps in perceived color difference. Without it, your visualization can create false features -- bright bands or phantom boundaries that exist in the colormap but not in the data. The classic "rainbow" and "jet" colormaps are notorious for this.

Prismatica provides every established scientific colormap collection as compile-time Rust data. A researcher writes `BATLOW.eval(0.5)` and gets a perceptually uniform, colorblind-safe color. No files to load, no dependencies to install.

### Quick start

Add prismatica to your project:

```toml
[dependencies]
prismatica = "0.3.1"
```

Use a colormap:

```rust
use prismatica::crameri::BATLOW;

let color = BATLOW.eval(0.5);
println!("RGB: ({}, {}, {})", color.r, color.g, color.b);
```

### Supported collections

| Collection | Author / Organization | Maps | Palettes | Type |
|---|---|---|---|---|
| Matplotlib | van der Walt, Smith, Firing | 8 | | Sequential, perceptually uniform |
| Crameri | Fabio Crameri | 40 | | Sequential, diverging, multi-sequential, cyclic |
| CET | Peter Kovesi | 59 | | Sequential, diverging, cyclic, rainbow, isoluminant |
| CMOcean | Kristen Thyng | 22 | | Oceanographic sequential, diverging |
| ColorBrewer | Cynthia Brewer | 35 | 35 | Sequential, diverging, qualitative |
| CMasher | Ellert van der Velden | 53 | | Sequential, diverging (astrophysics) |
| NCAR NCL | NCAR | 44 | | Geoscience sequential, diverging |
| CartoColors | CARTO | 34 | 34 | Cartographic sequential, diverging, qualitative |
| Moreland | Kenneth Moreland | 6 | | Cool-warm diverging, black body, Kindlmann |
| d3 | Mike Bostock | 7 | 1 | Turbo, Rainbow, Sinebow, Cubehelix, Tableau10 |
| **Total** | | **308** | **70** | |

### Choosing the right colormap

| Data type | Recommended maps | Why |
|---|---|---|
| **Sequential** (temperature, elevation) | `batlow`, `viridis`, `oslo`, `thermal` | Monotonic luminance, perceptually uniform |
| **Diverging** (anomalies, residuals) | `berlin`, `vik`, `balance`, `smooth-cool-warm` | Neutral center, symmetric extremes |
| **Cyclic** (phase, direction, time-of-day) | `romaO`, `phase`, `twilight` | End color equals start color |
| **Categorical** (labels, classes) | `SET2`, `DARK2`, `PAIRED`, `Tableau10` | Maximally distinct, non-interpolated |

### What prismatica is not

- Not a color manipulation library (use [`palette`](https://crates.io/crates/palette) for that)
- Not a gradient builder (use [`colorgrad`](https://crates.io/crates/colorgrad) for custom gradients)
- Not a rendering engine
- Not a data visualization framework

Prismatica is the data layer. It answers one question: *given a colormap name and a scalar `t` in `[0, 1]`, what RGB color should I use?*

---

## For Researchers

### The Color type

```rust
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
```

Methods: `new(r, g, b)`, `from_hex(0xFF8800)`, `from_css_hex("#ff8800")`, `from_f32(r, g, b)`, `to_hex()`, `to_css_hex()`, `to_f32()`, `lerp(other, t)`, `luminance()`, `contrast_ratio(other)`. Implements `Display` (CSS hex), `Default` (black), `From<[u8; 3]>`, and `From<(u8, u8, u8)>`.

### Colormap sampling

```rust
use prismatica::crameri::BATLOW;

// Continuous: sample at any float in [0, 1]
let color = BATLOW.eval(0.5);

// Rational: the 30th of 100 evenly-spaced samples
let color = BATLOW.eval_rational(30, 100);

// Reversed direction (zero allocation)
let rev = BATLOW.reversed();
let color = rev.eval(0.2); // equivalent to BATLOW.eval(0.8)

// Extract N discrete colors
let legend_colors = BATLOW.colors(10);
```

Values outside `[0, 1]` are clamped. Interpolation is linear in sRGB space, matching matplotlib, ParaView, and most scientific tools.

### Colormap discovery

```rust
use prismatica::{ColormapKind, all_colormaps, find_by_name, filter_by_collection};

// Find all perceptually uniform diverging colormaps
let diverging: Vec<_> = all_colormaps()
    .iter()
    .filter(|cm| {
        cm.meta.kind == ColormapKind::Diverging
            && cm.meta.perceptually_uniform
    })
    .collect();

// Look up by name
let viridis = find_by_name("viridis").expect("viridis should exist");

// Filter by collection
let crameri_maps = filter_by_collection("crameri");
```

### Discrete palettes

ColorBrewer, CartoColors, and d3 provide discrete palettes alongside continuous colormaps:

```rust
use prismatica::colorbrewer::SET2_PALETTE;

for i in 0..SET2_PALETTE.len() {
    let c = SET2_PALETTE.get(i);
    println!("Category {}: #{:02x}{:02x}{:02x}", i, c.r, c.g, c.b);
}

// Also available via the registry
let palette = prismatica::find_palette_by_name("Blues").expect("Blues should exist");
```

### Feature flags

| Feature | Maps | Description |
|---|---|---|
| `core` (default) | 48 | matplotlib + Crameri -- the maps journals recommend |
| `matplotlib` | 8 | viridis, inferno, magma, plasma, cividis, twilight, mako, rocket |
| `crameri` | 40 | batlow, berlin, roma, oslo, tokyo, hawaii, and more |
| `cet` | 59 | CET-L*, CET-D*, CET-C*, CET-R* perceptually uniform maps |
| `cmocean` | 22 | thermal, haline, solar, ice, deep, and 17 more |
| `colorbrewer` | 35 + 35 palettes | Blues, RdBu, Set2, Spectral, and more |
| `cmasher` | 53 | ember, ocean, gothic, fusion, wildfire, and more |
| `ncar` | 44 | NCAR NCL geoscience colour tables |
| `cartocolors` | 34 + 34 palettes | CARTO cartographic colour schemes |
| `moreland` | 6 | cool-warm, black body, Kindlmann, extended variants |
| `d3` | 7 + 1 palette | Turbo, Rainbow, Sinebow, Cubehelix, Tableau10 |
| `all` | 308 + 70 palettes | All collections |

### Framework integrations

All integrations provide bidirectional conversion via `From`/`Into`. Enum-based types use `TryFrom` for the reverse direction. Enable `all-integrations` to get everything.

| Feature flag | Framework | Forward | Reverse |
|---|---|---|---|
| `egui-integration` | egui | `Color` -> `Color32` | `Color32` -> `Color` |
| `plotters-integration` | plotters | `Color` -> `RGBColor` | `RGBColor` -> `Color` |
| `image-integration` | image | `Color` -> `Rgb<u8>` | `Rgb<u8>` -> `Color` |
| `palette-integration` | palette | `Color` -> `Srgb<u8>` | `Srgb<u8>` -> `Color` |
| `bevy-color-integration` | bevy_color | `Color` -> `Srgba` | `Srgba` -> `Color` |
| `iced-integration` | iced | `Color` -> `iced::Color` | `iced::Color` -> `Color` |
| `macroquad-integration` | macroquad | `Color` -> `mq::Color` | `mq::Color` -> `Color` |
| `tiny-skia-integration` | tiny-skia | `Color` -> `ts::Color` | `ts::Color` -> `Color` |
| `wgpu-integration` | wgpu | `Color` -> `wgpu::Color` | `wgpu::Color` -> `Color` |
| `slint-integration` | slint | `Color` -> `slint::Color` | `slint::Color` -> `Color` |
| `ratatui-integration` | ratatui | `Color` -> `Color::Rgb` | `TryFrom` |
| `crossterm-integration` | crossterm | `Color` -> `Color::Rgb` | `TryFrom` |
| `colored-integration` | colored | `Color` -> `TrueColor` | `TryFrom` |
| `owo-colors-integration` | owo-colors | `Color` -> `Rgb` | `Rgb` -> `Color` |
| `termion-integration` | termion | `Color` -> `Rgb` | `Rgb` -> `Color` |
| `cursive-integration` | cursive | `Color` -> `Color::Rgb` | `TryFrom` |
| `comfy-table-integration` | comfy-table | `Color` -> `Color::Rgb` | `TryFrom` |
| `syntect-integration` | syntect | `Color` -> `hl::Color` | `hl::Color` -> `Color` |
| `serde-support` | serde | Serialize/Deserialize for meta types | |
| `all-integrations` | all of the above | | |

### Binary size

Each colormap is a 256x3 = 768-byte LUT plus ~200 bytes of metadata. Approximately **1 KB per colormap**.

| Feature | Maps | Size |
|---|---|---|
| `core` (default) | 48 | ~48 KB |
| `all` | 308 | ~308 KB |

Even with all 308 colormaps enabled, the total is smaller than a single PNG image.

### Metadata

Every colormap carries structured metadata for programmatic filtering:

```rust
pub struct ColormapMeta {
    pub name: &'static str,              // "batlow"
    pub collection: &'static str,        // "crameri"
    pub author: &'static str,            // "Fabio Crameri"
    pub kind: ColormapKind,              // Sequential, Diverging, Cyclic, ...
    pub perceptually_uniform: bool,      // true
    pub cvd_friendly: bool,              // true (colorblind safe)
    pub grayscale_safe: bool,            // true
    pub lut_size: usize,                 // 256
    pub citation: &'static str,          // DOI / reference string
}
```

### Citation

If you use prismatica in academic work, please cite the upstream colormap authors. Each colormap's `meta.citation` field contains the appropriate reference. Key citations:

- **Crameri:** Crameri, F. (2018). Scientific colour maps. Zenodo. doi:10.5281/zenodo.1243862
- **CET:** Kovesi, P. (2015). Good Colour Maps: How to Design Them. arXiv:1509.03700
- **CMOcean:** Thyng, K. M. et al. (2016). True colors of oceanography. Oceanography, 29(3), 10.
- **CMasher:** van der Velden, E. (2020). CMasher: Scientific colormaps for making accessible, informative and 'cmashing' plots. JOSS, 5(46), 2004.

---

## For Developers

### Architecture

Prismatica's core data model is simple:

1. A **lookup table (LUT)** of 256 evenly-spaced RGB values, stored as `static [[u8; 3]; 256]`
2. A **metadata struct** describing the colormap's properties
3. A **sampling function** that interpolates between LUT entries for any input `t` in `[0, 1]`

All colormap data is compiled into the binary as `const`/`static` arrays. There is no runtime I/O, no parsing, and no allocation for basic operations.

### Module structure

```
src/
├── lib.rs              # Crate-level docs, re-exports, feature gates
├── types.rs            # Color, Colormap, ColormapMeta, ColormapKind, DiscretePalette
├── traits.rs           # Framework conversion traits (feature-gated)
├── registry.rs         # all_colormaps(), find_by_name(), filter functions, palette functions
├── prelude.rs          # Convenience re-exports
├── integration/        # Feature-gated framework conversions (19 crates)
│
├── matplotlib/         # Feature: "matplotlib" — 8 maps
├── crameri/            # Feature: "crameri" — 40 maps
├── cet/                # Feature: "cet" — 59 maps
├── cmocean/            # Feature: "cmocean" — 22 maps
├── colorbrewer/        # Feature: "colorbrewer" — 35 maps + 35 palettes
├── cmasher/            # Feature: "cmasher" — 53 maps
├── ncar/               # Feature: "ncar" — 44 maps
├── cartocolors/        # Feature: "cartocolors" — 34 maps + 34 palettes
├── moreland/           # Feature: "moreland" — 6 maps
└── d3/                 # Feature: "d3" — 7 maps + 1 palette
```

### Code generation pipeline

Colormaps are not written by hand. A two-stage pipeline generates all colormap source code:

1. **Fetch** (`cargo xtask fetch [collection]`): Downloads upstream data and normalizes to `data/{collection}/{name}.csv` (256 rows of `R,G,B` as uint8) plus `data/{collection}/{name}.json` (metadata).

2. **Generate** (`cargo xtask generate [collection]`): Reads normalized CSV + JSON and emits Rust source files with `const` colormap definitions and `static` LUT arrays.

```
Upstream data -> cargo xtask fetch -> data/*.csv + data/*.json -> cargo xtask generate -> src/{collection}/*.rs
```

### Data sources

| Collection | Source | Format | License |
|---|---|---|---|
| Crameri | [Zenodo](https://zenodo.org/records/8409685) | 256x3 CSV (floats 0-1) | MIT |
| CET | [colorcet.com](https://colorcet.com/) | CSV (floats 0-1) | CC-BY |
| CMOcean | [GitHub](https://github.com/matplotlib/cmocean) | Space-separated float .txt | MIT |
| ColorBrewer | [colorbrewer2.org](https://colorbrewer2.org/) | JSON (rgb() CSS strings) | Apache-2.0 |
| Matplotlib | [GitHub](https://github.com/matplotlib/matplotlib) | Python 256x3 float arrays | CC0 |
| CMasher | [GitHub](https://github.com/1313e/CMasher) | `.txt` LUT files (int/float) | BSD-3 |
| Moreland | [kennethmoreland.com](https://www.kennethmoreland.com/color-advice/) | CSV (int 0-255) | Public domain / BSD |
| NCAR NCL | [GitHub](https://github.com/NCAR/ncl) | `.rgb` integer tables | Apache-2.0 |
| CartoColors | [GitHub](https://github.com/CartoDB/CartoColor) | TypeScript hex arrays | CC-BY-3.0 |
| d3 | computed locally | Cubehelix/sinebow math | ISC |

### Adding new colormaps

To add a new collection:

1. Add a fetch module in `xtask/src/fetch/{collection}.rs`
2. Register it in `xtask/src/fetch/mod.rs`
3. Run `cargo xtask fetch {collection}` to populate `data/{collection}/`
4. Add generate module (or use the generic path) in `xtask/src/generate/`
5. Run `cargo xtask generate {collection}` to emit `src/{collection}/*.rs`
6. Add a feature flag in `Cargo.toml` and add to the `all` feature
7. Add `#[cfg(feature = "...")]` module declaration in `lib.rs`
8. Add the collection to `for_each_colormap()` in `registry.rs`

### Testing

```bash
cargo test                                         # All tests (unit + integration)
cargo test --all-features                          # With all collections enabled
cargo check --no-default-features --features core  # Verify no_std compatibility
cargo clippy -- -W clippy::all                     # Lint
cargo test --test property --features all          # Property-based tests (proptest)
cargo test --test snapshots --features all         # Snapshot tests (insta)
```

Snapshot tests use [insta](https://insta.rs/) to detect codegen changes. After modifying the code generation pipeline, run `cargo insta review` to inspect and accept updated snapshots.

### Code quality

```rust
#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
```

---

## Competitive positioning

| Crate | Colormaps | LUT-based | Metadata | Collections | `no_std` |
|---|---|---|---|---|---|
| colorous | ~40 | Partial | None | d3 only | Yes |
| colorgrad | ~40 | No (interpolated) | None | d3 + custom | No |
| scarlet | ~5 | No (computed) | None | Basic only | No |
| **prismatica** | **308** | **Yes (256-step const)** | **Full** | **10 collections** | **Yes** |

---

## Minimum supported Rust version

Rust edition 2024, targeting **stable Rust 1.85+**.

## Support

If prismatica is useful to your projects, consider supporting development via [thanks.dev](https://thanks.dev/u/gh/resonant-jovian).

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).

Prismatica bundles colormap data from multiple upstream sources, each with its own license. All upstream licenses are permissive (MIT, Apache-2.0, CC-BY, CC0, BSD-3, public domain) and compatible with GPL-3.0. See individual collection module docs for source URLs and license details.
