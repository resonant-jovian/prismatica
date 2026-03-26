use crate::codegen::{self, ColormapMeta};
use std::path::Path;

const COLORBREWER_JSON_URL: &str =
    "https://raw.githubusercontent.com/axismaps/colorbrewer/master/export/colorbrewer.json";

const COLORBREWER_CITATION: &str =
    "Brewer, C. A. (2003). ColorBrewer. www.colorbrewer2.org";

/// Map the ColorBrewer `"type"` field to our `ColormapKind` variant name.
fn classify(cb_type: &str) -> &'static str {
    match cb_type {
        "seq" => "Sequential",
        "div" => "Diverging",
        "qual" => "Qualitative",
        other => {
            eprintln!("  Warning: unknown ColorBrewer type '{other}', defaulting to Sequential");
            "Sequential"
        }
    }
}

/// Parse an `"rgb(R,G,B)"` string into `[u8; 3]`.
fn parse_rgb_string(s: &str) -> [u8; 3] {
    // Format: "rgb(R,G,B)"
    let inner = s
        .split('(')
        .nth(1)
        .and_then(|rest| rest.split(')').next())
        .unwrap_or_else(|| panic!("bad rgb string: {s}"));

    let parts: Vec<&str> = inner.split(',').collect();
    assert_eq!(parts.len(), 3, "expected 3 components in rgb(): {s}");

    let r: u8 = parts[0].trim().parse().unwrap_or_else(|e| {
        panic!("parse R from '{s}': {e}")
    });
    let g: u8 = parts[1].trim().parse().unwrap_or_else(|e| {
        panic!("parse G from '{s}': {e}")
    });
    let b: u8 = parts[2].trim().parse().unwrap_or_else(|e| {
        panic!("parse B from '{s}': {e}")
    });

    [r, g, b]
}

pub fn fetch(project_root: &Path) {
    println!("Fetching ColorBrewer from GitHub...");

    let data_dir = project_root.join("data").join("colorbrewer");
    std::fs::create_dir_all(&data_dir).expect("create data/colorbrewer/");

    let json_text = codegen::fetch_url_text(COLORBREWER_JSON_URL);
    let root: serde_json::Value =
        serde_json::from_str(&json_text).expect("parse ColorBrewer JSON");

    let obj = root.as_object().expect("top-level JSON must be an object");

    let mut count = 0u32;

    for (name, value) in obj {
        let palette_obj = match value.as_object() {
            Some(o) => o,
            None => continue,
        };

        // Extract the "type" field -- skip entries that don't have one
        // (e.g. a top-level "properties" key).
        let cb_type = match palette_obj.get("type").and_then(|v| v.as_str()) {
            Some(t) => t,
            None => continue,
        };

        // Find the maximum-length numeric variant.
        // Keys are "3", "4", ..., "9", "10", "11", "12", plus "type"/"properties".
        let max_variant = palette_obj
            .iter()
            .filter_map(|(k, v)| {
                let n: usize = k.parse().ok()?;
                let arr = v.as_array()?;
                Some((n, arr))
            })
            .max_by_key(|(n, _)| *n);

        let (_max_n, color_strings) = match max_variant {
            Some(v) => v,
            None => {
                eprintln!("  Warning: palette '{name}' has no numeric variants -- skipping");
                continue;
            }
        };

        // Parse each "rgb(R,G,B)" string.
        let colors: Vec<[u8; 3]> = color_strings
            .iter()
            .map(|v| {
                let s = v.as_str().unwrap_or_else(|| {
                    panic!("expected string in palette '{name}', got {v}")
                });
                parse_rgb_string(s)
            })
            .collect();

        if colors.is_empty() {
            eprintln!("  Warning: palette '{name}' produced 0 colors -- skipping");
            continue;
        }

        let kind = classify(cb_type);

        let meta = ColormapMeta {
            name: name.clone(),
            collection: "colorbrewer".to_string(),
            author: "Cynthia Brewer".to_string(),
            kind: kind.to_string(),
            perceptually_uniform: matches!(kind, "Sequential" | "Diverging"),
            cvd_friendly: false,
            grayscale_safe: false,
            citation: COLORBREWER_CITATION.to_string(),
        };

        // Resample to 256 entries for continuous colormap use.
        let resampled = codegen::resample(&colors, 256);
        codegen::write_csv(&data_dir.join(format!("{name}.csv")), &resampled);

        // Write the original discrete palette (no resampling).
        codegen::write_csv(&data_dir.join(format!("{name}_palette.csv")), &colors);

        // Write metadata JSON.
        codegen::write_json(&data_dir.join(format!("{name}.json")), &meta);

        println!(
            "  Wrote data/colorbrewer/{name}.csv + _palette.csv + .json ({kind}, {} colors)",
            colors.len()
        );
        count += 1;
    }

    println!("  ColorBrewer: {count} palettes fetched.");
}
