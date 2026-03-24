/// An sRGB color with 8-bit channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color from RGB components.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        todo!()
    }

    /// Create a color from a 24-bit hex value (e.g., `0xFF8800`).
    pub const fn from_hex(hex: u32) -> Self {
        todo!()
    }

    /// Format the color as a CSS hex string (e.g., `"#ff8800"`).
    pub fn to_css_hex(self) -> String {
        todo!()
    }

    /// Convert to floating-point RGB in `[0.0, 1.0]`.
    pub const fn to_f32(self) -> (f32, f32, f32) {
        todo!()
    }

    /// Linear interpolation between two colors.
    ///
    /// `t` is clamped to `[0.0, 1.0]`. Interpolation is performed in
    /// sRGB space, matching the behavior of matplotlib, ParaView, and
    /// most scientific visualization tools.
    pub fn lerp(self, other: Color, t: f32) -> Color {
        todo!()
    }
}

/// The type/class of a colormap, following standard scientific nomenclature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColormapKind {
    /// Low to high, single direction (e.g., viridis, batlow, blues).
    Sequential,
    /// Two extremes diverging from a neutral center (e.g., RdBu, berlin, coolwarm).
    Diverging,
    /// Wraps around: end color equals start color (e.g., phase, twilight, romaO).
    Cyclic,
    /// Discrete distinct colors for categorical data (e.g., Category10, Set2).
    Qualitative,
    /// Multiple sequential ramps joined (e.g., oleron: land+ocean).
    MultiSequential,
}

/// Metadata about a colormap's scientific properties.
#[derive(Debug, Clone, Copy)]
pub struct ColormapMeta {
    /// Human-readable name, e.g. `"batlow"`.
    pub name: &'static str,
    /// Collection it belongs to (e.g., `"crameri"`, `"cet"`, `"matplotlib"`).
    pub collection: &'static str,
    /// Author or organization.
    pub author: &'static str,
    /// Classification.
    pub kind: ColormapKind,
    /// Whether this colormap is perceptually uniform.
    pub perceptually_uniform: bool,
    /// Whether this colormap is safe for the most common color vision
    /// deficiency (deuteranopia / protanopia, red-green).
    pub cvd_friendly: bool,
    /// Whether this colormap degrades gracefully to grayscale.
    pub grayscale_safe: bool,
    /// Number of entries in the LUT (typically 256).
    pub lut_size: usize,
    /// Citation string for academic use.
    pub citation: &'static str,
}

/// A continuous colormap backed by a precomputed lookup table.
///
/// This is the primary type for scientific colormaps. It stores N
/// (typically 256) evenly-spaced RGB samples and interpolates between
/// them for arbitrary input values.
#[derive(Debug, Clone, Copy)]
pub struct Colormap {
    pub meta: ColormapMeta,
    /// The lookup table. Always `&'static` because it's compiled in.
    pub lut: &'static [[u8; 3]],
}

impl Colormap {
    /// Sample the colormap at a continuous value `t` in `[0, 1]`.
    ///
    /// Values outside `[0, 1]` are clamped. Interpolation between
    /// LUT entries is linear in sRGB space.
    pub fn eval(&self, t: f32) -> Color {
        todo!()
    }

    /// Sample at a rational index: the `i`-th of `n` evenly-spaced values.
    ///
    /// Equivalent to `eval(i as f32 / (n - 1) as f32)` for `n > 1`.
    pub fn eval_rational(&self, i: usize, n: usize) -> Color {
        todo!()
    }

    /// Return a reversed view of this colormap. Zero allocation.
    pub fn reversed(&self) -> ReversedColormap<'_> {
        todo!()
    }

    /// Extract `n` evenly-spaced discrete colors from the colormap.
    pub fn colors(&self, n: usize) -> Vec<Color> {
        todo!()
    }

    /// Return the raw LUT as a slice of `[r, g, b]` arrays.
    pub fn lut_raw(&self) -> &'static [[u8; 3]] {
        todo!()
    }
}

/// A reversed view of a colormap. Zero allocation.
pub struct ReversedColormap<'a> {
    inner: &'a Colormap,
}

impl ReversedColormap<'_> {
    /// Sample the reversed colormap at `t` (equivalent to `inner.eval(1 - t)`).
    pub fn eval(&self, t: f32) -> Color {
        todo!()
    }
}

/// A discrete palette of distinct colors for categorical data.
///
/// Unlike [`Colormap`], a `DiscretePalette` has a fixed set of colors
/// and does not interpolate between them.
#[derive(Debug, Clone, Copy)]
pub struct DiscretePalette {
    pub meta: ColormapMeta,
    pub colors: &'static [[u8; 3]],
}

impl DiscretePalette {
    /// Get the `i`-th color (wraps around if `i >= len()`).
    pub fn get(&self, i: usize) -> Color {
        todo!()
    }

    /// Number of distinct colors in the palette.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Returns `true` if the palette contains no colors.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// All colors as a `Vec`.
    pub fn all_colors(&self) -> Vec<Color> {
        todo!()
    }
}
