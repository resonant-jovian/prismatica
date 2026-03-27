//! Convenience re-exports for common prismatica types.
//!
//! ```
//! use prismatica::prelude::*;
//!
//! // Look up and sample a colormap
//! let cm = find_by_name("batlow").expect("batlow exists");
//! let color = cm.eval(0.5);
//! println!("midpoint: {color}");
//! ```

pub use crate::IntoFrameworkColor;
pub use crate::find_by_name;
pub use crate::find_palette_by_name;
pub use crate::{Color, Colormap, ColormapKind, ColormapMeta, DiscretePalette, ReversedColormap};

#[cfg(any(feature = "alloc", feature = "std"))]
pub use crate::{all_colormaps, all_discrete_palettes, filter_by_collection, filter_by_kind};
