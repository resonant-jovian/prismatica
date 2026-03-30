//! Property-based tests for core types and colormaps.
//!
//! Run with: `cargo test --test property --features all`

use proptest::prelude::*;

use prismatica::Color;
#[cfg(feature = "matplotlib")]
use prismatica::matplotlib::VIRIDIS;

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
        let hex = u32::from_str_radix(&s[1..], 16).expect("valid hex from Display");
        prop_assert_eq!(Color::from_hex(hex), c);
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
proptest! {
    #[test]
    fn from_css_hex_roundtrip(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
        let c = Color::new(r, g, b);
        let hex = c.to_css_hex();
        let parsed = Color::from_css_hex(&hex).expect("roundtrip should succeed");
        assert_eq!(parsed, c);
    }

    #[test]
    fn from_f32_near_roundtrip(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
        let c = Color::new(r, g, b);
        let (fr, fg, fb) = c.to_f32();
        let back = Color::from_f32(fr, fg, fb);
        // Allow +/-1 tolerance for floating-point rounding
        assert!((back.r as i16 - c.r as i16).abs() <= 1, "r: {} vs {}", back.r, c.r);
        assert!((back.g as i16 - c.g as i16).abs() <= 1, "g: {} vs {}", back.g, c.g);
        assert!((back.b as i16 - c.b as i16).abs() <= 1, "b: {} vs {}", back.b, c.b);
    }
}

#[cfg(feature = "matplotlib")]
proptest! {
    #[test]
    fn eval_returns_valid_color(t in 0.0f32..=1.0) {
        let c = VIRIDIS.eval(t);
        // Verify eval doesn't panic and returns a valid Color
        let _ = c.r;
        let _ = c.g;
        let _ = c.b;
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
