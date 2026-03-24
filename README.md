# prismatica

**The universal compile-time scientific colormap library for Rust**

[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.0.1-orange.svg)]()

---

## Status

| Component | Status |
|---|---|
| Core types (`Color`, `Colormap`, `ColormapMeta`) | Stubs |
| Registry (discovery, filtering) | Stubs |
| matplotlib (8 maps) | Planned |
| Crameri (35+ maps) | Planned |
| CET (60+ maps) | Planned |
| CMOcean (22 maps) | Planned |
| ColorBrewer (35 palettes) | Planned |
| CMasher (30+ maps) | Planned |
| NCAR NCL (40+ maps) | Planned |
| CartoColors (15+ maps) | Planned |
| Moreland (6 maps) | Planned |
| d3 (d3-scale-chromatic) | Planned |
| Framework integrations | Planned |

---

## Highlights

- **260+ scientific colormaps** from 10+ established collections
- **Compile-time constants** -- zero runtime file I/O, zero parsing
- **Zero dependencies** -- the core crate has no external dependencies
- **Continuous sampling** -- linear interpolation between 256-step LUT entries for any `t` in `[0, 1]`
- **Rich metadata** -- every colormap carries its kind, perceptual uniformity flag, CVD safety, and citation
- **Feature-gated collections** -- include only the maps you need; ~1 KB per colormap
- **Framework integrations** -- optional support for plotters, egui, image, and serde
- **Broad coverage** -- matplotlib, Crameri, CET, CMOcean, ColorBrewer, CMasher, NCAR NCL, CartoColors, Moreland, d3

---

## For Everyone **[Planned]**

### What are scientific colormaps?

A colormap is a function that converts a scalar value (like temperature, elevation, or density) into a color. When you see a weather map where blue means cold and red means hot, that is a colormap at work.

**Perceptual uniformity** means that equal steps in data produce equal steps in perceived color difference. Without it, your visualization can create false features -- bright bands or phantom boundaries that exist in the colormap but not in the data. The classic "rainbow" and "jet" colormaps are notorious for this.

Prismatica provides every established scientific colormap collection as compile-time Rust data. A researcher writes `BATLOW.eval(0.5)` and gets a perceptually uniform, colorblind-safe color. No files to load, no dependencies to install.

### Quick start

Add prismatica to your project:

```toml
[dependencies]
prismatica = "0.0.1"
```

Use a colormap:

```rust
use prismatica::crameri::BATLOW;

let color = BATLOW.eval(0.5);
println!("RGB: ({}, {}, {})", color.r, color.g, color.b);
```

### Supported collections

| Collection | Author / Organization | Maps | Type |
|---|---|---|---|
| Matplotlib | van der Walt, Smith, Firing | 8 | Sequential, perceptually uniform |
| Crameri | Fabio Crameri | 35+ | Sequential, diverging, multi-sequential, cyclic |
| CET | Peter Kovesi | 60+ | Sequential, diverging, cyclic, rainbow, isoluminant |
| CMOcean | Kristen Thyng | 22 | Oceanographic sequential, diverging |
| ColorBrewer | Cynthia Brewer | 35 | Sequential, diverging, qualitative (discrete) |
| CMasher | Ellert van der Velden | 30+ | Sequential, diverging (astrophysics) |
| NCAR NCL | NCAR | 40+ | Geoscience sequential, diverging |
| CartoColors | CARTO | 15+ | Cartographic sequential, diverging, qualitative |
| Moreland | Kenneth Moreland | 6 | Cool-warm diverging, black body, Kindlmann |
| d3 | Mike Bostock | varies | d3-scale-chromatic |
| **Total** | | **~260+** | |

### Choosing the right colormap

| Data type | Recommended maps | Why |
|---|---|---|
| **Sequential** (temperature, elevation) | `batlow`, `viridis`, `oslo`, `thermal` | Monotonic luminance, perceptually uniform |
| **Diverging** (anomalies, residuals) | `berlin`, `vik`, `balance`, `cool_warm` | Neutral center, symmetric extremes |
| **Cyclic** (phase, direction, time-of-day) | `romaO`, `phase`, `twilight` | End color equals start color |
| **Categorical** (labels, classes) | `SET2`, `DARK2`, `PAIRED` | Maximally distinct, non-interpolated |

### What prismatica is not

- Not a color manipulation library (use [`palette`](https://crates.io/crates/palette) for that)
- Not a gradient builder (use [`colorgrad`](https://crates.io/crates/colorgrad) for custom gradients)
- Not a rendering engine
- Not a data visualization framework

Prismatica is the data layer. It answers one question: *given a colormap name and a scalar `t` in `[0, 1]`, what RGB color should I use?*

---

## For Researchers **[Planned]**

### The Color type

```rust
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
```

Methods: `new(r, g, b)`, `from_hex(0xFF8800)`, `to_css_hex()`, `to_f32()`, `lerp(other, t)`.

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
use prismatica::{ColormapKind, all_colormaps};

// Find all perceptually uniform diverging colormaps
let diverging: Vec<_> = all_colormaps()
    .iter()
    .filter(|cm| {
        cm.meta.kind == ColormapKind::Diverging
            && cm.meta.perceptually_uniform
    })
    .collect();

// Look up by name
let viridis = prismatica::find_by_name("viridis").unwrap();

// Filter by collection
let crameri_maps = prismatica::filter_by_collection("crameri");
```

### Discrete palettes

ColorBrewer qualitative palettes provide distinct, non-interpolated colors for categorical data:

```rust
use prismatica::colorbrewer::qualitative::SET2;

for i in 0..SET2.len() {
    let c = SET2.get(i);
    println!("Category {}: {}", i, c.to_css_hex());
}
```

### Feature flags

| Feature | Maps | Description |
|---|---|---|
| `core` (default) | ~43 | matplotlib + Crameri -- the maps journals recommend |
| `matplotlib` | 8 | viridis, inferno, magma, plasma, cividis, twilight, mako, rocket |
| `crameri` | 35+ | batlow, berlin, roma, oslo, tokyo, hawaii, and more |
| `cet` | 60+ | CET-L*, CET-D*, CET-C*, CET-R* perceptually uniform maps |
| `cmocean` | 22 | thermal, haline, solar, ice, deep, and 17 more |
| `colorbrewer` | 35 | Blues, RdBu, Set2, Spectral, and more |
| `cmasher` | 30+ | ember, ocean, gothic, neon, and more |
| `ncar` | 40+ | NCAR NCL geoscience colour tables |
| `cartocolors` | 15+ | CARTO cartographic colour schemes |
| `moreland` | 6 | cool-warm, black body, Kindlmann, extended variants |
| `d3` | varies | d3-scale-chromatic maps |
| `all` | ~260+ | All collections |

### Framework integrations **[Planned]**

| Feature flag | Framework | Conversion |
|---|---|---|
| `plotters-integration` | plotters | `Color` to `RGBColor`, palette closures |
| `egui-integration` | egui | `Color` to `Color32` |
| `image-integration` | image | `Color` to `Rgb<u8>` |
| `serde-support` | serde | Serialization for `ColormapMeta` |

### Binary size

Each colormap is a 256x3 = 768-byte LUT plus ~200 bytes of metadata. Approximately **1 KB per colormap**.

| Feature | Maps | Size |
|---|---|---|
| `core` (default) | ~43 | ~43 KB |
| `all` | ~260 | ~260 KB |

Even with all 260+ colormaps enabled, the total is smaller than a single PNG image.

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

## For Developers **[Stubs Available]**

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
├── registry.rs         # all_colormaps(), find_by_name(), filter functions
│
├── matplotlib/         # Feature: "matplotlib" — 8 maps
│   └── mod.rs
├── crameri/            # Feature: "crameri" — 35+ maps
│   └── mod.rs
├── cet/                # Feature: "cet" — 60+ maps
│   └── mod.rs
├── cmocean/            # Feature: "cmocean" — 22 maps
│   └── mod.rs
├── colorbrewer/        # Feature: "colorbrewer" — 35 palettes
│   └── mod.rs
├── cmasher/            # Feature: "cmasher" — 30+ maps
│   └── mod.rs
├── ncar/               # Feature: "ncar" — 40+ maps
│   └── mod.rs
├── cartocolors/        # Feature: "cartocolors" — 15+ maps
│   └── mod.rs
├── moreland/           # Feature: "moreland" — 6 maps
│   └── mod.rs
└── d3/                 # Feature: "d3"
    └── mod.rs
```

Each collection module will contain one `.rs` file per colormap, auto-generated from upstream LUT data.

### Code generation pipeline

Colormaps are not written by hand. A two-stage pipeline generates all colormap source code:

1. **Extraction** (Python): Downloads upstream data (CSV, JSON, `.m` files, Python packages) and normalizes to `data/{collection}/{name}.csv` (256 rows of `R,G,B` as uint8) plus `data/{collection}/{name}.json` (metadata).

2. **Generation** (Python): Reads normalized CSV + JSON and emits Rust source files with `const` colormap definitions and `static` LUT arrays.

```
Upstream data → extraction script → data/*.csv + data/*.json → generator → src/{collection}/*.rs
```

### Data sources

| Collection | Source | Format | License |
|---|---|---|---|
| Crameri | [Zenodo](https://zenodo.org/records/8409685) | 256x3 CSV (floats 0-1) | MIT |
| CET | [colorcet.com](https://colorcet.com/) | MATLAB `.m` or CSV | CC-BY |
| CMOcean | [GitHub](https://github.com/matplotlib/cmocean) | Python/NumPy arrays | MIT |
| ColorBrewer | [colorbrewer2.org](https://colorbrewer2.org/) | JSON (3-12 discrete steps) | Apache-2.0 |
| Matplotlib | [BIDS/colormap](https://github.com/BIDS/colormap) | Python 256x3 float arrays | CC0 |
| CMasher | [GitHub](https://github.com/1313e/CMasher) | `.txt` LUT files | BSD-3 |
| Moreland | [kennethmoreland.com](https://www.kennethmoreland.com/color-advice/) | CSV | Public domain / BSD |
| NCAR NCL | [ncl.ucar.edu](https://www.ncl.ucar.edu/Document/Graphics/color_table_gallery.shtml) | `.rgb` integer tables | Apache-2.0 |
| CartoColors | [CARTO](https://carto.com/carto-colors/) | JSON | CC-BY-3.0 |

### Testing strategy

- **LUT integrity:** every LUT has exactly 256 entries with valid RGB values
- **Perceptual monotonicity:** sequential + perceptually uniform maps have monotonic luminance
- **Boundary clamping:** `eval(-1.0)` == `eval(0.0)`, `eval(2.0)` == `eval(1.0)`
- **Reference values:** spot-checks against upstream data (e.g., `viridis(0.0)` == `(68, 1, 84)`)
- **No duplicate names:** every colormap has a unique canonical name
- **Snapshot tests:** catalog snapshot via `insta` to catch regressions

### Code quality

```rust
#![forbid(warnings)]
#![deny(clippy::unwrap_used)]
```

### Adding new colormaps

To add a new collection:

1. Write an extraction function in the Python extraction script that normalizes upstream data to 256x3 uint8 CSV + metadata JSON
2. Run the extraction script to populate `data/{collection}/`
3. Run the generator script to emit `src/{collection}/*.rs`
4. Add a feature flag in `Cargo.toml`
5. Add the `#[cfg(feature = "...")]` module declaration in `lib.rs`
6. Add the collection to the `all` feature
7. Update the registry in `registry.rs`

### Companion projects

- [**caustic**](https://github.com/AlbinSidas/caustic) -- A 6D Vlasov-Poisson solver framework for collisionless gravitational dynamics
- [**phasma**](https://github.com/AlbinSidas/phasma) -- Terminal interface for the caustic Vlasov-Poisson solver

---

## Competitive positioning

| Crate | Colormaps | LUT-based | Metadata | Collections | no_std |
|---|---|---|---|---|---|
| colorous | ~40 | Partial | None | d3 only | Yes |
| colorgrad | ~40 | No (interpolated) | None | d3 + custom | No |
| scarlet | ~5 | No (computed) | None | Basic only | No |
| **prismatica** | **260+** | **Yes (256-step const)** | **Full** | **10+ collections** | **Planned** |

---

## Roadmap

| Version | Milestone | Maps |
|---|---|---|
| **v0.1** | Core types + matplotlib + Crameri | ~43 |
| **v0.2** | CET + CMOcean + ColorBrewer | ~160 |
| **v0.3** | CMasher + NCAR + CartoColors + Moreland | ~260+ |
| **v0.4** | Framework integrations, serde, gallery generator | ~260+ |
| **v1.0** | Stable API, upstream sync CI, WASM, benchmarks | ~260+ |

---

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).

Prismatica bundles colormap data from multiple upstream sources, each with its own license. All upstream licenses are permissive (MIT, Apache-2.0, CC-BY, CC0, BSD-3, public domain) and compatible with GPL-3.0. See individual collection module docs for source URLs and license details.
