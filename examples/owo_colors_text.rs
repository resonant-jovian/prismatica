//! Color text using owo-colors with prismatica colormaps.
//!
//! Applies VIRIDIS-derived `owo_colors::Rgb` values to text using the
//! `owo_colors::OwoColorize` trait for efficient styled terminal output.
//!
//! Run with: `cargo run --example owo_colors_text --features "matplotlib,owo-colors-integration"`

use owo_colors::OwoColorize;
use prismatica::matplotlib::VIRIDIS;

fn main() {
    println!("Colormap: {} ({})\n", VIRIDIS.meta.name, VIRIDIS.meta.collection);

    // Per-character gradient text
    println!("Per-character gradient:");
    let text = "Perceptually uniform colormaps for scientific visualization";
    print!("  ");
    for (i, ch) in text.chars().enumerate() {
        let t = i as f32 / (text.len() - 1) as f32;
        let rgb: owo_colors::Rgb = VIRIDIS.eval(t).into();
        print!("{}", ch.color(rgb));
    }
    println!();

    // Background-colored blocks
    println!("\nBackground gradient (32 steps):");
    print!("  ");
    for i in 0..32 {
        let t = i as f32 / 31.0;
        let rgb: owo_colors::Rgb = VIRIDIS.eval(t).into();
        print!("{}", "  ".on_color(rgb));
    }
    println!();

    // Labeled color swatches
    println!("\nLabeled swatches:");
    for i in 0..8 {
        let t = i as f32 / 7.0;
        let pc = VIRIDIS.eval(t);
        let rgb: owo_colors::Rgb = pc.into();
        let label = format!(" t={t:.2} ({:>3},{:>3},{:>3}) ", pc.r, pc.g, pc.b);
        println!("  {}", label.on_color(rgb).bold());
    }

    // Show both foreground and background coloring
    println!("\nForeground vs background:");
    for i in 0..6 {
        let t = i as f32 / 5.0;
        let rgb: owo_colors::Rgb = VIRIDIS.eval(t).into();
        let fg_text = format!(" Foreground at t={t:.2} ");
        let bg_text = format!(" Background at t={t:.2} ");
        println!(
            "  {}  {}",
            fg_text.color(rgb).bold(),
            bg_text.on_color(rgb),
        );
    }
}
