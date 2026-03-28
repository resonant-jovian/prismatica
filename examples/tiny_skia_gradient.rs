//! Render a gradient strip using tiny-skia and save as PNG.
//!
//! Creates a pixmap, fills each column with the corresponding VIRIDIS colormap
//! color, then saves the result as a PNG image via the image crate.
//!
//! Run with: `cargo run --example tiny_skia_gradient --features "matplotlib,tiny-skia-integration,image-integration"`

use image::RgbImage;
use prismatica::Color;
use prismatica::matplotlib::VIRIDIS;
use tiny_skia::Color as SkiaColor;

fn main() {
    let cm = &VIRIDIS;
    let width = 512u32;
    let height = 64u32;

    println!("Colormap: {} ({})", cm.meta.name, cm.meta.collection);
    println!("Rendering {}x{} gradient...\n", width, height);

    // Build a tiny-skia Pixmap and fill it with the colormap gradient
    let mut pixmap = tiny_skia::Pixmap::new(width, height).expect("failed to create pixmap");

    for x in 0..width {
        let t = x as f32 / (width - 1) as f32;
        let color = cm.eval(t);

        // Forward: prismatica::Color -> tiny_skia::Color
        let skia: SkiaColor = color.into();

        // Reverse: tiny_skia::Color -> prismatica::Color (alpha discarded)
        let _back: Color = skia.into();

        // Fill this column top to bottom
        let premul = skia.premultiply().to_color_u8();
        for y in 0..height {
            let idx = (y * width + x) as usize;
            pixmap.pixels_mut()[idx] = premul;
        }
    }

    // Extract pixel data and save as PNG using the image crate
    let mut img = RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let t = x as f32 / (width - 1) as f32;
            let color = cm.eval(t);
            let pixel: image::Rgb<u8> = color.into();
            img.put_pixel(x, y, pixel);
        }
    }

    let filename = "viridis_tiny_skia_gradient.png";
    img.save(filename).expect("failed to save PNG");
    println!("Saved {}", filename);

    // Print a small ANSI preview
    print!("  ");
    for i in 0..64 {
        let t = i as f32 / 63.0;
        let c = cm.eval(t);
        print!("\x1b[48;2;{};{};{}m \x1b[0m", c.r, c.g, c.b);
    }
    println!();
}
