//! Print a colormap gradient using crossterm Rgb colors.
//!
//! Uses crossterm's `SetForegroundColor` and `SetBackgroundColor` commands
//! to render colored blocks, demonstrating how prismatica integrates with
//! crossterm's styling system.
//!
//! Run with: `cargo run --example crossterm_gradient --features "matplotlib,crossterm-integration"`

use crossterm::style::{Color as CtColor, ResetColor, SetBackgroundColor, SetForegroundColor};
use prismatica::matplotlib::VIRIDIS;

fn main() {
    let n = 48;

    println!(
        "Colormap: {} ({})\n",
        VIRIDIS.meta.name, VIRIDIS.meta.collection
    );

    // Background-colored gradient
    println!("Background gradient ({n} steps):");
    print!("  ");
    for i in 0..n {
        let t = i as f32 / (n - 1) as f32;
        let color: CtColor = VIRIDIS.eval(t).into();
        print!("{}  {}", SetBackgroundColor(color), ResetColor);
    }
    println!();

    // Foreground-colored text
    println!("\nForeground-colored text:");
    let message = "The quick brown fox jumps over the lazy dog";
    print!("  ");
    for (i, ch) in message.chars().enumerate() {
        let t = i as f32 / (message.len() - 1) as f32;
        let color: CtColor = VIRIDIS.eval(t).into();
        print!("{}{ch}{}", SetForegroundColor(color), ResetColor);
    }
    println!();

    // Combined foreground + background blocks with RGB values
    println!("\nSample values:");
    println!("  {:<4} {:<8} {:<18} swatch", "i", "t", "RGB");
    println!("  {}", "-".repeat(44));
    for i in 0..8 {
        let t = i as f32 / 7.0;
        let pc = VIRIDIS.eval(t);
        let color: CtColor = pc.into();
        print!(
            "  {i:<4} {t:<8.4} ({r:>3},{g:>3},{b:>3})      ",
            i = i,
            t = t,
            r = pc.r,
            g = pc.g,
            b = pc.b,
        );
        print!("{}    {}", SetBackgroundColor(color), ResetColor);
        println!();
    }
}
