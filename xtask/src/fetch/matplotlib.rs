use crate::codegen::{self, ColormapMeta};
use std::path::Path;

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

pub fn fetch(project_root: &Path) {
    println!("Fetching matplotlib collection...");

    let data_dir = project_root.join("data").join("matplotlib");
    std::fs::create_dir_all(&data_dir).expect("create data/matplotlib/");

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

        codegen::write_csv(&data_dir.join(format!("{}.csv", def.name)), &lut);
        codegen::write_json(&data_dir.join(format!("{}.json", def.name)), &meta);
        println!("  Wrote data/matplotlib/{}.csv + .json", def.name);
    }
}

fn fetch_mpl_map(name: &str) -> Vec<[u8; 3]> {
    match name {
        "viridis" => fetch_matplotlib_cm_listed("_viridis_data"),
        "magma" => fetch_matplotlib_cm_listed("_magma_data"),
        "inferno" => fetch_matplotlib_cm_listed("_inferno_data"),
        "plasma" => fetch_matplotlib_cm_listed("_plasma_data"),
        "cividis" => fetch_matplotlib_cm_listed("_cividis_data"),
        "twilight" => fetch_matplotlib_cm_listed("_twilight_data"),
        "mako" => fetch_seaborn_map("mako"),
        "rocket" => fetch_seaborn_map("rocket"),
        _ => panic!("unknown matplotlib map: {name}"),
    }
}

/// Fetch from matplotlib _cm_listed.py.
fn fetch_matplotlib_cm_listed(marker: &str) -> Vec<[u8; 3]> {
    let url =
        "https://raw.githubusercontent.com/matplotlib/matplotlib/main/lib/matplotlib/_cm_listed.py";
    let text = codegen::fetch_url_text(url);
    parse_python_section(&text, marker)
}

/// Fetch from seaborn cm.py (mako, rocket).
fn fetch_seaborn_map(name: &str) -> Vec<[u8; 3]> {
    let url = "https://raw.githubusercontent.com/mwaskom/seaborn/master/seaborn/cm.py";
    let text = codegen::fetch_url_text(url);
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
                if let Some(triple) = try_parse_float_triple_from_line(line) {
                    result.push(triple);
                }
            }
            continue;
        }

        if let Some(triple) = try_parse_float_triple_from_line(line) {
            result.push(triple);
        } else if !result.is_empty() {
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed == "]"
                || trimmed == "),"
                || trimmed == ")"
                || trimmed == "],"
                || trimmed == "},"
                || trimmed == "}"
            {
                if result.len() >= 256 {
                    break;
                }
                continue;
            }
            break;
        }
    }

    if result.len() == 256 {
        result
    } else if result.len() > 256 || (result.len() >= 200 && result.len() < 256) {
        codegen::resample(&result, 256)
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
    let range = -0.01..=1.01;
    if !range.contains(&r) || !range.contains(&g) || !range.contains(&b) {
        return None;
    }

    Some(codegen::float_to_u8(r, g, b))
}
