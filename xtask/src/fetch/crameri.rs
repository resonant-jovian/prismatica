use crate::codegen::{self, ColormapMeta};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::Path;

const CRAMERI_ZIP_URL: &str =
    "https://zenodo.org/records/8409685/files/ScientificColourMaps8.zip";

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

pub fn fetch(project_root: &Path) {
    println!("Fetching Crameri from Zenodo...");

    let data_dir = project_root.join("data").join("crameri");
    std::fs::create_dir_all(&data_dir).expect("create data/crameri/");

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
    // ScientificColourMaps8/{name}/{name}.txt
    let mut maps: BTreeMap<String, Vec<[u8; 3]>> = BTreeMap::new();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).expect("zip entry");
        let path = entry.name().to_string();

        // Match .txt files: look for {name}/{name}.txt pattern
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

        let lut = codegen::parse_space_separated_floats(&contents);
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

        codegen::write_csv(&data_dir.join(format!("{}.csv", name)), lut);
        codegen::write_json(&data_dir.join(format!("{}.json", name)), &meta);
        println!("  Wrote data/crameri/{}.csv + .json ({kind})", name);
    }
}
