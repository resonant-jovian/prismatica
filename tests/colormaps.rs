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

#[cfg(feature = "cet")]
mod cet_tests {
    use super::*;

    #[test]
    fn all_cet_luts_are_256() {
        for cm in prismatica::cet::ALL {
            assert_eq!(
                cm.lut.len(),
                256,
                "CET map '{}' has {} entries",
                cm.meta.name,
                cm.lut.len()
            );
        }
    }

    #[test]
    fn cet_collection_count() {
        assert!(
            prismatica::cet::ALL.len() >= 56,
            "Expected >= 56 CET maps, got {}",
            prismatica::cet::ALL.len()
        );
    }

    #[test]
    fn cet_cyclic_maps_wrap() {
        for cm in prismatica::cet::ALL {
            if cm.meta.kind == ColormapKind::Cyclic {
                let first = cm.lut[0];
                let last = cm.lut[255];
                let dr = (first[0] as i16 - last[0] as i16).unsigned_abs();
                let dg = (first[1] as i16 - last[1] as i16).unsigned_abs();
                let db = (first[2] as i16 - last[2] as i16).unsigned_abs();
                assert!(
                    dr <= 10 && dg <= 10 && db <= 10,
                    "CET cyclic map '{}' doesn't wrap: first={:?}, last={:?}",
                    cm.meta.name,
                    first,
                    last
                );
            }
        }
    }
}

#[cfg(feature = "cmocean")]
mod cmocean_tests {
    use super::*;

    #[test]
    fn all_cmocean_luts_are_256() {
        for cm in prismatica::cmocean::ALL {
            assert_eq!(
                cm.lut.len(),
                256,
                "CMOcean map '{}' has {} entries",
                cm.meta.name,
                cm.lut.len()
            );
        }
    }

    #[test]
    fn cmocean_collection_count() {
        assert_eq!(prismatica::cmocean::ALL.len(), 22);
    }

    #[test]
    fn cmocean_phase_is_cyclic() {
        assert_eq!(prismatica::cmocean::PHASE.meta.kind, ColormapKind::Cyclic);
    }

    #[test]
    fn cmocean_balance_is_diverging() {
        assert_eq!(
            prismatica::cmocean::BALANCE.meta.kind,
            ColormapKind::Diverging
        );
    }
}

#[cfg(feature = "moreland")]
mod moreland_tests {
    use super::*;

    #[test]
    fn all_moreland_luts_are_256() {
        for cm in prismatica::moreland::ALL {
            assert_eq!(
                cm.lut.len(),
                256,
                "Moreland map '{}' has {} entries",
                cm.meta.name,
                cm.lut.len()
            );
        }
    }

    #[test]
    fn moreland_collection_count() {
        assert_eq!(prismatica::moreland::ALL.len(), 6);
    }

    #[test]
    fn smooth_cool_warm_is_diverging() {
        assert_eq!(
            prismatica::moreland::SMOOTH_COOL_WARM.meta.kind,
            ColormapKind::Diverging
        );
    }
}

#[cfg(feature = "cmasher")]
mod cmasher_tests {
    use super::*;

    #[test]
    fn all_cmasher_luts_are_256() {
        for cm in prismatica::cmasher::ALL {
            assert_eq!(
                cm.lut.len(),
                256,
                "CMasher map '{}' has {} entries",
                cm.meta.name,
                cm.lut.len()
            );
        }
    }

    #[test]
    fn cmasher_collection_count() {
        assert!(
            prismatica::cmasher::ALL.len() >= 30,
            "Expected >= 30 CMasher maps, got {}",
            prismatica::cmasher::ALL.len()
        );
    }

    #[test]
    fn cmasher_fusion_is_diverging() {
        assert_eq!(
            prismatica::cmasher::FUSION.meta.kind,
            ColormapKind::Diverging
        );
    }
}

#[cfg(feature = "colorbrewer")]
mod colorbrewer_tests {
    use super::*;

    #[test]
    fn all_colorbrewer_luts_are_256() {
        for cm in prismatica::colorbrewer::ALL {
            assert_eq!(
                cm.lut.len(),
                256,
                "ColorBrewer map '{}' has {} entries",
                cm.meta.name,
                cm.lut.len()
            );
        }
    }

    #[test]
    fn colorbrewer_collection_count() {
        assert_eq!(prismatica::colorbrewer::ALL.len(), 35);
    }

    #[test]
    fn colorbrewer_has_discrete_palettes() {
        assert!(!prismatica::colorbrewer::ALL_DISCRETE.is_empty());
        for p in prismatica::colorbrewer::ALL_DISCRETE {
            assert!(
                p.len() >= 3 && p.len() <= 12,
                "palette '{}' has {} colors, expected 3-12",
                p.meta.name,
                p.len()
            );
        }
    }

    #[test]
    fn colorbrewer_qualitative_exists() {
        let has_qual = prismatica::colorbrewer::ALL
            .iter()
            .any(|cm| cm.meta.kind == ColormapKind::Qualitative);
        assert!(has_qual);
    }
}

#[cfg(feature = "cartocolors")]
mod cartocolors_tests {
    #[test]
    fn all_cartocolors_luts_are_256() {
        for cm in prismatica::cartocolors::ALL {
            assert_eq!(
                cm.lut.len(),
                256,
                "CartoColors map '{}' has {} entries",
                cm.meta.name,
                cm.lut.len()
            );
        }
    }

    #[test]
    fn cartocolors_collection_count() {
        assert!(prismatica::cartocolors::ALL.len() >= 30);
    }

    #[test]
    fn cartocolors_has_discrete_palettes() {
        assert!(!prismatica::cartocolors::ALL_DISCRETE.is_empty());
    }
}

#[cfg(feature = "ncar")]
mod ncar_tests {
    #[test]
    fn all_ncar_luts_are_256() {
        for cm in prismatica::ncar::ALL {
            assert_eq!(
                cm.lut.len(),
                256,
                "NCAR map '{}' has {} entries",
                cm.meta.name,
                cm.lut.len()
            );
        }
    }

    #[test]
    fn ncar_collection_count() {
        assert!(prismatica::ncar::ALL.len() >= 20);
    }
}

#[cfg(feature = "d3")]
mod d3_tests {
    use super::*;

    #[test]
    fn all_d3_luts_are_256() {
        for cm in prismatica::d3::ALL {
            assert_eq!(
                cm.lut.len(),
                256,
                "d3 map '{}' has {} entries",
                cm.meta.name,
                cm.lut.len()
            );
        }
    }

    #[test]
    fn d3_collection_count() {
        assert_eq!(prismatica::d3::ALL.len(), 7);
    }

    #[test]
    fn d3_sinebow_is_cyclic() {
        assert_eq!(prismatica::d3::SINEBOW.meta.kind, ColormapKind::Cyclic);
    }

    #[test]
    fn d3_rainbow_is_cyclic() {
        assert_eq!(prismatica::d3::RAINBOW.meta.kind, ColormapKind::Cyclic);
    }

    #[test]
    fn d3_tableau10_palette_has_10_colors() {
        assert_eq!(prismatica::d3::TABLEAU10_PALETTE.len(), 10);
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

#[cfg(all(feature = "all", feature = "std"))]
mod full_registry_tests {
    use prismatica::*;

    #[test]
    fn total_colormap_count() {
        let all = all_colormaps();
        assert!(
            all.len() >= 200,
            "Expected 200+ colormaps, got {}",
            all.len()
        );
    }

    #[test]
    fn no_duplicate_names_across_collections() {
        let all = all_colormaps();
        let mut names: Vec<&str> = all.iter().map(|cm| cm.meta.name).collect();
        names.sort();
        for pair in names.windows(2) {
            assert_ne!(pair[0], pair[1], "Duplicate: '{}'", pair[0]);
        }
    }

    #[test]
    fn all_collections_represented() {
        let all = all_colormaps();
        for expected in &[
            "matplotlib",
            "crameri",
            "cet",
            "cmocean",
            "moreland",
            "cmasher",
            "colorbrewer",
            "cartocolors",
            "ncar",
            "d3",
        ] {
            assert!(
                all.iter().any(|cm| cm.meta.collection == *expected),
                "Missing collection: {}",
                expected
            );
        }
    }

    #[test]
    fn discrete_palettes_exist() {
        let palettes = all_discrete_palettes();
        assert!(!palettes.is_empty(), "Expected discrete palettes");
        // Should have at least ColorBrewer (35) + CartoColors (34) + d3 (1)
        assert!(
            palettes.len() >= 60,
            "Expected >= 60 palettes, got {}",
            palettes.len()
        );
    }

    #[test]
    fn find_palette_by_name_works() {
        // ColorBrewer
        assert!(find_palette_by_name("Blues").is_some());
        // d3
        assert!(find_palette_by_name("Tableau10").is_some());
        assert!(find_palette_by_name("nonexistent").is_none());
    }

    #[test]
    fn no_cross_collection_colormap_name_duplicates() {
        let all = all_colormaps();
        let mut seen = std::collections::HashMap::new();
        for cm in &all {
            if let Some(prev) = seen.insert(cm.meta.name, cm.meta.collection) {
                panic!(
                    "Duplicate colormap '{}' in both '{}' and '{}'",
                    cm.meta.name, prev, cm.meta.collection
                );
            }
        }
    }

    #[test]
    fn no_cross_collection_palette_name_duplicates() {
        let all = all_discrete_palettes();
        let mut seen = std::collections::HashMap::new();
        for p in &all {
            if let Some(prev) = seen.insert(p.meta.name, p.meta.collection) {
                panic!(
                    "Duplicate palette '{}' in both '{}' and '{}'",
                    p.meta.name, prev, p.meta.collection
                );
            }
        }
    }
}
