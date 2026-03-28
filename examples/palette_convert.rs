//! Demonstrate palette Srgb conversion with prismatica colormaps.
//!
//! Both prismatica::Color and palette::Srgb<u8> store sRGB as u8 channels,
//! so the round-trip is always perfectly lossless.
//!
//! Run with: `cargo run --example palette_convert --features "matplotlib,palette-integration"`

use palette::Srgb;
use prismatica::Color;
use prismatica::matplotlib::VIRIDIS;

fn main() {
    let cm = &VIRIDIS;
    let n = 10;

    println!("Colormap: {} ({})\n", cm.meta.name, cm.meta.collection);
    println!(
        "  {:<4} {:>3} {:>3} {:>3}   palette::Srgb<u8>",
        "i", "R", "G", "B"
    );
    println!("  {}", "-".repeat(50));

    for i in 0..n {
        let color = cm.eval_rational(i, n);

        // Forward: prismatica::Color -> palette::Srgb<u8>
        let srgb: Srgb<u8> = color.into();

        // Reverse: palette::Srgb<u8> -> prismatica::Color (both u8, perfect round-trip)
        let back: Color = srgb.into();

        let swatch = format!("\x1b[48;2;{};{};{}m  \x1b[0m", color.r, color.g, color.b);
        println!(
            "  {:<4} {:>3} {:>3} {:>3}   Srgb({:>3}, {:>3}, {:>3}) {}",
            i, color.r, color.g, color.b, srgb.red, srgb.green, srgb.blue, swatch
        );
        assert_eq!(color, back, "round-trip must be lossless at index {i}");
    }

    println!("\nAll {n} samples round-tripped perfectly (u8 -> u8, no precision loss).");
}
