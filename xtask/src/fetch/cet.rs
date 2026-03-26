use crate::codegen::{self, ColormapMeta};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::Path;

const CET_ZIP_URL: &str =
    "https://colorcet.com/download/CETperceptual_csv_0_1.zip";

fn cet_kind(name: &str) -> &'static str {
    // Check CET-CB* before CET-C* to avoid misclassifying colorblind maps as Cyclic
    if name.starts_with("CET-CB") {
        // CET-CBD = colorblind diverging, CET-CBC = colorblind cyclic, CET-CBL = colorblind linear
        if name.starts_with("CET-CBD") {
            "Diverging"
        } else if name.starts_with("CET-CBC") {
            "Cyclic"
        } else {
            "Sequential"
        }
    } else if name.starts_with("CET-D") {
        "Diverging"
    } else if name.starts_with("CET-C") {
        "Cyclic"
    } else {
        // CET-L, CET-R, CET-I, and anything else are Sequential
        "Sequential"
    }
}

fn cet_cvd_friendly(name: &str) -> bool {
    name.starts_with("CET-CB")
}

pub fn fetch(project_root: &Path) {
    println!("Fetching CET from Zenodo...");

    let data_dir = project_root.join("data").join("cet");
    std::fs::create_dir_all(&data_dir).expect("create data/cet/");

    // Download ZIP into memory
    let mut buf: Vec<u8> = Vec::new();
    ureq::get(CET_ZIP_URL)
        .call()
        .expect("download CET ZIP")
        .body_mut()
        .as_reader()
        .read_to_end(&mut buf)
        .expect("read CET ZIP body");

    println!("  Downloaded {} bytes", buf.len());

    let reader = std::io::Cursor::new(buf);
    let mut archive = zip::ZipArchive::new(reader).expect("open ZIP archive");

    let mut maps: BTreeMap<String, Vec<[u8; 3]>> = BTreeMap::new();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).expect("zip entry");
        let path = entry.name().to_string();

        // We want CSV files named like CET-L1.csv, CET-D1.csv, etc.
        let file_name = match path.rsplit('/').next() {
            Some(f) => f.to_string(),
            None => continue,
        };

        if !file_name.ends_with(".csv") || !file_name.starts_with("CET-") {
            continue;
        }

        let name = file_name.trim_end_matches(".csv").to_string();

        let mut contents = String::new();
        entry
            .read_to_string(&mut contents)
            .expect("read CSV from ZIP");

        // Parse CSV: 256 rows of comma-separated floats [0,1]
        let lut: Vec<[u8; 3]> = contents
            .lines()
            .filter(|l| !l.trim().is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                if parts.len() < 3 {
                    return None;
                }
                let r: f64 = parts[0].parse().ok()?;
                let g: f64 = parts[1].parse().ok()?;
                let b: f64 = parts[2].parse().ok()?;
                Some(codegen::float_to_u8(r, g, b))
            })
            .collect();

        if lut.len() != 256 {
            eprintln!(
                "  Warning: {} has {} rows, expected 256 -- skipping",
                name,
                lut.len()
            );
            continue;
        }

        maps.insert(name, lut);
    }

    println!("  Found {} colormaps in archive", maps.len());

    for (name, lut) in &maps {
        let kind = cet_kind(name);
        let meta = ColormapMeta {
            name: name.clone(),
            collection: "cet".to_string(),
            author: "Peter Kovesi".to_string(),
            kind: kind.to_string(),
            perceptually_uniform: true,
            cvd_friendly: cet_cvd_friendly(name),
            grayscale_safe: false,
            citation:
                "Kovesi, P. (2015). Good Colour Maps: How to Design Them. arXiv:1509.03700"
                    .to_string(),
        };

        codegen::write_csv(&data_dir.join(format!("{}.csv", name)), lut);
        codegen::write_json(&data_dir.join(format!("{}.json", name)), &meta);
        println!("  Wrote data/cet/{}.csv + .json ({kind})", name);
    }
}
