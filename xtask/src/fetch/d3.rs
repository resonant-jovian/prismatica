use crate::codegen::{self, ColormapMeta};
use std::f64::consts::PI;
use std::path::Path;

const D3_CITATION: &str =
    "Bostock, M. d3-scale-chromatic. github.com/d3/d3-scale-chromatic";
const D3_AUTHOR: &str = "Mike Bostock / d3";

// ---------------------------------------------------------------------------
// Turbo (Google polynomial)
// ---------------------------------------------------------------------------

fn turbo_sample(t: f64) -> [u8; 3] {
    let r = 34.61 + t * (1172.33 - t * (10793.56 - t * (33300.12 - t * (38394.49 - t * 14825.05))));
    let g = 23.31 + t * (557.33 + t * (1225.33 - t * (3574.96 - t * (1073.77 + t * 707.56))));
    let b = 27.2 + t * (3211.1 - t * (15327.97 - t * (27814.0 - t * (22569.18 - t * 6838.66))));
    [
        r.round().clamp(0.0, 255.0) as u8,
        g.round().clamp(0.0, 255.0) as u8,
        b.round().clamp(0.0, 255.0) as u8,
    ]
}

fn generate_turbo() -> Vec<[u8; 3]> {
    (0..256).map(|i| turbo_sample(i as f64 / 255.0)).collect()
}

// ---------------------------------------------------------------------------
// Sinebow
// ---------------------------------------------------------------------------

fn sinebow_sample(t: f64) -> [u8; 3] {
    // d3 sinebow offsets t by 0.5 and uses (0.5 - t) before applying the formula
    let t = 0.5 - t;
    let r = (PI * (t + 0.0 / 3.0)).sin().powi(2);
    let g = (PI * (t + 1.0 / 3.0)).sin().powi(2);
    let b = (PI * (t + 2.0 / 3.0)).sin().powi(2);
    [
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    ]
}

fn generate_sinebow() -> Vec<[u8; 3]> {
    (0..256).map(|i| sinebow_sample(i as f64 / 255.0)).collect()
}

// ---------------------------------------------------------------------------
// Cubehelix helpers (d3 cubehelix color space)
// ---------------------------------------------------------------------------

/// Cubehelix coefficients from Green (2011), as used by d3-color.
const A: f64 = -0.14861;
const B: f64 = 1.78277;
const C: f64 = -0.29227;
const D: f64 = -0.90649;
const E: f64 = 1.97294;
const ED: f64 = 0.0; // E * D, but we use separate constants
const EB: f64 = 0.0; // E * B

/// Convert a cubehelix (h, s, l) to [u8; 3].
///
/// h is in degrees, s is saturation [0,1], l is lightness [0,1].
fn cubehelix_to_rgb(h: f64, s: f64, l: f64) -> [u8; 3] {
    let h_rad = (h + 120.0) * PI / 180.0;
    let amp = s * l * (1.0 - l);
    let cos_h = h_rad.cos();
    let sin_h = h_rad.sin();
    let r = (l + amp * (A * cos_h + B * sin_h)).clamp(0.0, 1.0);
    let g = (l + amp * (C * cos_h + D * sin_h)).clamp(0.0, 1.0);
    let b = (l + amp * (E * cos_h)).clamp(0.0, 1.0);
    [
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    ]
}

/// Interpolate along the cubehelix color space (long path) between two
/// (h, s, l) endpoints.  This matches d3's `interpolateCubehelixLong`.
fn cubehelix_long(
    h0: f64, s0: f64, l0: f64,
    h1: f64, s1: f64, l1: f64,
    t: f64,
) -> [u8; 3] {
    let h = h0 + (h1 - h0) * t;
    let s = s0 + (s1 - s0) * t;
    let l = l0 + (l1 - l0) * t;
    cubehelix_to_rgb(h, s, l)
}

// ---------------------------------------------------------------------------
// Warm: cubehelixLong from (260, 0.75, 0.35) to (80, 1.50, 0.8)
// ---------------------------------------------------------------------------

fn generate_warm() -> Vec<[u8; 3]> {
    (0..256)
        .map(|i| {
            let t = i as f64 / 255.0;
            cubehelix_long(260.0, 0.75, 0.35, 80.0, 1.50, 0.8, t)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Cool: cubehelixLong from (80, 1.50, 0.8) to (260, 0.75, 0.35)
// ---------------------------------------------------------------------------

fn generate_cool() -> Vec<[u8; 3]> {
    (0..256)
        .map(|i| {
            let t = i as f64 / 255.0;
            cubehelix_long(80.0, 1.50, 0.8, 260.0, 0.75, 0.35, t)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Rainbow: d3's interpolateRainbow
//
// d3 rainbow is defined as:
//   cubehelix(t) where:
//     hue = (360 * t) - 100
//     saturation = 1.5 - 1.5 * abs(t - 0.5) = 0.75 at edges, 1.5 at center
//     lightness = 0.8 - 0.9 * abs(t - 0.5) = 0.35 at edges, 0.8 at center
// This is d3's `d3.interpolateRainbow(t)` which maps warm(t in [0,0.5])
// then cool(t in [0.5,1]).
// ---------------------------------------------------------------------------

fn generate_rainbow() -> Vec<[u8; 3]> {
    // d3's rainbow concatenates warm(2t) for t in [0, 0.5]
    // and cool(2t - 1) for t in [0.5, 1].  Equivalently, it uses the formula:
    //   ts = abs(t - 0.5)
    //   hue = 360*t - 100
    //   sat = 1.5 - 1.5 * ts   (range: 0.75 .. 1.5 .. 0.75)
    //   lum = 0.8 - 0.9 * ts   (range: 0.35 .. 0.8 .. 0.35)
    (0..256)
        .map(|i| {
            let t = i as f64 / 255.0;
            let ts = (t - 0.5).abs();
            let h = 360.0 * t - 100.0;
            let s = 1.5 - 1.5 * ts;
            let l = 0.8 - 0.9 * ts;
            cubehelix_to_rgb(h, s, l)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// CubehelixDefault: d3's interpolateCubehelixDefault
//
// d3.interpolateCubehelixDefault = d3.interpolateCubehelixLong(
//   d3.cubehelix(300, 0.5, 0.0),
//   d3.cubehelix(-240, 0.5, 1.0)
// )
// This means: hue goes from 300 to -240 (i.e., 300 -> -240, a span of -540),
// saturation stays at 0.5, lightness goes from 0 to 1.
// ---------------------------------------------------------------------------

fn generate_cubehelix_default() -> Vec<[u8; 3]> {
    (0..256)
        .map(|i| {
            let t = i as f64 / 255.0;
            cubehelix_long(300.0, 0.5, 0.0, -240.0, 0.5, 1.0, t)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Tableau10 palette (discrete 10 colors)
// ---------------------------------------------------------------------------

const TABLEAU10_COLORS: [[u8; 3]; 10] = [
    [0x4e, 0x79, 0xa7], // #4e79a7
    [0xf2, 0x8e, 0x2c], // #f28e2c
    [0xe1, 0x57, 0x59], // #e15759
    [0x76, 0xb7, 0xb2], // #76b7b2
    [0x59, 0xa1, 0x4f], // #59a14f
    [0xed, 0xc9, 0x49], // #edc949
    [0xaf, 0x7a, 0xa1], // #af7aa1
    [0xff, 0x9d, 0xa7], // #ff9da7
    [0x9c, 0x75, 0x5f], // #9c755f
    [0xba, 0xb0, 0xab], // #bab0ab
];

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn fetch(project_root: &Path) {
    println!("Generating d3-scale-chromatic colormaps (computed locally)...");

    let data_dir = project_root.join("data").join("d3");
    std::fs::create_dir_all(&data_dir).expect("create data/d3/");

    // Continuous colormaps: (name, kind, perceptually_uniform, generator)
    let continuous_maps: Vec<(&str, &str, bool, Vec<[u8; 3]>)> = vec![
        ("Turbo", "Sequential", true, generate_turbo()),
        ("Sinebow", "Cyclic", false, generate_sinebow()),
        ("Rainbow", "Cyclic", false, generate_rainbow()),
        ("Warm", "Sequential", false, generate_warm()),
        ("Cool", "Sequential", false, generate_cool()),
        ("CubehelixDefault", "Sequential", false, generate_cubehelix_default()),
    ];

    for (name, kind, perceptually_uniform, lut) in &continuous_maps {
        assert_eq!(lut.len(), 256, "{name} LUT has {} entries, expected 256", lut.len());

        let meta = ColormapMeta {
            name: name.to_string(),
            collection: "d3".to_string(),
            author: D3_AUTHOR.to_string(),
            kind: kind.to_string(),
            perceptually_uniform: *perceptually_uniform,
            cvd_friendly: false,
            grayscale_safe: false,
            citation: D3_CITATION.to_string(),
        };

        codegen::write_csv(&data_dir.join(format!("{name}.csv")), lut);
        codegen::write_json(&data_dir.join(format!("{name}.json")), &meta);
        println!("  Wrote data/d3/{name}.csv + .json ({kind})");
    }

    // Tableau10: discrete palette + resampled 256-entry colormap
    {
        let colors = TABLEAU10_COLORS.to_vec();
        let resampled = codegen::resample(&colors, 256);

        let meta = ColormapMeta {
            name: "Tableau10".to_string(),
            collection: "d3".to_string(),
            author: D3_AUTHOR.to_string(),
            kind: "Qualitative".to_string(),
            perceptually_uniform: false,
            cvd_friendly: false,
            grayscale_safe: false,
            citation: D3_CITATION.to_string(),
        };

        codegen::write_csv(&data_dir.join("Tableau10.csv"), &resampled);
        codegen::write_csv(&data_dir.join("Tableau10_palette.csv"), &colors);
        codegen::write_json(&data_dir.join("Tableau10.json"), &meta);
        println!("  Wrote data/d3/Tableau10.csv + _palette.csv + .json (Qualitative, 10 colors)");
    }

    println!("  d3: 7 colormaps generated.");
}
