//! Print a colormap gradient using ratatui Rgb colors.
//!
//! Converts prismatica colors to `ratatui::style::Color::Rgb` and renders
//! ANSI-colored blocks to the terminal, simulating how a ratatui TUI would
//! display colormap-derived styles.
//!
//! Run with: `cargo run --example ratatui_gradient --features "matplotlib,ratatui-integration"`

use prismatica::matplotlib::VIRIDIS;
use ratatui::style::Color as RatatuiColor;

fn main() {
    let n = 48;

    println!(
        "Colormap: {} ({})\n",
        VIRIDIS.meta.name, VIRIDIS.meta.collection
    );

    // Render a full-width gradient bar
    println!("Gradient ({n} steps):");
    print!("  ");
    for i in 0..n {
        let t = i as f32 / (n - 1) as f32;
        let color: RatatuiColor = VIRIDIS.eval(t).into();
        if let RatatuiColor::Rgb(r, g, b) = color {
            print!("\x1b[48;2;{r};{g};{b}m \x1b[0m");
        }
    }
    println!();

    // Show discrete palette blocks with labels
    println!("\nDiscrete samples (8 steps):");
    for i in 0..8 {
        let color: RatatuiColor = VIRIDIS.eval_rational(i, 8).into();
        if let RatatuiColor::Rgb(r, g, b) = color {
            let block = format!("\x1b[48;2;{r};{g};{b}m      \x1b[0m");
            println!("  {block}  step {i}/7  Rgb({r}, {g}, {b})");
        }
    }

    // Demonstrate a vertical bar chart effect
    println!("\nSimulated bar chart:");
    let values = [0.1, 0.35, 0.6, 0.85, 0.95, 0.7, 0.4, 0.2];
    let max_width = 40;
    for (i, &val) in values.iter().enumerate() {
        let color: RatatuiColor = VIRIDIS.eval(val).into();
        if let RatatuiColor::Rgb(r, g, b) = color {
            let width = (val * max_width as f32) as usize;
            let bar = format!("\x1b[48;2;{r};{g};{b}m{}\x1b[0m", " ".repeat(width));
            println!("  [{i}] {bar} {val:.2}");
        }
    }
}
