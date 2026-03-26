use crate::codegen::{self, ColormapMeta};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::Path;

const CMASHER_ZIP_URL: &str =
    "https://github.com/1313e/CMasher/archive/refs/heads/master.zip";

const CMASHER_DIVERGING: &[&str] = &[
    "fusion",
    "waterlily",
    "watermelon",
    "wildfire",
    "iceburn",
    "redshift",
    "guppy",
    "holly",
];

const CMASHER_CITATION: &str = "van der Velden, E. (2020). CMasher: Scientific colormaps for making accessible, informative and 'cmashing' plots. JOSS, 5(46), 2004.";

fn cmasher_kind(name: &str) -> &'static str {
    if CMASHER_DIVERGING.contains(&name) {
        "Diverging"
    } else {
        "Sequential"
    }
}

pub fn fetch(project_root: &Path) {
    println!("Fetching CMasher from GitHub...");

    let data_dir = project_root.join("data").join("cmasher");
    std::fs::create_dir_all(&data_dir).expect("create data/cmasher/");

    // Download ZIP into memory
    let mut buf: Vec<u8> = Vec::new();
    ureq::get(CMASHER_ZIP_URL)
        .call()
        .expect("download CMasher ZIP")
        .body_mut()
        .as_reader()
        .read_to_end(&mut buf)
        .expect("read CMasher ZIP body");

    println!("  Downloaded {} bytes", buf.len());

    let reader = std::io::Cursor::new(buf);
    let mut archive = zip::ZipArchive::new(reader).expect("open ZIP archive");

    // Scan the archive for colormap data files.
    // Prefer {name}_8bit.txt (u8 integers), fall back to cm_{name}.txt (floats).
    // Key: colormap name, Value: parsed LUT.
    let mut u8_files: BTreeMap<String, String> = BTreeMap::new();
    let mut float_files: BTreeMap<String, String> = BTreeMap::new();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).expect("zip entry");
        let path = entry.name().to_string();

        // We expect paths like:
        //   CMasher-master/cmasher/colormaps/{name}/{name}_8bit.txt
        //   CMasher-master/cmasher/colormaps/{name}/cm_{name}.txt
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() < 5 {
            continue;
        }

        // Verify structure: .../cmasher/colormaps/{name}/{file}
        if parts[parts.len() - 4] != "cmasher" || parts[parts.len() - 3] != "colormaps" {
            continue;
        }

        let dir_name = parts[parts.len() - 2];
        let file_name = parts[parts.len() - 1];

        // Skip reversed variants
        if dir_name.ends_with("_r") {
            continue;
        }

        let expected_8bit = format!("{}_8bit.txt", dir_name);
        let expected_float = format!("cm_{}.txt", dir_name);

        if file_name == expected_8bit || file_name == expected_float {
            let mut contents = String::new();
            entry
                .read_to_string(&mut contents)
                .expect("read txt from ZIP");

            if file_name == expected_8bit {
                u8_files.insert(dir_name.to_string(), contents);
            } else {
                float_files.insert(dir_name.to_string(), contents);
            }
        }
    }

    // Merge: prefer u8 files, fall back to float files
    let mut maps: BTreeMap<String, Vec<[u8; 3]>> = BTreeMap::new();

    for (name, contents) in &u8_files {
        let lut = codegen::parse_space_separated_u8(contents);
        if lut.is_empty() {
            eprintln!("  Warning: {name}_8bit.txt produced 0 rows -- skipping");
            continue;
        }
        let lut = if lut.len() == 256 {
            lut
        } else {
            eprintln!(
                "  Note: {name}_8bit.txt has {} rows, resampling to 256",
                lut.len()
            );
            codegen::resample(&lut, 256)
        };
        maps.insert(name.clone(), lut);
    }

    for (name, contents) in &float_files {
        if maps.contains_key(name) {
            // Already have u8 version
            continue;
        }
        let lut = codegen::parse_space_separated_floats(contents);
        if lut.is_empty() {
            eprintln!("  Warning: cm_{name}.txt produced 0 rows -- skipping");
            continue;
        }
        let lut = if lut.len() == 256 {
            lut
        } else {
            eprintln!(
                "  Note: cm_{name}.txt has {} rows, resampling to 256",
                lut.len()
            );
            codegen::resample(&lut, 256)
        };
        maps.insert(name.clone(), lut);
    }

    println!("  Found {} colormaps in archive", maps.len());

    for (name, lut) in &maps {
        let kind = cmasher_kind(name);
        let meta = ColormapMeta {
            name: name.clone(),
            collection: "cmasher".to_string(),
            author: "Ellert van der Velden".to_string(),
            kind: kind.to_string(),
            perceptually_uniform: true,
            cvd_friendly: true,
            grayscale_safe: false,
            citation: CMASHER_CITATION.to_string(),
        };

        codegen::write_csv(&data_dir.join(format!("{}.csv", name)), lut);
        codegen::write_json(&data_dir.join(format!("{}.json", name)), &meta);
        println!("  Wrote data/cmasher/{}.csv + .json ({kind})", name);
    }
}
