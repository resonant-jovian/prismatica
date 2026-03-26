use crate::codegen::{self, ColormapMeta};
use std::path::Path;

const NCAR_MAPS: &[(&str, &str)] = &[
    ("amwg256", "Sequential"),
    ("BlAqGrYeOrRe", "Sequential"),
    ("BlAqGrYeOrReVi200", "Sequential"),
    ("BlGrYeOrReVi200", "Sequential"),
    ("BlRe", "Diverging"),
    ("BlWhRe", "Diverging"),
    ("BkBlAqGrYeOrReViWh200", "Sequential"),
    ("GMT_cool", "Sequential"),
    ("GMT_copper", "Sequential"),
    ("GMT_drywet", "Diverging"),
    ("GMT_gebco", "Sequential"),
    ("GMT_globe", "Sequential"),
    ("GMT_gray", "Sequential"),
    ("GMT_haxby", "Sequential"),
    ("GMT_hot", "Sequential"),
    ("GMT_jet", "Sequential"),
    ("GMT_ocean", "Sequential"),
    ("GMT_panoply", "Diverging"),
    ("GMT_polar", "Diverging"),
    ("GMT_relief", "Sequential"),
    ("GMT_seis", "Diverging"),
    ("GMT_split", "Diverging"),
    ("GMT_topo", "Sequential"),
    ("GMT_wysiwyg", "Sequential"),
    ("GreenMagenta16", "Diverging"),
    ("hotcold_18lev", "Diverging"),
    ("hotcolr_19lev", "Sequential"),
    ("matlab_jet", "Sequential"),
    ("matlab_hsv", "Cyclic"),
    ("ncview_default", "Sequential"),
    ("nrl_sirkes", "Sequential"),
    ("nrl_sirkes_nowhite", "Sequential"),
    ("posneg_1", "Diverging"),
    ("precip3_16lev", "Sequential"),
    ("radar_1", "Sequential"),
    ("sunshine_9lev", "Sequential"),
    ("temp_19lev", "Sequential"),
    ("temp_diff_18lev", "Diverging"),
    ("thelix", "Sequential"),
    ("topo_15lev", "Sequential"),
    ("wgne15", "Sequential"),
    ("WhBlGrYeRe", "Sequential"),
    ("WhViBlGrYeOrRe", "Sequential"),
    ("WhViBlGrYeOrReWh", "Diverging"),
];

const NCAR_CITATION: &str = "NCAR Command Language (NCL). doi.org/10.5065/D6WD3XH5";

const BASE_URL: &str =
    "https://raw.githubusercontent.com/NCAR/ncl/develop/ni/src/db/colormaps";

pub fn fetch(project_root: &Path) {
    println!("Fetching NCAR NCL colormaps from GitHub...");

    let data_dir = project_root.join("data").join("ncar");
    std::fs::create_dir_all(&data_dir).expect("create data/ncar/");

    for &(name, kind) in NCAR_MAPS {
        println!("  Fetching {name}...");

        let url = format!("{BASE_URL}/{name}.rgb");

        let text = match try_fetch_url_text(&url) {
            Some(t) => t,
            None => {
                eprintln!("  Warning: failed to download {name}.rgb (404?) -- skipping");
                continue;
            }
        };

        let lut = parse_rgb_file(&text);
        if lut.is_empty() {
            eprintln!("  Warning: {name}.rgb parsed to 0 colors -- skipping");
            continue;
        }

        let lut = if lut.len() == 256 {
            lut
        } else {
            println!("  Resampling {name} from {} to 256 entries", lut.len());
            codegen::resample(&lut, 256)
        };

        let meta = ColormapMeta {
            name: name.to_string(),
            collection: "ncar".to_string(),
            author: "NCAR".to_string(),
            kind: kind.to_string(),
            perceptually_uniform: false,
            cvd_friendly: false,
            grayscale_safe: false,
            citation: NCAR_CITATION.to_string(),
        };

        codegen::write_csv(&data_dir.join(format!("{name}.csv")), &lut);
        codegen::write_json(&data_dir.join(format!("{name}.json")), &meta);
        println!("  Wrote data/ncar/{name}.csv + .json ({kind})");
    }
}

/// Fetch a URL, returning `None` on any error (404, network, etc.)
/// instead of panicking like `codegen::fetch_url_text`.
fn try_fetch_url_text(url: &str) -> Option<String> {
    let mut response = ureq::get(url).call().ok()?;
    response.body_mut().read_to_string().ok()
}

/// Parse an NCAR `.rgb` file into a Vec of RGB u8 triplets.
///
/// Format:
///  - Lines starting with `#` are comments
///  - An optional `ncolors = N` header line
///  - Remaining non-empty, non-comment lines are space-separated R G B integers [0-255]
///  - If `ncolors` header was found, the first two color entries may be
///    background/foreground sentinel colors (black + white or white + black).
///    Strip them if both conditions are met.
fn parse_rgb_file(text: &str) -> Vec<[u8; 3]> {
    let mut has_ncolors = false;
    let mut colors: Vec<[u8; 3]> = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Check for ncolors header
        if trimmed.contains("ncolors") {
            // Parse "ncolors = N" -- we just need to know it exists
            let after_eq = trimmed.split('=').nth(1);
            if let Some(val) = after_eq
                && val.trim().parse::<usize>().is_ok()
            {
                has_ncolors = true;
            }
            continue;
        }

        // Parse R G B integers
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }
        let r: u8 = match parts[0].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let g: u8 = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let b: u8 = match parts[2].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        colors.push([r, g, b]);
    }

    // Strip background/foreground sentinel entries if ncolors header was present
    if has_ncolors && colors.len() >= 3 {
        let first = colors[0];
        let second = colors[1];
        let is_bw = (first == [0, 0, 0] && second == [255, 255, 255])
            || (first == [255, 255, 255] && second == [0, 0, 0]);
        if is_bw {
            colors.drain(..2);
        }
    }

    colors
}
