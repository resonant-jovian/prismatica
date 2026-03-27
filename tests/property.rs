//! Property-based tests for core types and colormaps.
//!
//! Run with: `cargo test --test property --features all`

use proptest::prelude::*;

#[cfg(feature = "matplotlib")]
use prismatica::matplotlib::VIRIDIS;
use prismatica::Color;

proptest! {
    #[test]
    fn color_hex_roundtrip(hex in 0u32..=0xFFFFFFu32) {
        let c = Color::from_hex(hex);
        prop_assert_eq!(c.to_hex(), hex);
    }

    #[test]
    fn color_luminance_in_bounds(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
        let lum = Color::new(r, g, b).luminance();
        prop_assert!(lum >= 0.0, "luminance {} < 0", lum);
        prop_assert!(lum <= 1.0, "luminance {} > 1", lum);
    }

    #[test]
    fn contrast_ratio_symmetric(
        r1 in 0u8..=255, g1 in 0u8..=255, b1 in 0u8..=255,
        r2 in 0u8..=255, g2 in 0u8..=255, b2 in 0u8..=255,
    ) {
        let a = Color::new(r1, g1, b1);
        let b = Color::new(r2, g2, b2);
        let ab = a.contrast_ratio(b);
        let ba = b.contrast_ratio(a);
        prop_assert!(
            (ab - ba).abs() < 1e-10,
            "contrast_ratio not symmetric: {} vs {}",
            ab,
            ba
        );
    }

    #[test]
    fn contrast_ratio_in_bounds(
        r1 in 0u8..=255, g1 in 0u8..=255, b1 in 0u8..=255,
        r2 in 0u8..=255, g2 in 0u8..=255, b2 in 0u8..=255,
    ) {
        let ratio = Color::new(r1, g1, b1).contrast_ratio(Color::new(r2, g2, b2));
        prop_assert!(ratio >= 1.0, "contrast ratio {} < 1", ratio);
        prop_assert!(ratio <= 21.1, "contrast ratio {} > 21.1", ratio);
    }

    #[test]
    fn color_display_format(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
        let c = Color::new(r, g, b);
        let s = format!("{c}");
        prop_assert!(s.starts_with('#'));
        prop_assert_eq!(s.len(), 7);
        // Verify roundtrip through hex parsing
        let hex = u32::from_str_radix(&s[1..], 16).unwrap();
        prop_assert_eq!(Color::from_hex(hex), c);
    }
}

#[cfg(feature = "matplotlib")]
proptest! {
    #[test]
    fn eval_returns_valid_color(t in 0.0f32..=1.0) {
        let c = VIRIDIS.eval(t);
        // If we get here without panic, the color is valid (u8 fields are always in range)
        prop_assert!(c.r <= 255);
        prop_assert!(c.g <= 255);
        prop_assert!(c.b <= 255);
    }

    #[test]
    fn eval_clamps_out_of_range(t in -1000.0f32..=-0.001) {
        let c = VIRIDIS.eval(t);
        let at_zero = VIRIDIS.eval(0.0);
        prop_assert_eq!(c, at_zero, "eval({}) should clamp to eval(0.0)", t);
    }

    #[test]
    fn eval_clamps_above_range(t in 1.001f32..=1000.0) {
        let c = VIRIDIS.eval(t);
        let at_one = VIRIDIS.eval(1.0);
        prop_assert_eq!(c, at_one, "eval({}) should clamp to eval(1.0)", t);
    }
}
