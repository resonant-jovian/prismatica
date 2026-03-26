pub mod cartocolors;
pub mod cet;
pub mod cmasher;
pub mod cmocean;
pub mod colorbrewer;
pub mod crameri;
pub mod d3;
pub mod matplotlib;
pub mod moreland;
pub mod ncar;

use std::path::Path;

pub fn fetch_all(project_root: &Path) {
    crameri::fetch(project_root);
    matplotlib::fetch(project_root);
    cet::fetch(project_root);
    cmocean::fetch(project_root);
    moreland::fetch(project_root);
    cmasher::fetch(project_root);
    colorbrewer::fetch(project_root);
    cartocolors::fetch(project_root);
    ncar::fetch(project_root);
    d3::fetch(project_root);
    println!("Fetch complete.");
}

pub fn fetch_collection(project_root: &Path, name: &str) {
    match name {
        "crameri" => crameri::fetch(project_root),
        "matplotlib" => matplotlib::fetch(project_root),
        "cet" => cet::fetch(project_root),
        "cmocean" => cmocean::fetch(project_root),
        "moreland" => moreland::fetch(project_root),
        "cmasher" => cmasher::fetch(project_root),
        "colorbrewer" => colorbrewer::fetch(project_root),
        "cartocolors" => cartocolors::fetch(project_root),
        "ncar" => ncar::fetch(project_root),
        "d3" => d3::fetch(project_root),
        "all" => fetch_all(project_root),
        other => {
            eprintln!("Unknown collection: {other}");
            eprintln!("Available: crameri, matplotlib, cet, cmocean, moreland, cmasher, colorbrewer, cartocolors, ncar, all");
            std::process::exit(1);
        }
    }
}
