mod cartocolors;
mod colorbrewer;
mod d3;

use crate::codegen;
use std::path::Path;
use std::process::Command;

/// Collections that use the standard generic generate path.
const GENERIC_COLLECTIONS: &[&str] = &[
    "crameri",
    "matplotlib",
    "cet",
    "cmocean",
    "moreland",
    "cmasher",
    "ncar",
];

pub fn generate_all(project_root: &Path) {
    let data_dir = project_root.join("data");

    for collection in GENERIC_COLLECTIONS {
        let col_data = data_dir.join(collection);
        if !col_data.exists() {
            eprintln!(
                "Warning: data/{collection}/ does not exist -- run `cargo xtask fetch` first"
            );
            continue;
        }
        codegen::generate_collection(project_root, collection, &col_data);
    }

    // Custom generators for collections with discrete palettes
    colorbrewer::generate(project_root);
    cartocolors::generate(project_root);
    d3::generate(project_root);

    rustfmt_generated(project_root);
    println!("Generate complete.");
}

fn rustfmt_generated(project_root: &Path) {
    let src_dir = project_root.join("src");
    let all_collections: Vec<&str> = GENERIC_COLLECTIONS
        .iter()
        .copied()
        .chain(["colorbrewer", "cartocolors", "d3"])
        .collect();

    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for collection in &all_collections {
        let col_dir = src_dir.join(collection);
        if let Ok(entries) = std::fs::read_dir(&col_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "rs").unwrap_or(false) {
                    files.push(path);
                }
            }
        }
    }

    if files.is_empty() {
        return;
    }

    let status = Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .args(&files)
        .status();

    match status {
        Ok(s) if s.success() => println!("  Formatted {} generated files", files.len()),
        Ok(s) => eprintln!("Warning: rustfmt exited with {s}"),
        Err(e) => eprintln!("Warning: could not run rustfmt: {e}"),
    }
}

pub fn generate_collection(project_root: &Path, name: &str) {
    match name {
        "all" => generate_all(project_root),
        "colorbrewer" => {
            colorbrewer::generate(project_root);
            rustfmt_generated(project_root);
            println!("Generate complete.");
        }
        "cartocolors" => {
            cartocolors::generate(project_root);
            rustfmt_generated(project_root);
            println!("Generate complete.");
        }
        "d3" => {
            d3::generate(project_root);
            rustfmt_generated(project_root);
            println!("Generate complete.");
        }
        _ => {
            let col_data = project_root.join("data").join(name);
            if !col_data.exists() {
                eprintln!(
                    "Warning: data/{name}/ does not exist -- run `cargo xtask fetch {name}` first"
                );
                return;
            }
            codegen::generate_collection(project_root, name, &col_data);
            rustfmt_generated(project_root);
            println!("Generate complete.");
        }
    }
}
