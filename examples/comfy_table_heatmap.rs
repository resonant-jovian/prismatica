//! Render a colored table using comfy-table with prismatica colormaps.
//!
//! Creates a heatmap-style table where cell background colors are derived
//! from the VIRIDIS colormap, demonstrating comfy-table integration for
//! data visualization in the terminal.
//!
//! Run with: `cargo run --example comfy_table_heatmap --features "matplotlib,comfy-table-integration"`

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, Color as TableColor, Table};
use prismatica::matplotlib::VIRIDIS;

fn main() {
    println!("Colormap: {} ({})\n", VIRIDIS.meta.name, VIRIDIS.meta.collection);

    // Build a heatmap table with sample data
    let data: Vec<Vec<f32>> = vec![
        vec![0.05, 0.20, 0.35, 0.50, 0.65],
        vec![0.15, 0.30, 0.55, 0.70, 0.80],
        vec![0.25, 0.45, 0.60, 0.85, 0.95],
        vec![0.10, 0.40, 0.50, 0.75, 0.90],
    ];

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new(""),
            Cell::new("Col A").add_attribute(Attribute::Bold),
            Cell::new("Col B").add_attribute(Attribute::Bold),
            Cell::new("Col C").add_attribute(Attribute::Bold),
            Cell::new("Col D").add_attribute(Attribute::Bold),
            Cell::new("Col E").add_attribute(Attribute::Bold),
        ]);

    for (row_idx, row) in data.iter().enumerate() {
        let label = Cell::new(format!("Row {}", row_idx + 1)).add_attribute(Attribute::Bold);
        let mut cells = vec![label];

        for &val in row {
            let color: TableColor = VIRIDIS.eval(val).into();
            cells.push(
                Cell::new(format!("{val:.2}"))
                    .bg(color)
                    .set_alignment(CellAlignment::Center),
            );
        }

        table.add_row(cells);
    }

    println!("{table}");

    // Also show a simple gradient row
    println!("\nGradient strip:");
    let mut gradient_table = Table::new();
    gradient_table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);

    let header: Vec<Cell> = (0..10)
        .map(|i| {
            let t = i as f32 / 9.0;
            Cell::new(format!("{t:.1}")).add_attribute(Attribute::Bold)
        })
        .collect();
    gradient_table.set_header(header);

    let row: Vec<Cell> = (0..10)
        .map(|i| {
            let t = i as f32 / 9.0;
            let color: TableColor = VIRIDIS.eval(t).into();
            Cell::new("    ")
                .bg(color)
                .set_alignment(CellAlignment::Center)
        })
        .collect();
    gradient_table.add_row(row);

    println!("{gradient_table}");
}
