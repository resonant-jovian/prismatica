//! Demonstrate bevy_color Srgba conversion with prismatica colormaps.
//!
//! Converts VIRIDIS samples to `bevy_color::Srgba` (f32 channels + alpha)
//! and shows the round-trip back to prismatica::Color.
//!
//! Run with: `cargo run --example bevy_color_srgba --features "matplotlib,bevy-color-integration"`

use bevy_color::Srgba;
use prismatica::Color;
use prismatica::matplotlib::VIRIDIS;

fn main() {
    let cm = &VIRIDIS;
    let n = 8;

    println!("Colormap: {} ({})\n", cm.meta.name, cm.meta.collection);
    println!(
        "  {:<4} {:>3} {:>3} {:>3}   bevy Srgba (f32)",
        "i", "R", "G", "B"
    );
    println!("  {}", "-".repeat(56));

    for i in 0..n {
        let color = cm.eval_rational(i, n);

        // Forward: prismatica::Color -> bevy_color::Srgba (u8 -> f32, alpha = 1.0)
        let bevy: Srgba = color.into();

        // Reverse: bevy_color::Srgba -> prismatica::Color (f32 -> u8, alpha discarded)
        let back: Color = bevy.into();

        let swatch = format!("\x1b[48;2;{};{};{}m  \x1b[0m", color.r, color.g, color.b);
        println!(
            "  {:<4} {:>3} {:>3} {:>3}   ({:.4}, {:.4}, {:.4}, {:.1}) {}",
            i, color.r, color.g, color.b, bevy.red, bevy.green, bevy.blue, bevy.alpha, swatch
        );
        assert_eq!(color, back, "round-trip failed at index {i}");
    }

    println!("\nAll {n} samples round-tripped through bevy_color::Srgba.");
}
