//! Apply colormap colors as syntect highlighting colors.
//!
//! Shows how prismatica colors convert to syntect's RGBA color type.
//! Forward conversion sets alpha to 255 (fully opaque); reverse discards alpha.
//!
//! Run with: `cargo run --example syntect_highlight --features "matplotlib,syntect-integration"`

use prismatica::Color;
use prismatica::matplotlib::VIRIDIS;
use syntect::highlighting::Color as SyntectColor;

fn main() {
    let cm = &VIRIDIS;
    let n = 8;

    println!("Colormap: {} ({})\n", cm.meta.name, cm.meta.collection);
    println!(
        "  {:<4} {:>3} {:>3} {:>3}   syntect RGBA",
        "i", "R", "G", "B"
    );
    println!("  {}", "-".repeat(48));

    for i in 0..n {
        let color = cm.eval_rational(i, n);

        // Forward: prismatica::Color -> syntect Color (alpha set to 255)
        let syn: SyntectColor = color.into();
        assert_eq!(syn.a, 255, "forward conversion should set alpha to 255");

        // Reverse: syntect Color -> prismatica::Color (alpha discarded)
        let back: Color = syn.into();

        let swatch = format!("\x1b[48;2;{};{};{}m  \x1b[0m", color.r, color.g, color.b);
        println!(
            "  {:<4} {:>3} {:>3} {:>3}   rgba({:>3}, {:>3}, {:>3}, {:>3}) {}",
            i, color.r, color.g, color.b, syn.r, syn.g, syn.b, syn.a, swatch
        );
        assert_eq!(color, back, "round-trip failed at index {i}");
    }

    // Demonstrate that a semi-transparent syntect color also converts back
    let semi = SyntectColor {
        r: 68,
        g: 1,
        b: 84,
        a: 128,
    };
    let from_semi: Color = semi.into();
    println!(
        "\n  Semi-transparent syntect rgba({}, {}, {}, {}) -> prismatica ({}, {}, {})",
        semi.r, semi.g, semi.b, semi.a, from_semi.r, from_semi.g, from_semi.b,
    );
    println!("  (alpha channel is discarded on reverse conversion)");
}
