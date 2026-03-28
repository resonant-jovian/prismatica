use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Shared types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColormapMeta {
    pub name: String,
    pub collection: String,
    pub author: String,
    pub kind: String,
    pub perceptually_uniform: bool,
    pub cvd_friendly: bool,
    pub grayscale_safe: bool,
    pub citation: String,
}

/// A fully loaded colormap: 256 RGB triplets + metadata.
pub struct ColormapData {
    pub meta: ColormapMeta,
    pub lut: Vec<[u8; 3]>,
}

/// A discrete palette: variable-length RGB colors + metadata.
#[allow(dead_code)]
pub struct PaletteData {
    pub meta: ColormapMeta,
    pub colors: Vec<[u8; 3]>,
}

// ---------------------------------------------------------------------------
// Project layout
// ---------------------------------------------------------------------------

pub fn project_root() -> PathBuf {
    let xtask_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    xtask_dir
        .parent()
        .expect("xtask dir must have a parent")
        .to_path_buf()
}

// ---------------------------------------------------------------------------
// HTTP helpers
// ---------------------------------------------------------------------------

pub fn fetch_url_text(url: &str) -> String {
    ureq::get(url)
        .call()
        .unwrap_or_else(|e| panic!("GET {url}: {e}"))
        .body_mut()
        .read_to_string()
        .unwrap_or_else(|e| panic!("read body from {url}: {e}"))
}

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

pub fn float_to_u8(r: f64, g: f64, b: f64) -> [u8; 3] {
    [
        (r.clamp(0.0, 1.0) * 255.0).round() as u8,
        (g.clamp(0.0, 1.0) * 255.0).round() as u8,
        (b.clamp(0.0, 1.0) * 255.0).round() as u8,
    ]
}

/// Linearly resample a LUT to `n` entries.
pub fn resample(lut: &[[u8; 3]], n: usize) -> Vec<[u8; 3]> {
    if n <= 1 {
        return lut[..n.min(lut.len())].to_vec();
    }
    let src_len = lut.len() as f64;
    (0..n)
        .map(|i| {
            let t = i as f64 / (n - 1) as f64;
            let pos = t * (src_len - 1.0);
            let lo = pos.floor() as usize;
            let hi = (lo + 1).min(lut.len() - 1);
            let frac = pos - lo as f64;
            [
                ((1.0 - frac) * lut[lo][0] as f64 + frac * lut[hi][0] as f64).round() as u8,
                ((1.0 - frac) * lut[lo][1] as f64 + frac * lut[hi][1] as f64).round() as u8,
                ((1.0 - frac) * lut[lo][2] as f64 + frac * lut[hi][2] as f64).round() as u8,
            ]
        })
        .collect()
}

/// Parse space-separated float triples (Crameri .txt, CMOcean .txt format).
pub fn parse_space_separated_floats(text: &str) -> Vec<[u8; 3]> {
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 3 {
                return None;
            }
            let r: f64 = parts[0].parse().ok()?;
            let g: f64 = parts[1].parse().ok()?;
            let b: f64 = parts[2].parse().ok()?;
            Some(float_to_u8(r, g, b))
        })
        .collect()
}

/// Parse space-separated u8 integer triples (CMasher _8bit.txt format).
pub fn parse_space_separated_u8(text: &str) -> Vec<[u8; 3]> {
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                return None;
            }
            let r: u8 = parts[0].parse().ok()?;
            let g: u8 = parts[1].parse().ok()?;
            let b: u8 = parts[2].parse().ok()?;
            Some([r, g, b])
        })
        .collect()
}

// ---------------------------------------------------------------------------
// CSV / JSON I/O
// ---------------------------------------------------------------------------

pub fn write_csv(path: &Path, lut: &[[u8; 3]]) {
    let mut out = String::new();
    for rgb in lut {
        out.push_str(&format!("{},{},{}\n", rgb[0], rgb[1], rgb[2]));
    }
    fs::write(path, out).expect("write CSV");
}

pub fn write_json(path: &Path, meta: &ColormapMeta) {
    let json = serde_json::to_string_pretty(meta).expect("serialize JSON");
    fs::write(path, json).expect("write JSON");
}

/// Write file content only if it differs from existing content.
#[allow(dead_code)]
pub fn write_if_changed(path: &Path, content: &str) -> std::io::Result<bool> {
    if let Ok(existing) = fs::read_to_string(path)
        && existing == content
    {
        return Ok(false);
    }
    fs::write(path, content)?;
    Ok(true)
}

// ---------------------------------------------------------------------------
// Code generation
// ---------------------------------------------------------------------------

/// Read `//!` doc comment lines from an existing mod.rs file.
pub fn read_doc_comments(path: &Path) -> String {
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return String::new(),
    };

    let mut doc = String::new();
    for line in contents.lines() {
        if line.starts_with("//!") {
            doc.push_str(line);
            doc.push('\n');
        }
    }
    doc
}

/// Generate a single colormap .rs file content.
pub fn generate_colormap_rs(map: &ColormapData, const_name: &str) -> String {
    let meta = &map.meta;
    let mut out = String::new();

    out.push_str("// Auto-generated by prismatica xtask -- do not edit\n");
    out.push_str("use crate::{Colormap, ColormapKind, ColormapMeta};\n\n");

    // Doc comment
    out.push_str(&format!(
        "/// {} -- {} colormap from the {} collection.\n",
        meta.name, meta.kind, meta.collection
    ));
    out.push_str(&format!("///\n/// Author: {}\n", meta.author));
    out.push_str(&format!(
        "/// Perceptually uniform: {}\n",
        meta.perceptually_uniform
    ));
    out.push_str(&format!("/// CVD friendly: {}\n", meta.cvd_friendly));
    out.push_str(&format!("///\n/// Citation: {}\n", meta.citation));

    // Const declaration
    out.push_str(&format!("pub const {const_name}: Colormap = Colormap {{\n"));
    out.push_str("    meta: ColormapMeta {\n");
    out.push_str(&format!("        name: \"{}\",\n", meta.name));
    out.push_str(&format!("        collection: \"{}\",\n", meta.collection));
    out.push_str(&format!("        author: \"{}\",\n", meta.author));
    out.push_str(&format!("        kind: ColormapKind::{},\n", meta.kind));
    out.push_str(&format!(
        "        perceptually_uniform: {},\n",
        meta.perceptually_uniform
    ));
    out.push_str(&format!("        cvd_friendly: {},\n", meta.cvd_friendly));
    out.push_str(&format!(
        "        grayscale_safe: {},\n",
        meta.grayscale_safe
    ));
    out.push_str("        lut_size: 256,\n");
    out.push_str(&format!(
        "        citation: \"{}\",\n",
        meta.citation.replace('"', "\\\"")
    ));
    out.push_str("    },\n");
    out.push_str(&format!("    lut: &{const_name}_LUT,\n"));
    out.push_str("};\n\n");

    // LUT
    out.push_str(&format!("static {const_name}_LUT: [[u8; 3]; 256] = [\n"));
    for rgb in &map.lut {
        out.push_str(&format!("    [{}, {}, {}],\n", rgb[0], rgb[1], rgb[2]));
    }
    out.push_str("];\n");

    out
}

/// Generate a single discrete palette .rs file content.
#[allow(dead_code)]
pub fn generate_discrete_palette_rs(palette: &PaletteData, const_name: &str) -> String {
    let meta = &palette.meta;
    let n = palette.colors.len();
    let mut out = String::new();

    out.push_str("// Auto-generated by prismatica xtask -- do not edit\n");
    out.push_str("use crate::{ColormapKind, ColormapMeta, DiscretePalette};\n\n");

    // Doc comment
    out.push_str(&format!(
        "/// {} -- {} discrete palette from the {} collection ({} colors).\n",
        meta.name, meta.kind, meta.collection, n
    ));
    out.push_str(&format!("///\n/// Author: {}\n", meta.author));
    out.push_str(&format!("///\n/// Citation: {}\n", meta.citation));

    // Const declaration
    out.push_str(&format!(
        "pub const {const_name}_PALETTE: DiscretePalette = DiscretePalette {{\n"
    ));
    out.push_str("    meta: ColormapMeta {\n");
    out.push_str(&format!("        name: \"{}\",\n", meta.name));
    out.push_str(&format!("        collection: \"{}\",\n", meta.collection));
    out.push_str(&format!("        author: \"{}\",\n", meta.author));
    out.push_str(&format!("        kind: ColormapKind::{},\n", meta.kind));
    out.push_str(&format!(
        "        perceptually_uniform: {},\n",
        meta.perceptually_uniform
    ));
    out.push_str(&format!("        cvd_friendly: {},\n", meta.cvd_friendly));
    out.push_str(&format!(
        "        grayscale_safe: {},\n",
        meta.grayscale_safe
    ));
    out.push_str(&format!("        lut_size: {},\n", n));
    out.push_str(&format!(
        "        citation: \"{}\",\n",
        meta.citation.replace('"', "\\\"")
    ));
    out.push_str("    },\n");
    out.push_str(&format!("    colors: &{const_name}_PALETTE_COLORS,\n"));
    out.push_str("};\n\n");

    // Colors array
    out.push_str(&format!(
        "static {const_name}_PALETTE_COLORS: [[u8; 3]; {n}] = [\n"
    ));
    for rgb in &palette.colors {
        out.push_str(&format!("    [{}, {}, {}],\n", rgb[0], rgb[1], rgb[2]));
    }
    out.push_str("];\n");

    out
}

/// Generate a combined .rs file with both Colormap and DiscretePalette.
pub fn generate_dual_colormap_rs(
    map: &ColormapData,
    palette: &PaletteData,
    const_name: &str,
) -> String {
    let meta = &map.meta;
    let n = palette.colors.len();
    let mut out = String::new();

    out.push_str("// Auto-generated by prismatica xtask -- do not edit\n");
    out.push_str("use crate::{Colormap, ColormapKind, ColormapMeta, DiscretePalette};\n\n");

    // Doc comment
    out.push_str(&format!(
        "/// {} -- {} colormap from the {} collection.\n",
        meta.name, meta.kind, meta.collection
    ));
    out.push_str(&format!("///\n/// Author: {}\n", meta.author));
    out.push_str(&format!(
        "/// Perceptually uniform: {}\n",
        meta.perceptually_uniform
    ));
    out.push_str(&format!("/// CVD friendly: {}\n", meta.cvd_friendly));
    out.push_str(&format!("///\n/// Citation: {}\n", meta.citation));

    // Colormap const
    out.push_str(&format!("pub const {const_name}: Colormap = Colormap {{\n"));
    out.push_str("    meta: ColormapMeta {\n");
    out.push_str(&format!("        name: \"{}\",\n", meta.name));
    out.push_str(&format!("        collection: \"{}\",\n", meta.collection));
    out.push_str(&format!("        author: \"{}\",\n", meta.author));
    out.push_str(&format!("        kind: ColormapKind::{},\n", meta.kind));
    out.push_str(&format!(
        "        perceptually_uniform: {},\n",
        meta.perceptually_uniform
    ));
    out.push_str(&format!("        cvd_friendly: {},\n", meta.cvd_friendly));
    out.push_str(&format!(
        "        grayscale_safe: {},\n",
        meta.grayscale_safe
    ));
    out.push_str("        lut_size: 256,\n");
    out.push_str(&format!(
        "        citation: \"{}\",\n",
        meta.citation.replace('"', "\\\"")
    ));
    out.push_str("    },\n");
    out.push_str(&format!("    lut: &{const_name}_LUT,\n"));
    out.push_str("};\n\n");

    // LUT
    out.push_str(&format!("static {const_name}_LUT: [[u8; 3]; 256] = [\n"));
    for rgb in &map.lut {
        out.push_str(&format!("    [{}, {}, {}],\n", rgb[0], rgb[1], rgb[2]));
    }
    out.push_str("];\n\n");

    // DiscretePalette const
    out.push_str(&format!("/// Original {}-color discrete palette.\n", n));
    out.push_str(&format!(
        "pub const {const_name}_PALETTE: DiscretePalette = DiscretePalette {{\n"
    ));
    out.push_str("    meta: ColormapMeta {\n");
    out.push_str(&format!("        name: \"{}\",\n", meta.name));
    out.push_str(&format!("        collection: \"{}\",\n", meta.collection));
    out.push_str(&format!("        author: \"{}\",\n", meta.author));
    out.push_str(&format!("        kind: ColormapKind::{},\n", meta.kind));
    out.push_str(&format!(
        "        perceptually_uniform: {},\n",
        meta.perceptually_uniform
    ));
    out.push_str(&format!("        cvd_friendly: {},\n", meta.cvd_friendly));
    out.push_str(&format!(
        "        grayscale_safe: {},\n",
        meta.grayscale_safe
    ));
    out.push_str(&format!("        lut_size: {},\n", n));
    out.push_str(&format!(
        "        citation: \"{}\",\n",
        meta.citation.replace('"', "\\\"")
    ));
    out.push_str("    },\n");
    out.push_str(&format!("    colors: &{const_name}_PALETTE_COLORS,\n"));
    out.push_str("};\n\n");

    out.push_str(&format!(
        "static {const_name}_PALETTE_COLORS: [[u8; 3]; {n}] = [\n"
    ));
    for rgb in &palette.colors {
        out.push_str(&format!("    [{}, {}, {}],\n", rgb[0], rgb[1], rgb[2]));
    }
    out.push_str("];\n");

    out
}

/// Generate mod.rs for a collection (colormaps only).
pub fn generate_mod_rs(
    collection: &str,
    doc_comments: &str,
    maps: &[(&str, String, String)], // (original_name, mod_name, const_name)
) -> String {
    let mut out = String::new();

    out.push_str("// Auto-generated by prismatica xtask -- do not edit\n");

    // Preserved doc comments
    if !doc_comments.is_empty() {
        out.push_str(doc_comments);
        if !doc_comments.ends_with('\n') {
            out.push('\n');
        }
    }
    out.push('\n');

    // Module declarations and re-exports
    for (_orig, mod_name, const_name) in maps {
        out.push_str(&format!("mod {mod_name};\n"));
        out.push_str(&format!("pub use {mod_name}::{const_name};\n\n"));
    }

    // ALL array
    out.push_str(&format!(
        "/// All colormaps in the {collection} collection.\n"
    ));
    out.push_str("pub static ALL: &[&crate::Colormap] = &[\n");
    for (_orig, _mod_name, const_name) in maps {
        out.push_str(&format!("    &{const_name},\n"));
    }
    out.push_str("];\n");

    out
}

/// Generate mod.rs for a collection that has both colormaps and discrete palettes.
pub fn generate_mod_rs_with_palettes(
    collection: &str,
    doc_comments: &str,
    maps: &[(&str, String, String)], // (original_name, mod_name, const_name) — colormaps
    palettes: &[(&str, String, String)], // (original_name, mod_name, const_name) — palettes (const_name without _PALETTE suffix)
) -> String {
    let mut out = String::new();

    out.push_str("// Auto-generated by prismatica xtask -- do not edit\n");

    if !doc_comments.is_empty() {
        out.push_str(doc_comments);
        if !doc_comments.ends_with('\n') {
            out.push('\n');
        }
    }
    out.push('\n');

    // Module declarations and re-exports — deduplicate modules
    // (a map and its palette share the same module file)
    let mut declared_modules = std::collections::BTreeSet::new();
    for (_orig, mod_name, const_name) in maps {
        if declared_modules.insert(mod_name.clone()) {
            out.push_str(&format!("mod {mod_name};\n"));
        }
        out.push_str(&format!("pub use {mod_name}::{const_name};\n"));
    }
    for (_orig, mod_name, const_name) in palettes {
        if declared_modules.insert(mod_name.clone()) {
            out.push_str(&format!("mod {mod_name};\n"));
        }
        out.push_str(&format!("pub use {mod_name}::{const_name}_PALETTE;\n"));
    }
    out.push('\n');

    // ALL colormaps array
    out.push_str(&format!(
        "/// All colormaps in the {collection} collection.\n"
    ));
    out.push_str("pub static ALL: &[&crate::Colormap] = &[\n");
    for (_orig, _mod_name, const_name) in maps {
        out.push_str(&format!("    &{const_name},\n"));
    }
    out.push_str("];\n\n");

    // ALL_DISCRETE palettes array
    out.push_str(&format!(
        "/// All discrete palettes in the {collection} collection.\n"
    ));
    out.push_str("pub static ALL_DISCRETE: &[&crate::DiscretePalette] = &[\n");
    for (_orig, _mod_name, const_name) in palettes {
        out.push_str(&format!("    &{const_name}_PALETTE,\n"));
    }
    out.push_str("];\n");

    out
}

/// Generate a collection from data/{collection}/ into src/{collection}/.
/// This is the generic path for collections with only Colormap entries.
pub fn generate_collection(project_root: &Path, collection: &str, data_dir: &Path) {
    println!("Generating {collection}...");

    let src_dir = project_root.join("src").join(collection);
    fs::create_dir_all(&src_dir).expect("create src/{collection}/");

    // Load all colormaps from data directory
    let mut maps: Vec<ColormapData> = Vec::new();

    let mut csv_files: Vec<_> = fs::read_dir(data_dir)
        .expect("read data dir")
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.extension().map(|x| x == "csv").unwrap_or(false)
                && !path
                    .file_stem()
                    .map(|s| s.to_string_lossy().ends_with("_palette"))
                    .unwrap_or(false)
        })
        .collect();
    csv_files.sort_by_key(|e| e.file_name());

    for entry in csv_files {
        let csv_path = entry.path();
        let stem = csv_path
            .file_stem()
            .expect("CSV file should have a stem")
            .to_string_lossy()
            .to_string();
        let json_path = data_dir.join(format!("{stem}.json"));

        let csv_text = fs::read_to_string(&csv_path).expect("read CSV");
        let lut: Vec<[u8; 3]> = csv_text
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|line| {
                let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                assert_eq!(parts.len(), 3, "bad CSV line: {line}");
                [
                    parts[0].parse::<u8>().expect("parse R"),
                    parts[1].parse::<u8>().expect("parse G"),
                    parts[2].parse::<u8>().expect("parse B"),
                ]
            })
            .collect();

        let json_text = fs::read_to_string(&json_path).expect("read JSON");
        let meta: ColormapMeta = serde_json::from_str(&json_text).expect("parse JSON");

        maps.push(ColormapData { meta, lut });
    }

    // Generate per-colormap .rs files
    for map in &maps {
        let mod_name = to_mod_name(&map.meta.name);
        let const_name = to_const_name(&map.meta.name);
        let rs_path = src_dir.join(format!("{mod_name}.rs"));

        let code = generate_colormap_rs(map, &const_name);
        fs::write(&rs_path, code).expect("write .rs");
        println!(
            "  Generating {collection}/{mod_name}.rs ({})",
            map.meta.kind
        );
    }

    // Generate mod.rs (preserving doc comments from existing file)
    let mod_rs_path = src_dir.join("mod.rs");
    let existing_doc_comments = read_doc_comments(&mod_rs_path);

    let mut sorted_names: Vec<(&str, String, String)> = maps
        .iter()
        .map(|m| {
            (
                m.meta.name.as_str(),
                to_mod_name(&m.meta.name),
                to_const_name(&m.meta.name),
            )
        })
        .collect();
    sorted_names.sort_by(|a, b| a.1.cmp(&b.1));

    let mod_code = generate_mod_rs(collection, &existing_doc_comments, &sorted_names);
    fs::write(&mod_rs_path, mod_code).expect("write mod.rs");
    println!("  Generated {collection}/mod.rs ({} maps)", maps.len());
}

// ---------------------------------------------------------------------------
// Name conversion
// ---------------------------------------------------------------------------

/// Convert a colormap name to a Rust module/filename (lowercase, hyphens to underscores).
pub fn to_mod_name(name: &str) -> String {
    let mut s = name.replace('-', "_").to_lowercase();
    if s.starts_with(|c: char| c.is_ascii_digit()) {
        s.insert(0, 'n');
    }
    s
}

/// Convert a colormap name to a Rust const name (UPPER_CASE, hyphens to underscores).
pub fn to_const_name(name: &str) -> String {
    let mut s = name.replace('-', "_").to_uppercase();
    if s.starts_with(|c: char| c.is_ascii_digit()) {
        s.insert(0, 'N');
    }
    s
}
