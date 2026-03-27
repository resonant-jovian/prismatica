//! Render a 2D heatmap of `sin(x) * cos(y)` using plotters with the VIRIDIS colormap.
//!
//! Run with: `cargo run --example plotters_heatmap --features "matplotlib,plotters-integration"`
//!
//! Produces `heatmap.svg` in the current directory.

use std::f64::consts::PI;

use plotters::prelude::*;
use prismatica::matplotlib::VIRIDIS;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("heatmap.svg", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("sin(x) * cos(y)", ("sans-serif", 28))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(-PI..PI, -PI..PI)?;

    chart.configure_mesh().x_desc("x").y_desc("y").draw()?;

    let resolution = 100;
    let step = 2.0 * PI / resolution as f64;

    for xi in 0..resolution {
        for yi in 0..resolution {
            let x = -PI + (xi as f64 + 0.5) * step;
            let y = -PI + (yi as f64 + 0.5) * step;
            let value = x.sin() * y.cos();

            // Normalize from [-1, 1] to [0, 1]
            let t = ((value + 1.0) / 2.0) as f32;
            let color = VIRIDIS.eval(t);
            let rgb: RGBColor = color.into();

            let x0 = -PI + xi as f64 * step;
            let y0 = -PI + yi as f64 * step;

            chart.draw_series(std::iter::once(Rectangle::new(
                [(x0, y0), (x0 + step, y0 + step)],
                rgb.filled(),
            )))?;
        }
    }

    root.present()?;

    println!("Heatmap written to heatmap.svg");

    Ok(())
}
