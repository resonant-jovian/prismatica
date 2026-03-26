//! Convenience re-exports for common prismatica types.
//!
//! ```
//! use prismatica::prelude::*;
//! ```

pub use crate::{Color, Colormap, ColormapKind, ColormapMeta, DiscretePalette, ReversedColormap};
pub use crate::IntoFrameworkColor;
pub use crate::find_by_name;
pub use crate::find_palette_by_name;

#[cfg(any(feature = "alloc", feature = "std"))]
pub use crate::{all_colormaps, all_discrete_palettes, filter_by_collection, filter_by_kind};
