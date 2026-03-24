use prismatica::*;

#[cfg(feature = "matplotlib")]
mod matplotlib_tests {

    #[test]
    fn viridis_reference_values() {
        // Verify viridis endpoints are close to canonical values.
        // Slight differences from (68,1,84) and (253,231,37) are expected
        // due to resampling from upstream 255-entry source to 256 entries.
        let c = prismatica::matplotlib::VIRIDIS.eval(0.0);
        assert!(
            (c.r as i16 - 68).unsigned_abs() <= 2,
            "viridis(0.0) r={}",
            c.r
        );
        assert!(
            (c.g as i16 - 1).unsigned_abs() <= 2,
            "viridis(0.0) g={}",
            c.g
        );
        assert!(
            (c.b as i16 - 84).unsigned_abs() <= 3,
            "viridis(0.0) b={}",
            c.b
        );

        let c = prismatica::matplotlib::VIRIDIS.eval(1.0);
        assert!(
            (c.r as i16 - 253).unsigned_abs() <= 2,
            "viridis(1.0) r={}",
            c.r
        );
        assert!(
            (c.g as i16 - 231).unsigned_abs() <= 2,
            "viridis(1.0) g={}",
            c.g
        );
        assert!(
            (c.b as i16 - 37).unsigned_abs() <= 2,
            "viridis(1.0) b={}",
            c.b
        );
    }

    #[test]
    fn eval_clamps_properly() {
        let cm = &prismatica::matplotlib::VIRIDIS;
        assert_eq!(cm.eval(-1.0), cm.eval(0.0));
        assert_eq!(cm.eval(2.0), cm.eval(1.0));
    }

    #[test]
    fn all_matplotlib_luts_are_256() {
        for cm in prismatica::matplotlib::ALL {
            assert_eq!(
                cm.lut.len(),
                256,
                "Map '{}' has {} LUT entries, expected 256",
                cm.meta.name,
                cm.lut.len()
            );
        }
    }

    #[test]
    fn eval_rational_consistency() {
        let cm = &prismatica::matplotlib::VIRIDIS;
        assert_eq!(cm.eval_rational(0, 256), cm.eval(0.0));
        assert_eq!(cm.eval_rational(255, 256), cm.eval(1.0));
    }

    #[test]
    fn reversed_is_inverse() {
        let cm = &prismatica::matplotlib::VIRIDIS;
        let rev = cm.reversed();
        assert_eq!(rev.eval(0.0), cm.eval(1.0));
        assert_eq!(rev.eval(1.0), cm.eval(0.0));
    }

    #[test]
    fn matplotlib_count() {
        assert_eq!(prismatica::matplotlib::ALL.len(), 8);
    }
}

#[cfg(feature = "crameri")]
mod crameri_tests {
    use super::*;

    #[test]
    fn all_crameri_luts_are_256() {
        for cm in prismatica::crameri::ALL {
            assert_eq!(
                cm.lut.len(),
                256,
                "Map '{}' has {} LUT entries, expected 256",
                cm.meta.name,
                cm.lut.len()
            );
        }
    }

    #[test]
    fn crameri_collection_count() {
        assert_eq!(prismatica::crameri::ALL.len(), 40);
    }

    #[test]
    fn cyclic_maps_wrap() {
        for cm in prismatica::crameri::ALL {
            if cm.meta.kind == ColormapKind::Cyclic {
                let first = cm.lut[0];
                let last = cm.lut[255];
                let dr = (first[0] as i16 - last[0] as i16).unsigned_abs();
                let dg = (first[1] as i16 - last[1] as i16).unsigned_abs();
                let db = (first[2] as i16 - last[2] as i16).unsigned_abs();
                assert!(
                    dr <= 2 && dg <= 2 && db <= 2,
                    "Cyclic map '{}' doesn't wrap: first={:?}, last={:?}",
                    cm.meta.name,
                    first,
                    last
                );
            }
        }
    }

    #[test]
    fn batlow_eval_works() {
        let c = prismatica::crameri::BATLOW.eval(0.0);
        let c2 = prismatica::crameri::BATLOW.eval(0.5);
        assert_ne!(c, c2, "batlow(0.0) should differ from batlow(0.5)");
    }
}

#[cfg(all(feature = "matplotlib", feature = "crameri", feature = "std"))]
mod registry_tests {
    use super::*;

    #[test]
    fn all_colormaps_returns_all() {
        let all = all_colormaps();
        assert!(
            all.len() >= 48,
            "Expected at least 48 colormaps, got {}",
            all.len()
        );
    }

    #[test]
    fn find_by_name_works() {
        assert!(find_by_name("viridis").is_some());
        assert!(find_by_name("batlow").is_some());
        assert!(find_by_name("nonexistent").is_none());
    }

    #[test]
    fn no_duplicate_names() {
        let all = all_colormaps();
        let mut names: Vec<&str> = all.iter().map(|cm| cm.meta.name).collect();
        names.sort();
        for pair in names.windows(2) {
            assert_ne!(pair[0], pair[1], "Duplicate colormap name: '{}'", pair[0]);
        }
    }

    #[test]
    fn filter_by_kind_diverging() {
        let divs = filter_by_kind(ColormapKind::Diverging);
        assert!(
            divs.len() >= 10,
            "Expected at least 10 diverging maps, got {}",
            divs.len()
        );
        for cm in &divs {
            assert_eq!(cm.meta.kind, ColormapKind::Diverging);
        }
    }

    #[test]
    fn filter_by_collection_crameri() {
        let crameri = filter_by_collection("crameri");
        assert_eq!(crameri.len(), 40);
        for cm in &crameri {
            assert_eq!(cm.meta.collection, "crameri");
        }
    }
}
