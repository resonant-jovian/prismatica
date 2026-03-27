use crate::codegen::{self, ColormapMeta};
use std::path::Path;

struct MorelandMapDef {
    slug: &'static str,
    display_name: &'static str,
    kind: &'static str,
}

const MORELAND_MAPS: &[MorelandMapDef] = &[
    MorelandMapDef {
        slug: "smooth-cool-warm",
        display_name: "smooth-cool-warm",
        kind: "Diverging",
    },
    MorelandMapDef {
        slug: "bent-cool-warm",
        display_name: "bent-cool-warm",
        kind: "Diverging",
    },
    MorelandMapDef {
        slug: "black-body",
        display_name: "black-body",
        kind: "Sequential",
    },
    MorelandMapDef {
        slug: "kindlmann",
        display_name: "kindlmann",
        kind: "Sequential",
    },
    MorelandMapDef {
        slug: "extended-kindlmann",
        display_name: "extended-kindlmann",
        kind: "Sequential",
    },
    MorelandMapDef {
        slug: "fast",
        display_name: "fast",
        kind: "Sequential",
    },
];

const MORELAND_CITATION: &str =
    "Moreland, K. (2016). Why We Use Bad Color Maps and What You Can Do About It.";

pub fn fetch(project_root: &Path) {
    println!("Fetching Moreland colormaps...");

    let data_dir = project_root.join("data").join("moreland");
    std::fs::create_dir_all(&data_dir).expect("create data/moreland/");

    for def in MORELAND_MAPS {
        println!("  Fetching {}...", def.display_name);

        let url = format!(
            "https://www.kennethmoreland.com/color-advice/{slug}/{slug}-table-byte-0256.csv",
            slug = def.slug
        );
        let text = codegen::fetch_url_text(&url);

        let lut = parse_moreland_csv(&text);
        assert_eq!(
            lut.len(),
            256,
            "{} has {} entries, expected 256",
            def.display_name,
            lut.len()
        );

        let meta = ColormapMeta {
            name: def.display_name.to_string(),
            collection: "moreland".to_string(),
            author: "Kenneth Moreland".to_string(),
            kind: def.kind.to_string(),
            perceptually_uniform: true,
            cvd_friendly: true,
            grayscale_safe: true,
            citation: MORELAND_CITATION.to_string(),
        };

        codegen::write_csv(&data_dir.join(format!("{}.csv", def.display_name)), &lut);
        codegen::write_json(&data_dir.join(format!("{}.json", def.display_name)), &meta);
        println!(
            "  Wrote data/moreland/{}.csv + .json ({})",
            def.display_name, def.kind
        );
    }
}

/// Parse a Moreland CSV file.
///
/// Format: header row `scalar,RGB_r,RGB_g,RGB_b,...`, then 256 data rows.
/// Columns 1, 2, 3 are integer R, G, B values in [0, 255].
fn parse_moreland_csv(text: &str) -> Vec<[u8; 3]> {
    text.lines()
        .skip(1) // skip header row
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let cols: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            assert!(
                cols.len() >= 4,
                "Moreland CSV line has fewer than 4 columns: {line}"
            );
            let r: u8 = cols[1]
                .parse()
                .unwrap_or_else(|e| panic!("parse R from '{}': {e}", cols[1]));
            let g: u8 = cols[2]
                .parse()
                .unwrap_or_else(|e| panic!("parse G from '{}': {e}", cols[2]));
            let b: u8 = cols[3]
                .parse()
                .unwrap_or_else(|e| panic!("parse B from '{}': {e}", cols[3]));
            [r, g, b]
        })
        .collect()
}
