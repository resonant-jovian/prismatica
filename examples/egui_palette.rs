//! Demonstrate egui Color32 conversion with prismatica colormaps.
//!
//! Converts VIRIDIS samples to `egui::Color32` and back, printing RGB values
//! and ANSI-colored swatches to show bidirectional conversion fidelity.
//!
//! Run with: `cargo run --example egui_palette --features "matplotlib,egui-integration"`

use egui::Color32;
use prismatica::Color;
use prismatica::matplotlib::VIRIDIS;

fn main() {
    let n = 8;

    println!(
        "Colormap: {} ({})\n",
        VIRIDIS.meta.name, VIRIDIS.meta.collection
    );
    println!(
        "  {:<4} {:<18} {:<18} {:<10}",
        "i", "prismatica RGB", "egui Color32", "roundtrip"
    );
    println!("  {}", "-".repeat(54));

    for i in 0..n {
        let t = i as f32 / (n - 1) as f32;
        let color: Color = VIRIDIS.eval(t);

        // Convert prismatica::Color -> egui::Color32
        let egui_color: Color32 = color.into();

        // Convert back: egui::Color32 -> prismatica::Color
        let roundtrip: Color = egui_color.into();

        let swatch = format!(
            "\x1b[48;2;{};{};{}m    \x1b[0m",
            egui_color.r(),
            egui_color.g(),
            egui_color.b()
        );

        let ok = if color == roundtrip { "ok" } else { "MISMATCH" };

        println!(
            "  {i:<4} ({r:>3},{g:>3},{b:>3})        ({er:>3},{eg:>3},{eb:>3})        {swatch} {ok}",
            i = i,
            r = color.r,
            g = color.g,
            b = color.b,
            er = egui_color.r(),
            eg = egui_color.g(),
            eb = egui_color.b(),
            swatch = swatch,
            ok = ok,
        );
    }

    // Show a continuous gradient using egui Color32 values
    println!("\nGradient (32 steps via egui::Color32):");
    print!("  ");
    for i in 0..32 {
        let t = i as f32 / 31.0;
        let c: Color32 = VIRIDIS.eval(t).into();
        print!("\x1b[48;2;{};{};{}m  \x1b[0m", c.r(), c.g(), c.b());
    }
    println!();
}
