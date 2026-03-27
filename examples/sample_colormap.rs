//! Sample a colormap at evenly-spaced points and display as ANSI-colored blocks.
//!
//! Demonstrates `eval()` (continuous) and `eval_rational()` (integer index) APIs.
//!
//! Run with: `cargo run --example sample_colormap`

fn main() {
    let cm = &prismatica::matplotlib::VIRIDIS;
    let n = 16;

    println!("Colormap: {} ({})\n", cm.meta.name, cm.meta.collection);

    // Continuous eval: sample at 16 evenly-spaced t values
    println!("eval(t) -- continuous sampling:");
    print!("  ");
    for i in 0..n {
        let t = i as f32 / (n - 1) as f32;
        let c = cm.eval(t);
        print!("\x1b[48;2;{};{};{}m  \x1b[0m", c.r, c.g, c.b);
    }
    println!();

    // Rational eval: equivalent, but takes integer index + count
    println!("\neval_rational(i, n) -- discrete sampling:");
    print!("  ");
    for i in 0..n {
        let c = cm.eval_rational(i, n);
        print!("\x1b[48;2;{};{};{}m  \x1b[0m", c.r, c.g, c.b);
    }
    println!();

    // Print individual color values
    println!("\n  {:<4} {:<8} {:<18} HEX", "i", "t", "RGB");
    println!("  {}", "-".repeat(40));
    for i in 0..n {
        let t = i as f32 / (n - 1) as f32;
        let c = cm.eval(t);
        let swatch = format!("\x1b[48;2;{};{};{}m  \x1b[0m", c.r, c.g, c.b);
        println!(
            "  {i:<4} {t:<8.4} ({r:>3},{g:>3},{b:>3})      {swatch} {hex}",
            i = i,
            t = t,
            r = c.r,
            g = c.g,
            b = c.b,
            swatch = swatch,
            hex = c.to_css_hex(),
        );
    }
}
