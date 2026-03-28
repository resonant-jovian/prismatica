//! Color text using the colored crate with prismatica colormaps.
//!
//! Applies VIRIDIS-derived `colored::Color::TrueColor` values to text
//! using the `colored::Colorize` trait for styled terminal output.
//!
//! Run with: `cargo run --example colored_text --features "matplotlib,colored-integration"`

use colored::Colorize;
use prismatica::matplotlib::VIRIDIS;

fn main() {
    println!(
        "Colormap: {} ({})\n",
        VIRIDIS.meta.name, VIRIDIS.meta.collection
    );

    // Color each word of a sentence along the colormap
    println!("Colored words:");
    let words: Vec<&str> =
        "Prismatica provides 308 scientific colormaps as compile-time Rust constants"
            .split_whitespace()
            .collect();
    print!("  ");
    for (i, word) in words.iter().enumerate() {
        let t = i as f32 / (words.len() - 1) as f32;
        let color: colored::Color = VIRIDIS.eval(t).into();
        print!("{} ", word.color(color));
    }
    println!();

    // Show bold colored text on colored background
    println!("\nBold text with colored backgrounds:");
    for i in 0..8 {
        let t = i as f32 / 7.0;
        let color: colored::Color = VIRIDIS.eval(t).into();
        let label = format!("  t = {t:.2}  ");
        println!("  {}", label.bold().on_color(color));
    }

    // Rainbow-style character coloring
    println!("\nPer-character gradient:");
    let text = "Scientific visualization with prismatica colormaps";
    print!("  ");
    for (i, ch) in text.chars().enumerate() {
        let t = i as f32 / (text.len() - 1) as f32;
        let color: colored::Color = VIRIDIS.eval(t).into();
        print!("{}", ch.to_string().color(color).bold());
    }
    println!();

    // Show gradient blocks with color values
    println!("\nGradient blocks:");
    print!("  ");
    for i in 0..32 {
        let t = i as f32 / 31.0;
        let color: colored::Color = VIRIDIS.eval(t).into();
        print!("{}", "  ".on_color(color));
    }
    println!();
}
