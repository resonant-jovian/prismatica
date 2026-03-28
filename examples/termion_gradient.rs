//! Print a colormap gradient using termion colors.
//!
//! Uses `termion::color::Fg` and `termion::color::Bg` to render prismatica
//! colormap samples as colored terminal output. Note: termion only supports
//! Unix platforms.
//!
//! Run with: `cargo run --example termion_gradient --features "matplotlib,termion-integration"`

#[cfg(not(unix))]
fn main() {
    eprintln!("termion is only available on Unix platforms.");
}

#[cfg(unix)]
fn main() {
    use prismatica::matplotlib::VIRIDIS;
    use termion::color;

    let n = 48;

    println!(
        "Colormap: {} ({})\n",
        VIRIDIS.meta.name, VIRIDIS.meta.collection
    );

    // Background-colored gradient bar
    println!("Background gradient ({n} steps):");
    print!("  ");
    for i in 0..n {
        let t = i as f32 / (n - 1) as f32;
        let rgb: color::Rgb = VIRIDIS.eval(t).into();
        print!("{} {}", color::Bg(rgb), color::Bg(color::Reset));
    }
    println!();

    // Foreground-colored text
    println!("\nForeground-colored characters:");
    let text = "Scientific colormaps rendered with termion";
    print!("  ");
    for (i, ch) in text.chars().enumerate() {
        let t = i as f32 / (text.len() - 1) as f32;
        let rgb: color::Rgb = VIRIDIS.eval(t).into();
        print!("{}{ch}{}", color::Fg(rgb), color::Fg(color::Reset));
    }
    println!();

    // Dual-line thick gradient (Bg on two rows)
    println!("\nThick gradient:");
    for _ in 0..2 {
        print!("  ");
        for i in 0..n {
            let t = i as f32 / (n - 1) as f32;
            let rgb: color::Rgb = VIRIDIS.eval(t).into();
            print!("{} {}", color::Bg(rgb), color::Bg(color::Reset));
        }
        println!();
    }

    // Labeled discrete samples with Fg and Bg
    println!("\nDiscrete samples:");
    println!("  {:<4} {:<8} {:<14} swatch", "i", "t", "RGB");
    println!("  {}", "-".repeat(40));
    for i in 0..8 {
        let t = i as f32 / 7.0;
        let pc = VIRIDIS.eval(t);
        let rgb: color::Rgb = pc.into();
        println!(
            "  {i:<4} {t:<8.4} ({r:>3},{g:>3},{b:>3})  {bg}      {reset}",
            i = i,
            t = t,
            r = pc.r,
            g = pc.g,
            b = pc.b,
            bg = color::Bg(rgb),
            reset = color::Bg(color::Reset),
        );
    }
}
