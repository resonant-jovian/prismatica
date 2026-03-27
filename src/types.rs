/// An sRGB color with 8-bit channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl core::fmt::Display for Color {
    /// Formats the color as a CSS hex string (e.g., `#ff8800`).
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl Color {
    /// Create a new color from RGB components.
    ///
    /// # Examples
    ///
    /// ```
    /// use prismatica::Color;
    /// let orange = Color::new(255, 165, 0);
    /// assert_eq!(orange.r, 255);
    /// ```
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create a color from a 24-bit hex value (e.g., `0xFF8800`).
    ///
    /// # Examples
    ///
    /// ```
    /// use prismatica::Color;
    /// let orange = Color::from_hex(0xFF8800);
    /// assert_eq!(orange, Color::new(255, 136, 0));
    /// ```
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    /// Serialize the color as a 24-bit hex value (e.g., `0xFF8800`).
    pub const fn to_hex(self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
    }

    /// Format the color as a CSS hex string (e.g., `"#ff8800"`).
    #[cfg(any(feature = "alloc", feature = "std"))]
    pub fn to_css_hex(self) -> alloc::string::String {
        alloc::format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Convert to floating-point RGB in `[0.0, 1.0]`.
    pub const fn to_f32(self) -> (f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
    }

    /// Relative luminance per WCAG 2.0.
    ///
    /// Returns a value in `[0.0, 1.0]` where 0 is black and 1 is white.
    ///
    /// # Examples
    ///
    /// ```
    /// use prismatica::Color;
    /// let white = Color::new(255, 255, 255);
    /// assert!((white.luminance() - 1.0).abs() < 0.01);
    /// ```
    pub fn luminance(self) -> f64 {
        let r = srgb_to_linear(self.r as f64 / 255.0);
        let g = srgb_to_linear(self.g as f64 / 255.0);
        let b = srgb_to_linear(self.b as f64 / 255.0);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// WCAG contrast ratio between two colors.
    ///
    /// Returns a value in `[1.0, 21.0]`. A ratio of 4.5+ meets WCAG AA
    /// for normal text; 7.0+ meets AAA.
    ///
    /// # Examples
    ///
    /// ```
    /// use prismatica::Color;
    /// let ratio = Color::new(0, 0, 0).contrast_ratio(Color::new(255, 255, 255));
    /// assert!((ratio - 21.0).abs() < 0.1);
    /// ```
    pub fn contrast_ratio(self, other: Color) -> f64 {
        let l1 = self.luminance();
        let l2 = other.luminance();
        let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
        (lighter + 0.05) / (darker + 0.05)
    }

    /// Linear interpolation between two colors.
    ///
    /// `t` is clamped to `[0.0, 1.0]`. Interpolation is performed in
    /// sRGB space, matching the behavior of matplotlib, ParaView, and
    /// most scientific visualization tools.
    pub fn lerp(self, other: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
        }
    }
}

fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.03928 {
        c / 12.92
    } else {
        libm::pow((c + 0.055) / 1.055, 2.4)
    }
}

/// The type/class of a colormap, following standard scientific nomenclature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
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

impl core::fmt::Display for ColormapKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Sequential => write!(f, "Sequential"),
            Self::Diverging => write!(f, "Diverging"),
            Self::Cyclic => write!(f, "Cyclic"),
            Self::Qualitative => write!(f, "Qualitative"),
            Self::MultiSequential => write!(f, "Multi-sequential"),
        }
    }
}

/// Metadata about a colormap's scientific properties.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
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
    ///
    /// # Examples
    ///
    /// ```
    /// use prismatica::crameri::BATLOW;
    /// let color = BATLOW.eval(0.5);
    /// assert!(color.r <= 255);
    /// ```
    pub fn eval(&self, t: f32) -> Color {
        debug_assert!(!self.lut.is_empty(), "Colormap LUT must not be empty");
        let t = t.clamp(0.0, 1.0);
        let n = self.lut.len();
        let scaled = t * (n - 1) as f32;
        let idx = scaled as usize;
        let frac = scaled - idx as f32;

        if idx >= n - 1 {
            let [r, g, b] = self.lut[n - 1];
            return Color::new(r, g, b);
        }

        let [r0, g0, b0] = self.lut[idx];
        let [r1, g1, b1] = self.lut[idx + 1];

        Color::new(
            (r0 as f32 + (r1 as f32 - r0 as f32) * frac) as u8,
            (g0 as f32 + (g1 as f32 - g0 as f32) * frac) as u8,
            (b0 as f32 + (b1 as f32 - b0 as f32) * frac) as u8,
        )
    }

    /// Sample at a rational index: the `i`-th of `n` evenly-spaced values.
    ///
    /// Equivalent to `eval(i as f32 / (n - 1) as f32)` for `n > 1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use prismatica::crameri::BATLOW;
    /// // The 5th of 10 evenly-spaced samples
    /// let color = BATLOW.eval_rational(5, 10);
    /// ```
    pub fn eval_rational(&self, i: usize, n: usize) -> Color {
        if n <= 1 {
            return self.eval(0.0);
        }
        self.eval(i as f32 / (n - 1) as f32)
    }

    /// Return a reversed view of this colormap. Zero allocation.
    ///
    /// # Examples
    ///
    /// ```
    /// use prismatica::crameri::BATLOW;
    /// let rev = BATLOW.reversed();
    /// assert_eq!(rev.eval(0.0), BATLOW.eval(1.0));
    /// ```
    pub fn reversed(&self) -> ReversedColormap<'_> {
        ReversedColormap { inner: self }
    }

    /// Extract `n` evenly-spaced discrete colors from the colormap.
    #[cfg(any(feature = "alloc", feature = "std"))]
    pub fn colors(&self, n: usize) -> alloc::vec::Vec<Color> {
        (0..n).map(|i| self.eval_rational(i, n)).collect()
    }

    /// Return the raw LUT as a slice of `[r, g, b]` arrays.
    pub fn lut_raw(&self) -> &'static [[u8; 3]] {
        self.lut
    }
}

/// A reversed view of a colormap. Zero allocation.
pub struct ReversedColormap<'a> {
    inner: &'a Colormap,
}

impl ReversedColormap<'_> {
    /// Sample the reversed colormap at `t` (equivalent to `inner.eval(1 - t)`).
    pub fn eval(&self, t: f32) -> Color {
        self.inner.eval(1.0 - t)
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
    ///
    /// # Examples
    ///
    /// ```
    /// use prismatica::colorbrewer::SET2_PALETTE;
    /// let first = SET2_PALETTE.get(0);
    /// // Wraps around: index 8 == index 0 for an 8-color palette
    /// assert_eq!(SET2_PALETTE.get(SET2_PALETTE.len()), first);
    /// ```
    pub fn get(&self, i: usize) -> Color {
        let [r, g, b] = self.colors[i % self.colors.len()];
        Color::new(r, g, b)
    }

    /// Number of distinct colors in the palette.
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Returns `true` if the palette contains no colors.
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// All colors as a `Vec`.
    #[cfg(any(feature = "alloc", feature = "std"))]
    pub fn all_colors(&self) -> alloc::vec::Vec<Color> {
        self.colors
            .iter()
            .map(|[r, g, b]| Color::new(*r, *g, *b))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn color_new() {
        let c = Color::new(255, 128, 0);
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 128);
        assert_eq!(c.b, 0);
    }

    #[test]
    fn color_from_hex() {
        let c = Color::from_hex(0xFF8800);
        assert_eq!(c, Color::new(255, 136, 0));
    }

    #[test]
    fn color_to_f32() {
        let c = Color::new(255, 0, 128);
        let (r, g, b) = c.to_f32();
        assert!((r - 1.0).abs() < 0.001);
        assert!(g.abs() < 0.001);
        assert!((b - 128.0 / 255.0).abs() < 0.001);
    }

    #[cfg(any(feature = "alloc", feature = "std"))]
    #[test]
    fn color_to_css_hex() {
        let c = Color::new(255, 136, 0);
        assert_eq!(c.to_css_hex(), "#ff8800");
    }

    #[test]
    fn color_lerp_boundaries() {
        let a = Color::new(0, 0, 0);
        let b = Color::new(255, 255, 255);
        assert_eq!(a.lerp(b, 0.0), a);
        assert_eq!(a.lerp(b, 1.0), b);
    }

    #[test]
    fn color_lerp_midpoint() {
        let a = Color::new(0, 0, 0);
        let b = Color::new(200, 100, 50);
        let mid = a.lerp(b, 0.5);
        assert_eq!(mid, Color::new(100, 50, 25));
    }

    #[test]
    fn color_lerp_clamps() {
        let a = Color::new(100, 100, 100);
        let b = Color::new(200, 200, 200);
        assert_eq!(a.lerp(b, -1.0), a);
        assert_eq!(a.lerp(b, 2.0), b);
    }

    static TEST_LUT: [[u8; 3]; 3] = [[0, 0, 0], [128, 128, 128], [255, 255, 255]];

    fn test_colormap() -> Colormap {
        Colormap {
            meta: ColormapMeta {
                name: "test",
                collection: "test",
                author: "test",
                kind: ColormapKind::Sequential,
                perceptually_uniform: true,
                cvd_friendly: true,
                grayscale_safe: true,
                lut_size: 3,
                citation: "",
            },
            lut: &TEST_LUT,
        }
    }

    #[test]
    fn colormap_eval_boundaries() {
        let cm = test_colormap();
        assert_eq!(cm.eval(0.0), Color::new(0, 0, 0));
        assert_eq!(cm.eval(1.0), Color::new(255, 255, 255));
    }

    #[test]
    fn colormap_eval_clamps() {
        let cm = test_colormap();
        assert_eq!(cm.eval(-1.0), cm.eval(0.0));
        assert_eq!(cm.eval(2.0), cm.eval(1.0));
    }

    #[test]
    fn colormap_eval_midpoint() {
        let cm = test_colormap();
        let mid = cm.eval(0.5);
        assert_eq!(mid, Color::new(128, 128, 128));
    }

    #[test]
    fn colormap_reversed() {
        let cm = test_colormap();
        let rev = cm.reversed();
        assert_eq!(rev.eval(0.0), cm.eval(1.0));
        assert_eq!(rev.eval(1.0), cm.eval(0.0));
    }

    #[test]
    fn colormap_eval_rational() {
        let cm = test_colormap();
        assert_eq!(cm.eval_rational(0, 3), cm.eval(0.0));
        assert_eq!(cm.eval_rational(2, 3), cm.eval(1.0));
    }

    #[test]
    fn colormap_lut_raw() {
        let cm = test_colormap();
        assert_eq!(cm.lut_raw().len(), 3);
    }

    static TEST_PALETTE_COLORS: [[u8; 3]; 3] = [[255, 0, 0], [0, 255, 0], [0, 0, 255]];

    #[test]
    fn discrete_palette_get() {
        let p = DiscretePalette {
            meta: ColormapMeta {
                name: "test",
                collection: "test",
                author: "test",
                kind: ColormapKind::Qualitative,
                perceptually_uniform: false,
                cvd_friendly: false,
                grayscale_safe: false,
                lut_size: 3,
                citation: "",
            },
            colors: &TEST_PALETTE_COLORS,
        };
        assert_eq!(p.get(0), Color::new(255, 0, 0));
        assert_eq!(p.get(1), Color::new(0, 255, 0));
        assert_eq!(p.get(3), Color::new(255, 0, 0)); // wraps
        assert_eq!(p.len(), 3);
        assert!(!p.is_empty());
    }

    #[test]
    fn color_display() {
        let c = Color::new(255, 136, 0);
        assert_eq!(format!("{c}"), "#ff8800");
        assert_eq!(format!("{}", Color::new(0, 0, 0)), "#000000");
    }

    #[test]
    fn colormap_kind_display() {
        assert_eq!(format!("{}", ColormapKind::Sequential), "Sequential");
        assert_eq!(format!("{}", ColormapKind::Diverging), "Diverging");
        assert_eq!(format!("{}", ColormapKind::Cyclic), "Cyclic");
        assert_eq!(format!("{}", ColormapKind::Qualitative), "Qualitative");
        assert_eq!(
            format!("{}", ColormapKind::MultiSequential),
            "Multi-sequential"
        );
    }

    #[test]
    fn color_to_hex_roundtrip() {
        assert_eq!(Color::from_hex(0xFF8800).to_hex(), 0xFF8800);
        assert_eq!(Color::new(0, 0, 0).to_hex(), 0x000000);
        assert_eq!(Color::new(255, 255, 255).to_hex(), 0xFFFFFF);
    }

    #[test]
    fn color_luminance_black_white() {
        let black = Color::new(0, 0, 0).luminance();
        let white = Color::new(255, 255, 255).luminance();
        assert!(black < 0.01, "black luminance should be ~0, got {black}");
        assert!(
            (white - 1.0).abs() < 0.01,
            "white luminance should be ~1, got {white}"
        );
    }

    #[test]
    fn color_contrast_ratio_bw() {
        let ratio = Color::new(0, 0, 0).contrast_ratio(Color::new(255, 255, 255));
        assert!(
            (ratio - 21.0).abs() < 0.1,
            "black/white contrast should be ~21, got {ratio}"
        );
    }

    #[test]
    fn color_contrast_ratio_symmetric() {
        let a = Color::new(100, 50, 200);
        let b = Color::new(200, 150, 50);
        let ab = a.contrast_ratio(b);
        let ba = b.contrast_ratio(a);
        assert!(
            (ab - ba).abs() < 0.001,
            "contrast ratio should be symmetric"
        );
    }
}
