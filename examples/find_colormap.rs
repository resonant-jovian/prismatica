//! Demonstrate colormap discovery: find_by_name, filter_by_kind, filter_by_collection.
//!
//! Run with: `cargo run --example find_colormap --features all`

use prismatica::prelude::*;

fn main() {
    // Look up a colormap by exact name
    println!("-- find_by_name --");
    for name in ["batlow", "viridis", "nonexistent"] {
        match find_by_name(name) {
            Some(cm) => println!(
                "  Found \"{}\": {:?}, collection={}, cvd_friendly={}",
                cm.meta.name, cm.meta.kind, cm.meta.collection, cm.meta.cvd_friendly,
            ),
            None => println!("  \"{}\" not found", name),
        }
    }

    // Filter by kind
    println!("\n-- filter_by_kind(Diverging) --");
    let diverging = filter_by_kind(ColormapKind::Diverging);
    println!("  {} diverging colormaps:", diverging.len());
    for cm in diverging.iter().take(8) {
        let c = cm.eval(0.5);
        print!("  \x1b[48;2;{};{};{}m  \x1b[0m {}", c.r, c.g, c.b, cm.meta.name);
        println!(" ({})", cm.meta.collection);
    }
    if diverging.len() > 8 {
        println!("  ... and {} more", diverging.len() - 8);
    }

    // Filter by collection
    println!("\n-- filter_by_collection(\"cmocean\") --");
    let cmocean = filter_by_collection("cmocean");
    println!("  {} cmocean colormaps:", cmocean.len());
    for cm in &cmocean {
        let c = cm.eval(0.5);
        print!("  \x1b[48;2;{};{};{}m  \x1b[0m", c.r, c.g, c.b);
        println!(" {:<14} {:?}", cm.meta.name, cm.meta.kind);
    }

    // Look up a discrete palette
    println!("\n-- find_palette_by_name --");
    if let Some(p) = find_palette_by_name("Set2") {
        print!("  {} ({} colors): ", p.meta.name, p.len());
        for i in 0..p.len() {
            let c = p.get(i);
            print!("\x1b[48;2;{};{};{}m  \x1b[0m", c.r, c.g, c.b);
        }
        println!();
    }
}
