use crate::codegen::{self, ColormapData, ColormapMeta, PaletteData};
use std::fs;
use std::path::Path;

pub fn generate(project_root: &Path) {
    let data_dir = project_root.join("data").join("colorbrewer");
    if !data_dir.exists() {
        eprintln!(
            "Warning: data/colorbrewer/ does not exist -- run `cargo xtask fetch colorbrewer` first"
        );
        return;
    }

    println!("Generating colorbrewer...");

    let src_dir = project_root.join("src").join("colorbrewer");
    fs::create_dir_all(&src_dir).expect("create src/colorbrewer/");

    // Discover all colormaps by finding .json files (excluding _palette.csv)
    let mut json_files: Vec<_> = fs::read_dir(&data_dir)
        .expect("read data dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "json").unwrap_or(false))
        .collect();
    json_files.sort_by_key(|e| e.file_name());

    let mut colormap_entries: Vec<(String, String, String)> = Vec::new(); // (name, mod_name, const_name)
    let mut palette_entries: Vec<(String, String, String)> = Vec::new();

    for entry in &json_files {
        let json_path = entry.path();
        let stem = json_path
            .file_stem()
            .expect("JSON file should have a stem")
            .to_string_lossy()
            .to_string();

        let json_text = fs::read_to_string(&json_path).expect("read JSON");
        let meta: ColormapMeta = serde_json::from_str(&json_text).expect("parse JSON");

        let mod_name = codegen::to_mod_name(&meta.name);
        let const_name = codegen::to_const_name(&meta.name);

        // Read the 256-entry resampled CSV
        let csv_path = data_dir.join(format!("{stem}.csv"));
        let csv_text = fs::read_to_string(&csv_path).expect("read CSV");
        let lut = parse_csv_lut(&csv_text);

        // Read the discrete palette CSV
        let palette_path = data_dir.join(format!("{stem}_palette.csv"));
        let palette_text = fs::read_to_string(&palette_path).expect("read palette CSV");
        let palette_colors = parse_csv_lut(&palette_text);

        let map = ColormapData {
            meta: meta.clone(),
            lut,
        };
        let palette = PaletteData {
            meta,
            colors: palette_colors,
        };

        // Generate combined .rs file with both Colormap and DiscretePalette
        let code = codegen::generate_dual_colormap_rs(&map, &palette, &const_name);
        let rs_path = src_dir.join(format!("{mod_name}.rs"));
        fs::write(&rs_path, code).expect("write .rs");
        println!("  Generating colorbrewer/{mod_name}.rs ({})", map.meta.kind);

        colormap_entries.push((map.meta.name.clone(), mod_name.clone(), const_name.clone()));
        palette_entries.push((map.meta.name.clone(), mod_name, const_name));
    }

    // Sort entries
    colormap_entries.sort_by(|a, b| a.1.cmp(&b.1));
    palette_entries.sort_by(|a, b| a.1.cmp(&b.1));

    // Generate mod.rs with both ALL and ALL_DISCRETE
    let mod_rs_path = src_dir.join("mod.rs");
    let existing_doc_comments = codegen::read_doc_comments(&mod_rs_path);

    let maps_ref: Vec<(&str, String, String)> = colormap_entries
        .iter()
        .map(|(n, m, c)| (n.as_str(), m.clone(), c.clone()))
        .collect();
    let palettes_ref: Vec<(&str, String, String)> = palette_entries
        .iter()
        .map(|(n, m, c)| (n.as_str(), m.clone(), c.clone()))
        .collect();

    let mod_code = codegen::generate_mod_rs_with_palettes(
        "colorbrewer",
        &existing_doc_comments,
        &maps_ref,
        &palettes_ref,
    );
    fs::write(&mod_rs_path, mod_code).expect("write mod.rs");
    println!(
        "  Generated colorbrewer/mod.rs ({} maps, {} palettes)",
        colormap_entries.len(),
        palette_entries.len()
    );
}

fn parse_csv_lut(text: &str) -> Vec<[u8; 3]> {
    text.lines()
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
        .collect()
}
