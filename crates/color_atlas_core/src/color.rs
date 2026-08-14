use crate::error::CoreError;

/// An 8-bit RGBA color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create an opaque color from RGB components.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a color from RGBA components.
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

// ---------------------------------------------------------------------------
// Hex parsing
// ---------------------------------------------------------------------------

/// Parse a hex color string into a `Color`.
///
/// Supports formats:
/// - `#RGB`       → expanded to `#RRGGBB` with alpha=255
/// - `#RRGGBB`    → alpha=255
/// - `#RRGGBBAA`  → with alpha
/// - `RGB` (without `#`) is also accepted.
pub fn hex_to_color(hex: &str) -> Result<Color, CoreError> {
    let s = hex.strip_prefix('#').unwrap_or(hex);

    if s.is_empty() {
        return Err(CoreError::InvalidHexFormat(hex.to_string()));
    }

    // Validate the alphabet before byte slicing so malformed Unicode input
    // is rejected instead of being able to panic at a non-character boundary.
    if !s.is_ascii() {
        return Err(CoreError::InvalidHexChar);
    }

    let len = s.len();
    let parse = |hex_str: &str| -> Result<u8, CoreError> {
        u8::from_str_radix(hex_str, 16).map_err(|_| CoreError::InvalidHexChar)
    };

    match len {
        3 => {
            // #RGB
            let r = parse(&s[0..1].repeat(2))?;
            let g = parse(&s[1..2].repeat(2))?;
            let b = parse(&s[2..3].repeat(2))?;
            Ok(Color::rgb(r, g, b))
        }
        6 => {
            let r = parse(&s[0..2])?;
            let g = parse(&s[2..4])?;
            let b = parse(&s[4..6])?;
            Ok(Color::rgb(r, g, b))
        }
        8 => {
            let r = parse(&s[0..2])?;
            let g = parse(&s[2..4])?;
            let b = parse(&s[4..6])?;
            let a = parse(&s[6..8])?;
            Ok(Color::rgba(r, g, b, a))
        }
        _ => Err(CoreError::InvalidHexLength),
    }
}

/// Convert a `Color` back to a `#RRGGBB` hex string (or `#RRGGBBAA` if alpha ≠ 255).
pub fn color_to_hex(color: &Color) -> String {
    if color.a == 255 {
        format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b)
    } else {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            color.r, color.g, color.b, color.a
        )
    }
}

// ---------------------------------------------------------------------------
// RGB ↔ HSL
// ---------------------------------------------------------------------------

/// Convert RGB (0–255) to HSL.
///
/// Returns `(hue: 0–360, saturation: 0.0–1.0, lightness: 0.0–1.0)`.
pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let rn = r as f64 / 255.0;
    let gn = g as f64 / 255.0;
    let bn = b as f64 / 255.0;

    let max = rn.max(gn).max(bn);
    let min = rn.min(gn).min(bn);
    let delta = max - min;

    let l = (max + min) / 2.0;

    if delta == 0.0 {
        return (0.0, 0.0, l);
    }

    let s = if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let h = if rn >= max {
        (gn - bn) / delta + if gn < bn { 6.0 } else { 0.0 }
    } else if gn >= max {
        (bn - rn) / delta + 2.0
    } else {
        (rn - gn) / delta + 4.0
    };

    (h * 60.0, s, l)
}

/// Convert HSL to RGB.
///
/// `h` in degrees [0, 360), `s` and `l` in [0.0, 1.0].
pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> Color {
    let h = h % 360.0;
    let h = if h < 0.0 { h + 360.0 } else { h };

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (rn, gn, bn) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    fn clamp(v: f64) -> u8 {
        (v * 255.0).round().clamp(0.0, 255.0) as u8
    }

    Color::rgb(clamp(rn + m), clamp(gn + m), clamp(bn + m))
}

// ---------------------------------------------------------------------------
// RGB ↔ HSV
// ---------------------------------------------------------------------------

/// Convert RGB (0–255) to HSV.
///
/// Returns `(hue: 0–360, saturation: 0.0–1.0, value: 0.0–1.0)`.
pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let rn = r as f64 / 255.0;
    let gn = g as f64 / 255.0;
    let bn = b as f64 / 255.0;

    let max = rn.max(gn).max(bn);
    let min = rn.min(gn).min(bn);
    let delta = max - min;

    let v = max;

    if delta == 0.0 {
        return (0.0, 0.0, v);
    }

    let s = delta / max;

    let h = if rn >= max {
        (gn - bn) / delta + if gn < bn { 6.0 } else { 0.0 }
    } else if gn >= max {
        (bn - rn) / delta + 2.0
    } else {
        (rn - gn) / delta + 4.0
    };

    (h * 60.0, s, v)
}

/// Convert HSV to RGB.
///
/// `h` in degrees [0, 360), `s` and `v` in [0.0, 1.0].
pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> Color {
    let h = h % 360.0;
    let h = if h < 0.0 { h + 360.0 } else { h };

    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (rn, gn, bn) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    fn clamp(v: f64) -> u8 {
        (v * 255.0).round().clamp(0.0, 255.0) as u8
    }

    Color::rgb(clamp(rn + m), clamp(gn + m), clamp(bn + m))
}

// ---------------------------------------------------------------------------
// RGB ↔ CIELAB (for perceptual work)
// ---------------------------------------------------------------------------

/// Convert sRGB (0–255) to CIE L*a*b* (D65 illuminant, 2° observer).
///
/// Returns `(L*: 0–100, a*: approximately -128..128, b*: approximately -128..128)`.
pub fn rgb_to_lab(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    // sRGB → linear
    fn linearize(c: u8) -> f64 {
        let cn = c as f64 / 255.0;
        if cn <= 0.04045 {
            cn / 12.92
        } else {
            ((cn + 0.055) / 1.055).powf(2.4)
        }
    }

    // linear RGB → XYZ (D65)
    let rl = linearize(r);
    let gl = linearize(g);
    let bl = linearize(b);

    let x = rl * 0.4124564 + gl * 0.3575761 + bl * 0.1804375;
    let y = rl * 0.2126729 + gl * 0.7151522 + bl * 0.0721750;
    let z = rl * 0.0193339 + gl * 0.1191920 + bl * 0.9503041;

    // XYZ → L*a*b*
    fn f(t: f64) -> f64 {
        let delta: f64 = 6.0 / 29.0;
        if t > delta.powi(3) {
            t.cbrt()
        } else {
            t / (3.0 * delta * delta) + 4.0 / 29.0
        }
    }

    let xn = 0.95047;
    let yn = 1.0;
    let zn = 1.08883;

    let l = 116.0 * f(y / yn) - 16.0;
    let a = 500.0 * (f(x / xn) - f(y / yn));
    let b = 200.0 * (f(y / yn) - f(z / zn));

    (l, a, b)
}

// ---------------------------------------------------------------------------
// CSS string
// ---------------------------------------------------------------------------

/// Format a color as a CSS string: `rgba(r, g, b, a)` or `#RRGGBB` if opaque.
pub fn color_to_css(color: &Color) -> String {
    if color.a == 255 {
        color_to_hex(color)
    } else {
        format!(
            "rgba({}, {}, {}, {:.3})",
            color.r,
            color.g,
            color.b,
            color.a as f64 / 255.0
        )
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_roundtrip() {
        let cases = ["#ff0000", "#00FF00", "#0000FF", "#f0a080"];
        for &c in &cases {
            let color = hex_to_color(c).unwrap();
            assert_eq!(color_to_hex(&color).to_lowercase(), c.to_lowercase());
        }
    }

    #[test]
    fn hex_3digit() {
        let c = hex_to_color("#f00").unwrap();
        assert_eq!(c, Color::rgb(255, 0, 0));
    }

    #[test]
    fn hex_8digit() {
        let c = hex_to_color("#ff000080").unwrap();
        assert_eq!(c, Color::rgba(255, 0, 0, 128));
    }

    #[test]
    fn hex_no_prefix() {
        let c = hex_to_color("ff8800").unwrap();
        assert_eq!(c, Color::rgb(255, 136, 0));
    }

    #[test]
    fn hex_invalid() {
        assert!(hex_to_color("").is_err());
        assert!(hex_to_color("#GGG").is_err());
        assert!(hex_to_color("#12345").is_err());
    }

    #[test]
    fn hex_non_ascii_is_rejected_without_panicking() {
        assert_eq!(hex_to_color("€€"), Err(CoreError::InvalidHexChar));
    }

    #[test]
    fn hsl_roundtrip() {
        let color = Color::rgb(200, 100, 50);
        let (h, s, l) = rgb_to_hsl(color.r, color.g, color.b);
        let back = hsl_to_rgb(h, s, l);
        assert!((color.r as i16 - back.r as i16).abs() <= 1);
        assert!((color.g as i16 - back.g as i16).abs() <= 1);
        assert!((color.b as i16 - back.b as i16).abs() <= 1);
    }

    #[test]
    fn hsv_roundtrip() {
        let color = Color::rgb(120, 200, 80);
        let (h, s, v) = rgb_to_hsv(color.r, color.g, color.b);
        let back = hsv_to_rgb(h, s, v);
        assert!((color.r as i16 - back.r as i16).abs() <= 1);
        assert!((color.g as i16 - back.g as i16).abs() <= 1);
        assert!((color.b as i16 - back.b as i16).abs() <= 1);
    }

    #[test]
    fn css_opaque() {
        let c = Color::rgb(255, 0, 0);
        assert_eq!(color_to_css(&c), "#FF0000");
    }

    #[test]
    fn css_transparent() {
        let c = Color::rgba(0, 128, 0, 128);
        assert_eq!(color_to_css(&c), "rgba(0, 128, 0, 0.502)");
    }

    #[test]
    fn lab_black() {
        let (l, a, b) = rgb_to_lab(0, 0, 0);
        assert!(l < 1.0);
        assert!(a.abs() < 0.1);
        assert!(b.abs() < 0.1);
    }

    #[test]
    fn lab_white() {
        let (l, _a, _b) = rgb_to_lab(255, 255, 255);
        assert!((l - 100.0).abs() < 0.5);
    }
}
