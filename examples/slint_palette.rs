//! Demonstrate slint Color conversion with prismatica colormaps.
//!
//! Converts VIRIDIS samples to `slint::Color` and verifies the u8 accessor
//! round-trip (slint::Color stores u8 internally, so conversion is lossless).
//!
//! Run with: `cargo run --example slint_palette --features "matplotlib,slint-integration"`

use prismatica::matplotlib::VIRIDIS;
use prismatica::Color;
use slint::Color as SlintColor;

fn main() {
    let cm = &VIRIDIS;
    let n = 8;

    println!("Colormap: {} ({})\n", cm.meta.name, cm.meta.collection);
    println!(
        "  {:<4} {:>3} {:>3} {:>3}   slint Color (u8 accessors)",
        "i", "R", "G", "B"
    );
    println!("  {}", "-".repeat(52));

    for i in 0..n {
        let color = cm.eval_rational(i, n);

        // Forward: prismatica::Color -> slint::Color (via from_rgb_u8)
        let slint_c: SlintColor = color.into();

        // Verify u8 accessors match the original values
        assert_eq!(slint_c.red(), color.r, "red mismatch at index {i}");
        assert_eq!(slint_c.green(), color.g, "green mismatch at index {i}");
        assert_eq!(slint_c.blue(), color.b, "blue mismatch at index {i}");

        // Reverse: slint::Color -> prismatica::Color
        let back: Color = slint_c.into();

        let swatch = format!("\x1b[48;2;{};{};{}m  \x1b[0m", color.r, color.g, color.b);
        println!(
            "  {:<4} {:>3} {:>3} {:>3}   red={:>3} green={:>3} blue={:>3} {}",
            i,
            color.r,
            color.g,
            color.b,
            slint_c.red(),
            slint_c.green(),
            slint_c.blue(),
            swatch
        );
        assert_eq!(color, back, "round-trip failed at index {i}");
    }

    println!("\nAll {n} samples round-tripped losslessly through slint::Color.");
}
