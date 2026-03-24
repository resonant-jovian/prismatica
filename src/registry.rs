//! Colormap discovery and filtering.
//!
//! Functions for querying the full catalog of available colormaps at
//! runtime. Which colormaps are present depends on the enabled feature
//! flags.

use crate::Colormap;

#[cfg(any(feature = "alloc", feature = "std"))]
use {alloc::vec::Vec, crate::ColormapKind};

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
    // Suppress unused variable warning when no collections are enabled
    let _ = &mut f;
}

/// Returns all colormaps enabled by the current feature flags.
#[cfg(any(feature = "alloc", feature = "std"))]
pub fn all_colormaps() -> Vec<&'static Colormap> {
    let mut result = Vec::new();
    for_each_colormap(|cm| result.push(cm));
    result
}

/// Look up a colormap by its canonical name (case-sensitive).
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
#[cfg(any(feature = "alloc", feature = "std"))]
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
#[cfg(any(feature = "alloc", feature = "std"))]
pub fn filter_by_collection(collection: &str) -> Vec<&'static Colormap> {
    let mut result = Vec::new();
    for_each_colormap(|cm| {
        if cm.meta.collection == collection {
            result.push(cm);
        }
    });
    result
}
