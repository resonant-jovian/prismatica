//! Colormap discovery and filtering.
//!
//! Functions for querying the full catalog of available colormaps at
//! runtime. Which colormaps are present depends on the enabled feature
//! flags.

use crate::{Colormap, ColormapKind};

/// Returns all colormaps enabled by the current feature flags.
pub fn all_colormaps() -> &'static [&'static Colormap] {
    todo!()
}

/// Look up a colormap by its canonical name (case-sensitive).
pub fn find_by_name(name: &str) -> Option<&'static Colormap> {
    todo!()
}

/// Return all colormaps of a given kind.
pub fn filter_by_kind(kind: ColormapKind) -> Vec<&'static Colormap> {
    todo!()
}

/// Return all colormaps from a given collection.
pub fn filter_by_collection(collection: &str) -> Vec<&'static Colormap> {
    todo!()
}
