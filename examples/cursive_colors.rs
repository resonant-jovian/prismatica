//! Demonstrate cursive theme color conversion with prismatica colormaps.
//!
//! Converts VIRIDIS colormap samples to `cursive_core::theme::Color::Rgb` and
//! shows the TryFrom reverse conversion (only the Rgb variant can round-trip).
//!
//! Run with: `cargo run --example cursive_colors --features "matplotlib,cursive-integration"`

use cursive_core::theme::Color as CursiveColor;
use prismatica::matplotlib::VIRIDIS;
use prismatica::Color;

fn main() {
    let cm = &VIRIDIS;
    let n = 8;

    println!("Colormap: {} ({})\n", cm.meta.name, cm.meta.collection);
    println!("  {:<4} {:>3} {:>3} {:>3}   cursive variant", "i", "R", "G", "B");
    println!("  {}", "-".repeat(44));

    for i in 0..n {
        let color = cm.eval_rational(i, n);

        // Forward: prismatica::Color -> cursive_core::theme::Color::Rgb
        let cursive: CursiveColor = color.into();

        // Reverse: TryFrom since only the Rgb variant converts back
        let back: Color = cursive.try_into().expect("Rgb variant always succeeds");

        let swatch = format!("\x1b[48;2;{};{};{}m  \x1b[0m", back.r, back.g, back.b);
        println!(
            "  {:<4} {:>3} {:>3} {:>3}   {:?} {}",
            i, color.r, color.g, color.b, cursive, swatch
        );
        assert_eq!(color, back, "round-trip failed at index {i}");
    }

    println!("\nAll {n} samples round-tripped through cursive_core::theme::Color::Rgb.");
}
