//! Render a colormap as a horizontal gradient PNG image.
//!
//! Run with: `cargo run --example image_gradient --features "all,image-integration"`
//!
//! Accepts an optional colormap name as the first CLI argument (defaults to "batlow").

use image::RgbImage;
use prismatica::find_by_name;

fn main() {
    let name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "batlow".to_string());

    let colormap = match find_by_name(&name) {
        Some(cm) => cm,
        None => {
            eprintln!("Error: colormap \"{}\" not found.", name);
            eprintln!("Usage: image_gradient [COLORMAP_NAME]");
            eprintln!("Example: image_gradient viridis");
            std::process::exit(1);
        }
    };

    let width = 512u32;
    let height = 64u32;

    let mut img = RgbImage::new(width, height);

    for x in 0..width {
        let t = x as f32 / (width - 1) as f32;
        let color = colormap.eval(t);
        let pixel: image::Rgb<u8> = color.into();
        for y in 0..height {
            img.put_pixel(x, y, pixel);
        }
    }

    let filename = format!("{}_gradient.png", name);
    img.save(&filename).expect("failed to save image");
    println!("Saved {}", filename);
}
