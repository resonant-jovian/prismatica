//! Export discrete palette colors as CSS hex values, and extract N colors from a continuous colormap.
//!
//! Run with: `cargo run --example export_palette --features all`

use prismatica::prelude::*;

fn main() {
    // Export a discrete palette (ColorBrewer Set2) as CSS hex
    println!("-- Discrete palette: ColorBrewer Set2 --");
    let palette = find_palette_by_name("Set2").expect("Set2 palette not found");
    println!("  {} colors from \"{}\":\n", palette.len(), palette.meta.name);
    for i in 0..palette.len() {
        let c = palette.get(i);
        println!(
            "  \x1b[48;2;{};{};{}m    \x1b[0m  {}",
            c.r, c.g, c.b,
            c.to_css_hex(),
        );
    }

    // Extract N discrete colors from a continuous colormap
    println!("\n-- 6 colors from continuous colormap \"batlow\" --\n");
    let cm = find_by_name("batlow").expect("batlow not found");
    let colors = cm.colors(6);
    for c in &colors {
        println!(
            "  \x1b[48;2;{};{};{}m    \x1b[0m  {}",
            c.r, c.g, c.b,
            c.to_css_hex(),
        );
    }

    // CSS custom properties output
    println!("\n-- CSS custom properties (Set2) --\n");
    println!(":root {{");
    for i in 0..palette.len() {
        let c = palette.get(i);
        println!("  --palette-{}: {};", i, c.to_css_hex());
    }
    println!("}}");
}
