//! Demonstrate iced Color conversion with prismatica colormaps.
//!
//! Converts VIRIDIS samples to `iced_core::Color` (f32 RGBA) and shows the
//! round-trip back to prismatica::Color. The f32 -> u8 -> f32 conversion
//! introduces tiny rounding, but u8 values always match exactly.
//!
//! Run with: `cargo run --example iced_colors --features "matplotlib,iced-integration"`

use iced_core::Color as IcedColor;
use prismatica::matplotlib::VIRIDIS;
use prismatica::Color;

fn main() {
    let cm = &VIRIDIS;
    let n = 8;

    println!("Colormap: {} ({})\n", cm.meta.name, cm.meta.collection);
    println!(
        "  {:<4} {:>3} {:>3} {:>3}   iced Color (f32 rgba)",
        "i", "R", "G", "B"
    );
    println!("  {}", "-".repeat(56));

    for i in 0..n {
        let color = cm.eval_rational(i, n);

        // Forward: prismatica::Color -> iced_core::Color (f32 with alpha = 1.0)
        let iced: IcedColor = color.into();

        // Reverse: iced_core::Color -> prismatica::Color (alpha discarded)
        let back: Color = iced.into();

        let swatch = format!("\x1b[48;2;{};{};{}m  \x1b[0m", color.r, color.g, color.b);
        println!(
            "  {:<4} {:>3} {:>3} {:>3}   ({:.4}, {:.4}, {:.4}, {:.1}) {}",
            i, color.r, color.g, color.b, iced.r, iced.g, iced.b, iced.a, swatch
        );
        assert_eq!(color, back, "u8 round-trip failed at index {i}");
    }

    println!("\nAll {n} samples round-tripped through iced_core::Color.");
}
