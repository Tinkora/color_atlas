use crate::color::Color;

/// WCAG 2.1 relative luminance of an sRGB color.
///
/// Uses the sRGB → linear RGB conversion defined in the WCAG 2.1 spec.
pub fn relative_luminance(color: &Color) -> f64 {
    fn linearize(c: u8) -> f64 {
        let s = c as f64 / 255.0;
        if s <= 0.04045 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    }

    0.2126 * linearize(color.r) + 0.7152 * linearize(color.g) + 0.0722 * linearize(color.b)
}

/// Compute the WCAG contrast ratio between two colors.
///
/// Ratio = (L_light + 0.05) / (L_dark + 0.05)
pub fn contrast_ratio(fg: &Color, bg: &Color) -> f64 {
    let l1 = relative_luminance(fg);
    let l2 = relative_luminance(bg);
    let lighter = if l1 > l2 { l1 } else { l2 };
    let darker = if l1 <= l2 { l1 } else { l2 };
    (lighter + 0.05) / (darker + 0.05)
}

/// WCAG 2.1 AA normal text: contrast ratio ≥ 4.5:1
pub fn wcag_aa_normal(ratio: f64) -> bool {
    ratio >= 4.5
}

/// WCAG 2.1 AA large text: contrast ratio ≥ 3.0:1
pub fn wcag_aa_large(ratio: f64) -> bool {
    ratio >= 3.0
}

/// WCAG 2.1 AAA normal text: contrast ratio ≥ 7.0:1
pub fn wcag_aaa_normal(ratio: f64) -> bool {
    ratio >= 7.0
}

/// WCAG 2.1 AAA large text: contrast ratio ≥ 4.5:1
pub fn wcag_aaa_large(ratio: f64) -> bool {
    ratio >= 4.5
}

/// Classification of WCAG contrast compliance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WcagLevel {
    /// Fails even AA large text (ratio < 3.0)
    Fail,
    /// Passes AA large text only (3.0 ≤ ratio < 4.5)
    AaLarge,
    /// Passes AA normal and large text (4.5 ≤ ratio < 7.0)
    Aa,
    /// Passes AAA (ratio ≥ 7.0)
    Aaa,
}

impl WcagLevel {
    /// Human-readable label in the default English locale.
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Fail => "Fail",
            Self::AaLarge => "AA large text",
            Self::Aa => "AA",
            Self::Aaa => "AAA",
        }
    }

    /// Human-readable label in Chinese.
    pub const fn label_cn(&self) -> &'static str {
        match self {
            Self::Fail => "不合格",
            Self::AaLarge => "AA 大文字",
            Self::Aa => "AA",
            Self::Aaa => "AAA",
        }
    }

    /// CSS class name for badge styling.
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Fail => "wcag-fail",
            Self::AaLarge => "wcag-aa-large",
            Self::Aa => "wcag-aa",
            Self::Aaa => "wcag-aaa",
        }
    }
}

/// Determine the WCAG compliance level for a foreground/background pair.
pub fn wcag_level(fg: &Color, bg: &Color) -> WcagLevel {
    let ratio = contrast_ratio(fg, bg);
    if ratio >= 7.0 {
        WcagLevel::Aaa
    } else if ratio >= 4.5 {
        WcagLevel::Aa
    } else if ratio >= 3.0 {
        WcagLevel::AaLarge
    } else {
        WcagLevel::Fail
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_white_contrast() {
        let black = Color::rgb(0, 0, 0);
        let white = Color::rgb(255, 255, 255);
        let ratio = contrast_ratio(&black, &white);
        assert!((ratio - 21.0).abs() < 0.1);
    }

    #[test]
    fn same_color_ratio_is_one() {
        let c = Color::rgb(128, 128, 128);
        let ratio = contrast_ratio(&c, &c);
        assert!((ratio - 1.0).abs() < 0.01);
    }

    #[test]
    fn wcag_level_black_on_white() {
        let black = Color::rgb(0, 0, 0);
        let white = Color::rgb(255, 255, 255);
        assert_eq!(wcag_level(&black, &white), WcagLevel::Aaa);
    }

    #[test]
    fn wcag_level_gray_on_gray() {
        let fg = Color::rgb(120, 120, 120);
        let bg = Color::rgb(128, 128, 128);
        assert_eq!(wcag_level(&fg, &bg), WcagLevel::Fail);
    }

    #[test]
    fn wcag_aa_normal_boundary() {
        // #767676 on white = ~4.54:1 just passes
        let fg = Color::rgb(118, 118, 118);
        let bg = Color::rgb(255, 255, 255);
        let ratio = contrast_ratio(&fg, &bg);
        assert!(wcag_aa_normal(ratio));

        // #777777 on white = ~4.48:1 just fails
        let fg2 = Color::rgb(119, 119, 119);
        let bg2 = Color::rgb(255, 255, 255);
        let ratio2 = contrast_ratio(&fg2, &bg2);
        assert!(!wcag_aa_normal(ratio2));
    }

    #[test]
    fn wcag_level_labels() {
        assert_eq!(WcagLevel::Fail.label(), "Fail");
        assert_eq!(WcagLevel::AaLarge.label(), "AA large text");
        assert_eq!(WcagLevel::Aa.label(), "AA");
        assert_eq!(WcagLevel::Aaa.label(), "AAA");
        assert_eq!(WcagLevel::Fail.label_cn(), "不合格");
        assert_eq!(WcagLevel::AaLarge.label_cn(), "AA 大文字");
        assert_eq!(WcagLevel::Aa.label_cn(), "AA");
        assert_eq!(WcagLevel::Aaa.label_cn(), "AAA");
    }
}
