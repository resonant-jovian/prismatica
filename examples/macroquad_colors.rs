//! Demonstrate macroquad color conversion with prismatica colormaps.
//!
//! Converts VIRIDIS samples to `macroquad::color::Color` (f32 RGBA) and
//! shows the round-trip back to prismatica::Color.
//!
//! Run with: `cargo run --example macroquad_colors --features "matplotlib,macroquad-integration"`

use macroquad::color::Color as MqColor;
use prismatica::matplotlib::VIRIDIS;
use prismatica::Color;

fn main() {
    let cm = &VIRIDIS;
    let n = 8;

    println!("Colormap: {} ({})\n", cm.meta.name, cm.meta.collection);
    println!(
        "  {:<4} {:>3} {:>3} {:>3}   macroquad RGBA (f32)",
        "i", "R", "G", "B"
    );
    println!("  {}", "-".repeat(56));

    for i in 0..n {
        let color = cm.eval_rational(i, n);

        // Forward: prismatica::Color -> macroquad::color::Color (alpha = 1.0)
        let mq: MqColor = color.into();

        // Reverse: macroquad::color::Color -> prismatica::Color (alpha discarded)
        let back: Color = mq.into();

        let swatch = format!("\x1b[48;2;{};{};{}m  \x1b[0m", color.r, color.g, color.b);
        println!(
            "  {:<4} {:>3} {:>3} {:>3}   ({:.4}, {:.4}, {:.4}, {:.1}) {}",
            i, color.r, color.g, color.b, mq.r, mq.g, mq.b, mq.a, swatch
        );
        assert_eq!(color, back, "round-trip failed at index {i}");
    }

    println!("\nAll {n} samples round-tripped through macroquad::color::Color.");
}
