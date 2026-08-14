pub mod color;
pub mod contrast;
pub mod error;
pub mod palette;
pub mod wasm;

pub use color::{
    Color, color_to_css, color_to_hex, hex_to_color, hsl_to_rgb, hsv_to_rgb, rgb_to_hsl,
    rgb_to_hsv, rgb_to_lab,
};
pub use contrast::{
    WcagLevel, contrast_ratio, relative_luminance, wcag_aa_large, wcag_aa_normal, wcag_aaa_large,
    wcag_aaa_normal, wcag_level,
};
pub use error::CoreError;
pub use palette::{
    analogous, complementary, extract_palette, monochromatic, shades, simulate_deuteranopia,
    simulate_protanopia, simulate_tritanopia, tetradic, tints, triadic,
};
