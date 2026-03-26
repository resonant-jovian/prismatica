mod colorbrewer;
mod cartocolors;

use crate::codegen;
use std::path::Path;

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

    println!("Generate complete.");
}

pub fn generate_collection(project_root: &Path, name: &str) {
    match name {
        "all" => generate_all(project_root),
        "colorbrewer" => {
            colorbrewer::generate(project_root);
            println!("Generate complete.");
        }
        "cartocolors" => {
            cartocolors::generate(project_root);
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
            println!("Generate complete.");
        }
    }
}
