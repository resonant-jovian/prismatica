//! Colormap discovery and filtering.
//!
//! Functions for querying the full catalog of available colormaps at
//! runtime. Which colormaps are present depends on the enabled feature
//! flags.

use crate::{Colormap, DiscretePalette};

#[cfg(any(feature = "alloc", feature = "std"))]
use {crate::ColormapKind, alloc::vec::Vec};

/// Iterate over each enabled collection's ALL slice.
///
/// Each collection module exposes `pub static ALL: &[&Colormap]`.
/// This function iterates all enabled collections without allocation.
fn for_each_colormap(mut f: impl FnMut(&'static Colormap)) {
    #[cfg(feature = "matplotlib")]
    for cm in crate::matplotlib::ALL {
        f(cm);
    }
    #[cfg(feature = "crameri")]
    for cm in crate::crameri::ALL {
        f(cm);
    }
    #[cfg(feature = "cet")]
    for cm in crate::cet::ALL {
        f(cm);
    }
    #[cfg(feature = "cmocean")]
    for cm in crate::cmocean::ALL {
        f(cm);
    }
    #[cfg(feature = "colorbrewer")]
    for cm in crate::colorbrewer::ALL {
        f(cm);
    }
    #[cfg(feature = "cmasher")]
    for cm in crate::cmasher::ALL {
        f(cm);
    }
    #[cfg(feature = "ncar")]
    for cm in crate::ncar::ALL {
        f(cm);
    }
    #[cfg(feature = "cartocolors")]
    for cm in crate::cartocolors::ALL {
        f(cm);
    }
    #[cfg(feature = "moreland")]
    for cm in crate::moreland::ALL {
        f(cm);
    }
    #[cfg(feature = "d3")]
    for cm in crate::d3::ALL {
        f(cm);
    }
    // Suppress unused variable warning when no collections are enabled
    let _ = &mut f;
}

/// Iterate over each enabled collection's ALL_DISCRETE slice.
fn for_each_discrete_palette(mut f: impl FnMut(&'static DiscretePalette)) {
    #[cfg(feature = "colorbrewer")]
    for p in crate::colorbrewer::ALL_DISCRETE {
        f(p);
    }
    #[cfg(feature = "cartocolors")]
    for p in crate::cartocolors::ALL_DISCRETE {
        f(p);
    }
    #[cfg(feature = "d3")]
    for p in crate::d3::ALL_DISCRETE {
        f(p);
    }
    let _ = &mut f;
}

/// Returns all colormaps enabled by the current feature flags.
///
/// # Examples
///
/// ```
/// use prismatica::all_colormaps;
/// let maps = all_colormaps();
/// assert!(!maps.is_empty());
/// ```
#[must_use]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
pub fn all_colormaps() -> Vec<&'static Colormap> {
    let mut result = Vec::new();
    for_each_colormap(|cm| result.push(cm));
    result
}

/// Look up a colormap by its canonical name (case-sensitive).
///
/// Names use their original casing from upstream sources (e.g.,
/// `"batlowK"`, `"romaO"`, `"viridis"`). Most names are lowercase,
/// but Crameri maps with suffixes preserve mixed case.
///
/// # Examples
///
/// ```
/// use prismatica::find_by_name;
/// let batlow = find_by_name("batlow").expect("batlow should exist");
/// assert_eq!(batlow.meta.name, "batlow");
/// assert!(find_by_name("nonexistent").is_none());
/// ```
#[must_use]
pub fn find_by_name(name: &str) -> Option<&'static Colormap> {
    let mut found = None;
    for_each_colormap(|cm| {
        if cm.meta.name == name {
            found = Some(cm);
        }
    });
    found
}

/// Return all colormaps of a given kind.
///
/// # Examples
///
/// ```
/// use prismatica::{filter_by_kind, ColormapKind};
/// let sequential = filter_by_kind(ColormapKind::Sequential);
/// assert!(sequential.iter().all(|cm| cm.meta.kind == ColormapKind::Sequential));
/// ```
#[must_use]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
pub fn filter_by_kind(kind: ColormapKind) -> Vec<&'static Colormap> {
    let mut result = Vec::new();
    for_each_colormap(|cm| {
        if cm.meta.kind == kind {
            result.push(cm);
        }
    });
    result
}

/// Return all colormaps from a given collection.
///
/// # Examples
///
/// ```
/// use prismatica::filter_by_collection;
/// let crameri = filter_by_collection("crameri");
/// assert!(crameri.iter().all(|cm| cm.meta.collection == "crameri"));
/// ```
#[must_use]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
pub fn filter_by_collection(collection: &str) -> Vec<&'static Colormap> {
    let mut result = Vec::new();
    for_each_colormap(|cm| {
        if cm.meta.collection == collection {
            result.push(cm);
        }
    });
    result
}

/// Returns all discrete palettes enabled by the current feature flags.
///
/// # Examples
///
/// ```
/// use prismatica::all_discrete_palettes;
/// let palettes = all_discrete_palettes();
/// for p in &palettes {
///     assert!(!p.is_empty());
/// }
/// ```
#[must_use]
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
pub fn all_discrete_palettes() -> Vec<&'static DiscretePalette> {
    let mut result = Vec::new();
    for_each_discrete_palette(|p| result.push(p));
    result
}

/// Look up a discrete palette by its canonical name (case-sensitive).
///
/// Names use their original casing from upstream sources (e.g.,
/// `"Set2"`, `"Tableau10"`).
///
/// # Examples
///
/// ```ignore
/// use prismatica::find_palette_by_name;
/// let set2 = find_palette_by_name("Set2").expect("Set2 should exist");
/// assert_eq!(set2.meta.collection, "colorbrewer");
/// ```
#[must_use]
pub fn find_palette_by_name(name: &str) -> Option<&'static DiscretePalette> {
    let mut found = None;
    for_each_discrete_palette(|p| {
        if p.meta.name == name {
            found = Some(p);
        }
    });
    found
}
