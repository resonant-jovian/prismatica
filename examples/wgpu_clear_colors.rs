//! Generate wgpu clear colors from a colormap.
//!
//! Converts VIRIDIS samples to `wgpu_types::Color` (f64 RGBA) suitable for
//! use as render pass clear colors, and shows the round-trip conversion.
//!
//! Run with: `cargo run --example wgpu_clear_colors --features "matplotlib,wgpu-integration"`

use prismatica::matplotlib::VIRIDIS;
use prismatica::Color;
use wgpu_types::Color as WgpuColor;

fn main() {
    let cm = &VIRIDIS;
    let n = 8;

    println!("Colormap: {} ({})\n", cm.meta.name, cm.meta.collection);
    println!(
        "  {:<4} {:>3} {:>3} {:>3}   wgpu clear color (f64 rgba)",
        "i", "R", "G", "B"
    );
    println!("  {}", "-".repeat(62));

    for i in 0..n {
        let color = cm.eval_rational(i, n);

        // Forward: prismatica::Color -> wgpu_types::Color (f64 with alpha = 1.0)
        let wgpu: WgpuColor = color.into();

        // Reverse: wgpu_types::Color -> prismatica::Color (alpha discarded)
        let back: Color = wgpu.into();

        let swatch = format!("\x1b[48;2;{};{};{}m  \x1b[0m", color.r, color.g, color.b);
        println!(
            "  {:<4} {:>3} {:>3} {:>3}   ({:.6}, {:.6}, {:.6}, {:.1}) {}",
            i, color.r, color.g, color.b, wgpu.r, wgpu.g, wgpu.b, wgpu.a, swatch
        );
        assert_eq!(color, back, "round-trip failed at index {i}");
    }

    // Show how you might use these as clear colors in a render pass
    println!("\nExample wgpu render pass descriptor usage:");
    let clear = WgpuColor::from(cm.eval(0.5));
    println!(
        "  color_attachments: load: Clear(Color {{ r: {:.6}, g: {:.6}, b: {:.6}, a: {:.1} }})",
        clear.r, clear.g, clear.b, clear.a
    );
}
