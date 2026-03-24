use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("all");

    let project_root = project_root();

    match command {
        "fetch" => fetch_all(&project_root),
        "generate" => generate_all(&project_root),
        "all" => {
            fetch_all(&project_root);
            generate_all(&project_root);
        }
        other => {
            eprintln!("Unknown command: {other}");
            eprintln!("Usage: cargo xtask [fetch|generate|all]");
            std::process::exit(1);
        }
    }
}

fn project_root() -> PathBuf {
    let xtask_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    xtask_dir.parent().unwrap().to_path_buf()
}

// ---------------------------------------------------------------------------
// Shared types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ColormapMeta {
    name: String,
    collection: String,
    author: String,
    kind: String,
    perceptually_uniform: bool,
    cvd_friendly: bool,
    grayscale_safe: bool,
    citation: String,
}

/// A fully loaded colormap: 256 RGB triplets + metadata.
struct ColormapData {
    meta: ColormapMeta,
    lut: Vec<[u8; 3]>,
}

// ---------------------------------------------------------------------------
// FETCH
// ---------------------------------------------------------------------------

fn fetch_all(project_root: &Path) {
    fetch_crameri(project_root);
    fetch_matplotlib(project_root);
    println!("Fetch complete.");
}

// -- Crameri ----------------------------------------------------------------

const CRAMERI_ZIP_URL: &str = "https://zenodo.org/records/8409685/files/ScientificColourMaps8.zip";

const CRAMERI_CYCLIC: &[&str] = &["romaO", "bamO", "brocO", "corkO", "vikO"];
const CRAMERI_DIVERGING: &[&str] = &[
    "berlin", "broc", "cork", "vik", "lisbon", "tofino", "vanimo", "bam", "roma", "managua",
];
const CRAMERI_MULTI_SEQ: &[&str] = &["oleron", "bukavu", "fes"];

fn crameri_kind(name: &str) -> &'static str {
    if CRAMERI_CYCLIC.contains(&name) {
        "Cyclic"
    } else if CRAMERI_DIVERGING.contains(&name) {
        "Diverging"
    } else if CRAMERI_MULTI_SEQ.contains(&name) {
        "MultiSequential"
    } else {
        "Sequential"
    }
}

fn fetch_crameri(project_root: &Path) {
    println!("Fetching Crameri from Zenodo...");

    let data_dir = project_root.join("data").join("crameri");
    fs::create_dir_all(&data_dir).expect("create data/crameri/");

    // Download ZIP into memory using raw Read (ureq default limit is too low)
    let mut buf: Vec<u8> = Vec::new();
    ureq::get(CRAMERI_ZIP_URL)
        .call()
        .expect("download Crameri ZIP")
        .body_mut()
        .as_reader()
        .read_to_end(&mut buf)
        .expect("read Crameri ZIP body");

    println!("  Downloaded {} bytes", buf.len());

    let reader = std::io::Cursor::new(buf);
    let mut archive = zip::ZipArchive::new(reader).expect("open ZIP archive");

    // Collect matching CSV entries. We want paths like
    // ScientificColourMaps8/{name}/{name}.csv
    let mut maps: BTreeMap<String, Vec<[u8; 3]>> = BTreeMap::new();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).expect("zip entry");
        let path = entry.name().to_string();

        // Match .txt files: look for {name}/{name}.txt pattern
        // Crameri distributes as space-separated float .txt files, not CSV
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() < 2 {
            continue;
        }
        let dir_name = parts[parts.len() - 2];
        let file_name = parts[parts.len() - 1];

        // The file must be {dir_name}.txt exactly
        let expected = format!("{}.txt", dir_name);
        if file_name != expected {
            continue;
        }
        // Skip special directories
        if dir_name.starts_with('+') || dir_name.starts_with('.') {
            continue;
        }

        let mut contents = String::new();
        entry
            .read_to_string(&mut contents)
            .expect("read TXT from ZIP");

        let lut = parse_space_separated_floats(&contents);
        if lut.len() != 256 {
            eprintln!(
                "  Warning: {}/{} has {} rows, expected 256 -- skipping",
                dir_name,
                file_name,
                lut.len()
            );
            continue;
        }

        maps.insert(dir_name.to_string(), lut);
    }

    println!("  Found {} colormaps in archive", maps.len());

    for (name, lut) in &maps {
        let kind = crameri_kind(name);
        let meta = ColormapMeta {
            name: name.clone(),
            collection: "crameri".to_string(),
            author: "Fabio Crameri".to_string(),
            kind: kind.to_string(),
            perceptually_uniform: true,
            cvd_friendly: true,
            grayscale_safe: true,
            citation:
                "Crameri, F. (2018). Scientific colour maps. Zenodo. doi:10.5281/zenodo.1243862"
                    .to_string(),
        };

        write_csv(&data_dir.join(format!("{}.csv", name)), lut);
        write_json(&data_dir.join(format!("{}.json", name)), &meta);
        println!("  Wrote data/crameri/{}.csv + .json ({kind})", name);
    }
}

// -- Matplotlib -------------------------------------------------------------

struct MplMapDef {
    name: &'static str,
    kind: &'static str,
    author: &'static str,
    cvd_friendly: bool,
    grayscale_safe: bool,
    citation: &'static str,
}

const MPL_MAPS: &[MplMapDef] = &[
    MplMapDef {
        name: "viridis",
        kind: "Sequential",
        author: "van der Walt, Smith",
        cvd_friendly: true,
        grayscale_safe: true,
        citation: "van der Walt, S. & Smith, N. (2015). matplotlib colormaps.",
    },
    MplMapDef {
        name: "magma",
        kind: "Sequential",
        author: "van der Walt, Smith",
        cvd_friendly: true,
        grayscale_safe: true,
        citation: "van der Walt, S. & Smith, N. (2015). matplotlib colormaps.",
    },
    MplMapDef {
        name: "inferno",
        kind: "Sequential",
        author: "van der Walt, Smith",
        cvd_friendly: true,
        grayscale_safe: true,
        citation: "van der Walt, S. & Smith, N. (2015). matplotlib colormaps.",
    },
    MplMapDef {
        name: "plasma",
        kind: "Sequential",
        author: "van der Walt, Smith",
        cvd_friendly: true,
        grayscale_safe: true,
        citation: "van der Walt, S. & Smith, N. (2015). matplotlib colormaps.",
    },
    MplMapDef {
        name: "cividis",
        kind: "Sequential",
        author: "Nunez, Anderton, Renslow",
        cvd_friendly: true,
        grayscale_safe: true,
        citation: "Nunez, J. R. et al. (2018). Optimizing colormaps for color vision deficiency.",
    },
    MplMapDef {
        name: "twilight",
        kind: "Cyclic",
        author: "Bastian",
        cvd_friendly: true,
        grayscale_safe: true,
        citation: "matplotlib project",
    },
    MplMapDef {
        name: "mako",
        kind: "Sequential",
        author: "Waskom",
        cvd_friendly: true,
        grayscale_safe: true,
        citation: "Waskom, M. (2021). seaborn. JOSS, 6(60), 3021.",
    },
    MplMapDef {
        name: "rocket",
        kind: "Sequential",
        author: "Waskom",
        cvd_friendly: true,
        grayscale_safe: true,
        citation: "Waskom, M. (2021). seaborn. JOSS, 6(60), 3021.",
    },
];

fn fetch_matplotlib(project_root: &Path) {
    println!("Fetching matplotlib collection...");

    let data_dir = project_root.join("data").join("matplotlib");
    fs::create_dir_all(&data_dir).expect("create data/matplotlib/");

    for def in MPL_MAPS {
        println!("  Fetching {}...", def.name);
        let lut = fetch_mpl_map(def.name);
        assert_eq!(
            lut.len(),
            256,
            "{} has {} entries, expected 256",
            def.name,
            lut.len()
        );

        let meta = ColormapMeta {
            name: def.name.to_string(),
            collection: "matplotlib".to_string(),
            author: def.author.to_string(),
            kind: def.kind.to_string(),
            perceptually_uniform: true,
            cvd_friendly: def.cvd_friendly,
            grayscale_safe: def.grayscale_safe,
            citation: def.citation.to_string(),
        };

        write_csv(&data_dir.join(format!("{}.csv", def.name)), &lut);
        write_json(&data_dir.join(format!("{}.json", def.name)), &meta);
        println!("  Wrote data/matplotlib/{}.csv + .json", def.name);
    }
}

fn fetch_mpl_map(name: &str) -> Vec<[u8; 3]> {
    match name {
        // All in matplotlib's _cm_listed.py with _<name>_data markers
        "viridis" => fetch_matplotlib_cm_listed("_viridis_data"),
        "magma" => fetch_matplotlib_cm_listed("_magma_data"),
        "inferno" => fetch_matplotlib_cm_listed("_inferno_data"),
        "plasma" => fetch_matplotlib_cm_listed("_plasma_data"),
        "cividis" => fetch_matplotlib_cm_listed("_cividis_data"),
        "twilight" => fetch_matplotlib_cm_listed("_twilight_data"),
        // Seaborn colormaps
        "mako" => fetch_seaborn_map("mako"),
        "rocket" => fetch_seaborn_map("rocket"),
        _ => panic!("unknown matplotlib map: {name}"),
    }
}

fn fetch_url_text(url: &str) -> String {
    ureq::get(url)
        .call()
        .unwrap_or_else(|e| panic!("GET {url}: {e}"))
        .body_mut()
        .read_to_string()
        .unwrap_or_else(|e| panic!("read body from {url}: {e}"))
}

/// Fetch from matplotlib _cm_listed.py (viridis, magma, inferno, plasma, cividis, twilight).
/// All these maps are defined in matplotlib's _cm_listed.py.
fn fetch_matplotlib_cm_listed(marker: &str) -> Vec<[u8; 3]> {
    let url =
        "https://raw.githubusercontent.com/matplotlib/matplotlib/main/lib/matplotlib/_cm_listed.py";
    let text = fetch_url_text(url);
    parse_python_section(&text, marker)
}

/// Fetch from seaborn cm.py (mako, rocket).
fn fetch_seaborn_map(name: &str) -> Vec<[u8; 3]> {
    let url = "https://raw.githubusercontent.com/mwaskom/seaborn/master/seaborn/cm.py";
    let text = fetch_url_text(url);
    let marker = format!("_{}_lut", name);
    parse_python_section(&text, &marker)
}

/// Parse a section of a Python file starting at `marker`, collecting float
/// triples until we run out.
fn parse_python_section(text: &str, marker: &str) -> Vec<[u8; 3]> {
    let mut result = Vec::new();
    let mut found = false;

    for line in text.lines() {
        if !found {
            if line.contains(marker) {
                found = true;
                // The marker line itself might contain data after `= [`
                if let Some(triple) = try_parse_float_triple_from_line(line) {
                    result.push(triple);
                }
            }
            continue;
        }

        // Stop when we hit a line that looks like a new top-level definition
        // (non-indented, not blank, not a continuation of the data).
        // Heuristic: if we already have data and hit a line starting with
        // a letter/underscore or `]` or `)` at column 0 that isn't data,
        // we might be done. But safer: just try to parse and stop when we
        // get enough, or when a non-data line shows up after we started
        // collecting.
        if let Some(triple) = try_parse_float_triple_from_line(line) {
            result.push(triple);
        } else if !result.is_empty() {
            // We had data and now hit a non-data line. If it's just a
            // closing bracket/paren or blank, keep going; otherwise stop.
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed == "]"
                || trimmed == "),"
                || trimmed == ")"
                || trimmed == "],"
                || trimmed == "},"
                || trimmed == "}"
            {
                // Could be the end of the array -- if we have 256 entries
                // we're done.
                if result.len() >= 256 {
                    break;
                }
                continue;
            }
            // Definitely a new section
            break;
        }
    }

    // Resample to exactly 256 entries if needed.
    // Most sources have 256, twilight has 510, some might have 255 due to parsing edge cases.
    if result.len() == 256 {
        result
    } else if result.len() > 256 || (result.len() >= 200 && result.len() < 256) {
        resample(&result, 256)
    } else {
        panic!(
            "Expected ~256 entries after marker '{}', got {}",
            marker,
            result.len()
        );
    }
}

/// Try to parse a single `[r, g, b]` triple from a Python-ish line.
fn try_parse_float_triple_from_line(line: &str) -> Option<[u8; 3]> {
    let trimmed = line.trim();

    // Strip common wrappers: leading `[`, trailing `]`, trailing `,`
    let s = trimmed
        .trim_start_matches('[')
        .trim_end_matches(',')
        .trim_end_matches(']')
        .trim();

    let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
    if parts.len() != 3 {
        return None;
    }

    let r: f64 = parts[0].parse().ok()?;
    let g: f64 = parts[1].parse().ok()?;
    let b: f64 = parts[2].parse().ok()?;

    // Sanity check: should be in [0, 1]
    if r < -0.01 || r > 1.01 || g < -0.01 || g > 1.01 || b < -0.01 || b > 1.01 {
        return None;
    }

    Some(float_to_u8(r, g, b))
}

fn float_to_u8(r: f64, g: f64, b: f64) -> [u8; 3] {
    [
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    ]
}

/// Linearly resample a LUT to `n` entries.
fn resample(lut: &[[u8; 3]], n: usize) -> Vec<[u8; 3]> {
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

// -- CSV / JSON I/O ---------------------------------------------------------

/// Parse space-separated float triples (Crameri .txt format).
fn parse_space_separated_floats(text: &str) -> Vec<[u8; 3]> {
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

fn write_csv(path: &Path, lut: &[[u8; 3]]) {
    let mut out = String::new();
    for rgb in lut {
        out.push_str(&format!("{},{},{}\n", rgb[0], rgb[1], rgb[2]));
    }
    fs::write(path, out).expect("write CSV");
}

fn write_json(path: &Path, meta: &ColormapMeta) {
    let json = serde_json::to_string_pretty(meta).expect("serialize JSON");
    fs::write(path, json).expect("write JSON");
}

// ---------------------------------------------------------------------------
// GENERATE
// ---------------------------------------------------------------------------

fn generate_all(project_root: &Path) {
    let data_dir = project_root.join("data");

    for collection in &["crameri", "matplotlib"] {
        let col_data = data_dir.join(collection);
        if !col_data.exists() {
            eprintln!(
                "Warning: data/{collection}/ does not exist -- run `cargo xtask fetch` first"
            );
            continue;
        }
        generate_collection(project_root, collection, &col_data);
    }
    println!("Generate complete.");
}

fn generate_collection(project_root: &Path, collection: &str, data_dir: &Path) {
    println!("Generating {collection}...");

    let src_dir = project_root.join("src").join(collection);
    fs::create_dir_all(&src_dir).expect("create src/{collection}/");

    // Load all colormaps from data directory
    let mut maps: Vec<ColormapData> = Vec::new();

    let mut csv_files: Vec<_> = fs::read_dir(data_dir)
        .expect("read data dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "csv").unwrap_or(false))
        .collect();
    csv_files.sort_by_key(|e| e.file_name());

    for entry in csv_files {
        let csv_path = entry.path();
        let stem = csv_path.file_stem().unwrap().to_string_lossy().to_string();
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

fn generate_colormap_rs(map: &ColormapData, const_name: &str) -> String {
    let meta = &map.meta;
    let mut out = String::new();

    out.push_str("// Auto-generated by prismatica xtask -- do not edit\n");
    out.push_str("use crate::{Colormap, ColormapMeta, ColormapKind};\n\n");

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

fn generate_mod_rs(
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

/// Read `//!` doc comment lines from an existing mod.rs file.
fn read_doc_comments(path: &Path) -> String {
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

// -- Name conversion --------------------------------------------------------

/// Convert a colormap name to a Rust module/filename (lowercase, hyphens to underscores).
fn to_mod_name(name: &str) -> String {
    let mut s = name.replace('-', "_").to_lowercase();
    if s.starts_with(|c: char| c.is_ascii_digit()) {
        s.insert(0, 'n');
    }
    s
}

/// Convert a colormap name to a Rust const name (UPPER_CASE, hyphens to underscores).
fn to_const_name(name: &str) -> String {
    let mut s = name.replace('-', "_").to_uppercase();
    if s.starts_with(|c: char| c.is_ascii_digit()) {
        s.insert(0, 'N');
    }
    s
}
