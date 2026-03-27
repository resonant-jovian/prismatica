//! List every available colormap with its kind, collection, and properties.
//!
//! Run with: `cargo run --example list_all --features all`

use prismatica::prelude::*;

fn main() {
    let colormaps = all_colormaps();
    let palettes = all_discrete_palettes();

    println!(
        "{} colormaps + {} discrete palettes available:\n",
        colormaps.len(),
        palettes.len()
    );
    println!(
        "  {name:<24} {kind:<16} {collection:<14} {pu:<4} {cvd:<4}",
        name = "NAME",
        kind = "KIND",
        collection = "COLLECTION",
        pu = "PU",
        cvd = "CVD",
    );
    println!("  {}", "-".repeat(66));

    for cm in &colormaps {
        let m = &cm.meta;
        println!(
            "  {name:<24} {kind:<16} {collection:<14} {pu:<4} {cvd:<4}",
            name = m.name,
            kind = format!("{:?}", m.kind),
            collection = m.collection,
            pu = if m.perceptually_uniform { "yes" } else { "-" },
            cvd = if m.cvd_friendly { "yes" } else { "-" },
        );
    }

    println!("\nDiscrete palettes:");
    println!(
        "  {name:<24} {n:<6} {collection:<14}",
        name = "NAME",
        n = "COLORS",
        collection = "COLLECTION"
    );
    println!("  {}", "-".repeat(46));

    for p in &palettes {
        println!(
            "  {name:<24} {n:<6} {collection:<14}",
            name = p.meta.name,
            n = p.len(),
            collection = p.meta.collection,
        );
    }
}
