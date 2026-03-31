use criterion::{Criterion, black_box, criterion_group, criterion_main};
use prismatica::colorbrewer::SET2_PALETTE;
use prismatica::crameri::BATLOW;
use prismatica::{Color, all_colormaps, find_by_name};

fn bench_eval(c: &mut Criterion) {
    c.bench_function("Colormap::eval", |b| {
        b.iter(|| BATLOW.eval(black_box(0.5)));
    });
}

fn bench_eval_rational(c: &mut Criterion) {
    c.bench_function("Colormap::eval_rational", |b| {
        b.iter(|| BATLOW.eval_rational(black_box(128), black_box(256)));
    });
}

fn bench_colors(c: &mut Criterion) {
    c.bench_function("Colormap::colors(256)", |b| {
        b.iter(|| BATLOW.colors(black_box(256)));
    });
}

fn bench_find_by_name(c: &mut Criterion) {
    c.bench_function("find_by_name", |b| {
        b.iter(|| find_by_name(black_box("batlow")));
    });
}

fn bench_all_colormaps(c: &mut Criterion) {
    c.bench_function("all_colormaps", |b| {
        b.iter(all_colormaps);
    });
}

fn bench_color_lerp(c: &mut Criterion) {
    let black = Color::new(0, 0, 0);
    let white = Color::new(255, 255, 255);
    c.bench_function("Color::lerp", |b| {
        b.iter(|| black_box(black).lerp(black_box(white), black_box(0.5)));
    });
}

fn bench_color_luminance(c: &mut Criterion) {
    let color = Color::new(128, 64, 192);
    c.bench_function("Color::luminance", |b| {
        b.iter(|| black_box(color).luminance());
    });
}

fn bench_color_from_css_hex(c: &mut Criterion) {
    c.bench_function("Color::from_css_hex", |b| {
        b.iter(|| Color::from_css_hex(black_box("#ff8800")));
    });
}

fn bench_discrete_palette_iter(c: &mut Criterion) {
    c.bench_function("DiscretePalette::iter().count()", |b| {
        b.iter(|| black_box(&SET2_PALETTE).iter().count());
    });
}

criterion_group!(
    benches,
    bench_eval,
    bench_eval_rational,
    bench_colors,
    bench_find_by_name,
    bench_all_colormaps,
    bench_color_lerp,
    bench_color_luminance,
    bench_color_from_css_hex,
    bench_discrete_palette_iter,
);
criterion_main!(benches);
