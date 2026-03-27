//! Snapshot tests to detect codegen changes.
//!
//! Run with: `cargo test --test snapshots --features all`
//! Update snapshots: `cargo insta review`

#[cfg(feature = "all")]
mod snapshot_tests {
    use prismatica::prelude::*;

    #[test]
    fn registry_collection_counts() {
        let all = all_colormaps();
        let palettes = all_discrete_palettes();

        let mut summary = format!(
            "total_colormaps: {}\ntotal_palettes: {}\n\ncollections:\n",
            all.len(),
            palettes.len()
        );

        // Count per collection
        let collections = [
            "matplotlib",
            "crameri",
            "cet",
            "cmocean",
            "colorbrewer",
            "cmasher",
            "ncar",
            "cartocolors",
            "moreland",
            "d3",
        ];

        for name in collections {
            let count = filter_by_collection(name).len();
            summary.push_str(&format!("  {name}: {count}\n"));
        }

        insta::assert_snapshot!("registry_counts", summary);
    }

    #[test]
    fn crameri_mod_header() {
        let content = include_str!("../src/crameri/mod.rs");
        let header: String = content.lines().take(15).collect::<Vec<_>>().join("\n");
        insta::assert_snapshot!("crameri_mod_header", header);
    }

    #[test]
    fn batlow_colormap_definition() {
        let content = include_str!("../src/crameri/batlow.rs");
        let header: String = content.lines().take(25).collect::<Vec<_>>().join("\n");
        insta::assert_snapshot!("batlow_definition", header);
    }

    #[test]
    fn colorbrewer_mod_header() {
        let content = include_str!("../src/colorbrewer/mod.rs");
        let header: String = content.lines().take(15).collect::<Vec<_>>().join("\n");
        insta::assert_snapshot!("colorbrewer_mod_header", header);
    }
}
