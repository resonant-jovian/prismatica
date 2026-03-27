use crate::codegen::{self, ColormapMeta};
use std::path::Path;

const CARTOCOLORS_TS_URL: &str =
    "https://raw.githubusercontent.com/CartoDB/CartoColor/master/src/carto.ts";

const CARTOCOLORS_CITATION: &str = "CARTO. CartoColor. carto.com/carto-colors";

/// CartoColor-unique scheme definitions: (name, kind).
/// Excludes ColorBrewer duplicates.
const SCHEMES: &[(&str, &str)] = &[
    ("Burg", "Sequential"),
    ("BurgYl", "Sequential"),
    ("RedOr", "Sequential"),
    ("OrYel", "Sequential"),
    ("Peach", "Sequential"),
    ("PinkYl", "Sequential"),
    ("Mint", "Sequential"),
    ("BluGrn", "Sequential"),
    ("DarkMint", "Sequential"),
    ("Emrld", "Sequential"),
    ("ag_GrnYl", "Sequential"),
    ("BluYl", "Sequential"),
    ("Teal", "Sequential"),
    ("TealGrn", "Sequential"),
    ("Purp", "Sequential"),
    ("PurpOr", "Sequential"),
    ("Sunset", "Sequential"),
    ("Magenta", "Sequential"),
    ("SunsetDark", "Sequential"),
    ("ag_Sunset", "Sequential"),
    ("BrwnYl", "Sequential"),
    ("ArmyRose", "Diverging"),
    ("Fall", "Diverging"),
    ("Geyser", "Diverging"),
    ("Temps", "Diverging"),
    ("TealRose", "Diverging"),
    ("Tropic", "Diverging"),
    ("Earth", "Diverging"),
    ("Antique", "Qualitative"),
    ("Bold", "Qualitative"),
    ("Pastel", "Qualitative"),
    ("Prism", "Qualitative"),
    ("Safe", "Qualitative"),
    ("Vivid", "Qualitative"),
];

/// Parse a `"#RRGGBB"` hex string into `[u8; 3]`.
fn parse_hex_color(s: &str) -> Option<[u8; 3]> {
    let hex = s.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some([r, g, b])
}

/// Extract hex colors for a given scheme name from the TypeScript source.
/// Looks for `export const {name} = { ... };` and finds the highest
/// numeric key's array of hex strings.
fn extract_scheme_colors(ts_text: &str, name: &str) -> Option<Vec<[u8; 3]>> {
    // Find the `export const {name} = {` block
    let marker = format!("export const {name} = {{");
    let start = ts_text.find(&marker)?;
    let block_start = start + marker.len();

    // Find the closing `};` for this export
    let rest = &ts_text[block_start..];
    let block_end = rest.find("};")?;
    let block = &rest[..block_end];

    // Find all hex color arrays. We want the longest one.
    // Lines look like: 7: ["#hex1", "#hex2", ...],
    let mut best_colors: Vec<[u8; 3]> = Vec::new();

    for line in block.lines() {
        let trimmed = line.trim();
        // Look for lines that start with a digit (numeric key)
        if !trimmed.starts_with(|c: char| c.is_ascii_digit()) {
            continue;
        }

        // Extract all #RRGGBB strings from this line
        let colors: Vec<[u8; 3]> = trimmed
            .split('"')
            .filter(|s| s.starts_with('#') && s.len() == 7)
            .filter_map(parse_hex_color)
            .collect();

        if colors.len() > best_colors.len() {
            best_colors = colors;
        }
    }

    // Also check multi-line arrays (e.g. 7: [\n "#hex1",\n "#hex2",\n ...])
    // by collecting all hex strings between a numeric key and the next numeric key or `tags`
    let mut current_key: Option<usize> = None;
    let mut current_colors: Vec<[u8; 3]> = Vec::new();

    for line in block.lines() {
        let trimmed = line.trim();

        // Check if this is a new key line
        if let Some(colon_pos) = trimmed.find(':') {
            let key_part = trimmed[..colon_pos].trim();
            if let Ok(n) = key_part.parse::<usize>() {
                // Flush previous
                if current_colors.len() > best_colors.len() {
                    best_colors = current_colors.clone();
                }
                current_key = Some(n);
                current_colors.clear();
            } else if key_part == "tags" {
                // Flush and stop
                if current_colors.len() > best_colors.len() {
                    best_colors = current_colors.clone();
                }
                current_key = None;
                current_colors.clear();
            }
        }

        if current_key.is_some() {
            // Collect hex colors from this line
            for part in trimmed.split('"') {
                if part.starts_with('#')
                    && part.len() == 7
                    && let Some(c) = parse_hex_color(part)
                {
                    current_colors.push(c);
                }
            }
        }
    }
    // Final flush
    if current_colors.len() > best_colors.len() {
        best_colors = current_colors;
    }

    if best_colors.is_empty() {
        None
    } else {
        Some(best_colors)
    }
}

pub fn fetch(project_root: &Path) {
    println!("Fetching CartoColors from GitHub...");

    let data_dir = project_root.join("data").join("cartocolors");
    std::fs::create_dir_all(&data_dir).expect("create data/cartocolors/");

    let ts_text = codegen::fetch_url_text(CARTOCOLORS_TS_URL);

    let mut count = 0u32;

    for &(name, kind) in SCHEMES {
        let colors = match extract_scheme_colors(&ts_text, name) {
            Some(c) => c,
            None => {
                eprintln!("  Warning: scheme '{name}' not found in carto.ts -- skipping");
                continue;
            }
        };

        let meta = ColormapMeta {
            name: name.to_string(),
            collection: "cartocolors".to_string(),
            author: "CARTO".to_string(),
            kind: kind.to_string(),
            perceptually_uniform: false,
            cvd_friendly: false,
            grayscale_safe: false,
            citation: CARTOCOLORS_CITATION.to_string(),
        };

        let resampled = codegen::resample(&colors, 256);
        codegen::write_csv(&data_dir.join(format!("{name}.csv")), &resampled);
        codegen::write_csv(&data_dir.join(format!("{name}_palette.csv")), &colors);
        codegen::write_json(&data_dir.join(format!("{name}.json")), &meta);

        println!(
            "  Wrote data/cartocolors/{name}.csv + _palette.csv + .json ({kind}, {} colors)",
            colors.len()
        );
        count += 1;
    }

    println!("  CartoColors: {count} schemes fetched.");
}
