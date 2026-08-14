//! WASM bindings for the color_atlas_core toolkit.
//!
//! All public functions are exported to JavaScript via `wasm-bindgen`.
//! Colors are passed as JSON objects `{"r":..., "g":..., "b":..., "a":...}`.

use crate::color::{self, Color};
use crate::contrast;
use crate::error::CoreError;
use crate::palette;
use wasm_bindgen::prelude::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Convert a `CoreError` into a `JsValue` with stable `code` and `message` fields.
fn core_err(e: CoreError) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"code".into(), &e.code().into()).ok();
    js_sys::Reflect::set(&obj, &"message".into(), &e.to_string().into()).ok();
    obj.into()
}

/// Deserialize a `Color` from a JSON string.
fn parse_color_json(json: &str) -> Result<Color, CoreError> {
    serde_json::from_str::<Color>(json).map_err(|e| CoreError::InvalidJson(e.to_string()))
}

/// Serialize a value to a JSON string at the JS boundary.
///
/// Returning one representation for every structured result keeps the API
/// stable across browsers and avoids relying on implementation details of
/// `serde_json::Value` conversion.
fn to_js<T: serde::Serialize>(value: &T) -> Result<JsValue, JsValue> {
    serde_json::to_string(value)
        .map(|json| JsValue::from_str(&json))
        .map_err(|e| JsValue::from_str(&format!("JSON serialization failed: {e}")))
}

// ---------------------------------------------------------------------------
// Color Conversion WASM exports
// ---------------------------------------------------------------------------

/// Parse a hex color string and return a Color object.
#[wasm_bindgen]
pub fn wasm_hex_to_color(hex: &str) -> Result<JsValue, JsValue> {
    let color = color::hex_to_color(hex).map_err(core_err)?;
    to_js(&color)
}

/// Convert a Color object to a hex string.
#[wasm_bindgen]
pub fn wasm_color_to_hex(color_json: &str) -> Result<String, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    Ok(color::color_to_hex(&color))
}

/// Convert a Color object to a CSS string (rgba or hex).
#[wasm_bindgen]
pub fn wasm_color_to_css(color_json: &str) -> Result<String, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    Ok(color::color_to_css(&color))
}

/// Convert a Color object to HSL. Returns { h, s, l }.
#[wasm_bindgen]
pub fn wasm_rgb_to_hsl(color_json: &str) -> Result<JsValue, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    let (h, s, l) = color::rgb_to_hsl(color.r, color.g, color.b);
    let result = serde_json::json!({
        "h": (h * 100.0).round() / 100.0,
        "s": (s * 100.0).round() / 100.0,
        "l": (l * 100.0).round() / 100.0
    });
    to_js(&result)
}

/// Convert HSL to a Color object. Takes { h, s, l } JSON.
#[wasm_bindgen]
pub fn wasm_hsl_to_rgb(hsl_json: &str) -> Result<JsValue, JsValue> {
    let v: serde_json::Value = serde_json::from_str(hsl_json)
        .map_err(|e| core_err(CoreError::InvalidJson(e.to_string())))?;
    let h = v["h"].as_f64().unwrap_or(0.0);
    let s = v["s"].as_f64().unwrap_or(0.0);
    let l = v["l"].as_f64().unwrap_or(0.0);
    let color = color::hsl_to_rgb(h, s, l);
    to_js(&color)
}

/// Convert a Color object to HSV. Returns { h, s, v }.
#[wasm_bindgen]
pub fn wasm_rgb_to_hsv(color_json: &str) -> Result<JsValue, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    let (h, s, v) = color::rgb_to_hsv(color.r, color.g, color.b);
    let result = serde_json::json!({
        "h": (h * 100.0).round() / 100.0,
        "s": (s * 100.0).round() / 100.0,
        "v": (v * 100.0).round() / 100.0
    });
    to_js(&result)
}

/// Convert HSV to a Color object. Takes { h, s, v } JSON.
#[wasm_bindgen]
pub fn wasm_hsv_to_rgb(hsv_json: &str) -> Result<JsValue, JsValue> {
    let v: serde_json::Value = serde_json::from_str(hsv_json)
        .map_err(|e| core_err(CoreError::InvalidJson(e.to_string())))?;
    let h = v["h"].as_f64().unwrap_or(0.0);
    let s = v["s"].as_f64().unwrap_or(0.0);
    let val = v["v"].as_f64().unwrap_or(0.0);
    let color = color::hsv_to_rgb(h, s, val);
    to_js(&color)
}

/// Convert a Color object to CIE L*a*b*. Returns { l, a, b }.
#[wasm_bindgen]
pub fn wasm_rgb_to_lab(color_json: &str) -> Result<JsValue, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    let (l, a, b) = color::rgb_to_lab(color.r, color.g, color.b);
    let result = serde_json::json!({
        "l": (l * 100.0).round() / 100.0,
        "a": (a * 100.0).round() / 100.0,
        "b": (b * 100.0).round() / 100.0
    });
    to_js(&result)
}

// ---------------------------------------------------------------------------
// Palette WASM exports
// ---------------------------------------------------------------------------

/// Extract a color palette from raw RGBA pixel data.
///
/// `rgba_pixels` is a flat `Uint8Array` of RGBA bytes.
/// Returns an array of Color objects.
#[wasm_bindgen]
pub fn wasm_extract_palette(
    rgba_pixels: &[u8],
    width: u32,
    height: u32,
    count: u32,
) -> Result<JsValue, JsValue> {
    let colors = palette::extract_palette(rgba_pixels, width, height, count).map_err(core_err)?;
    to_js(&colors)
}

/// Generate complementary color.
#[wasm_bindgen]
pub fn wasm_complementary(color_json: &str) -> Result<JsValue, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    let c = palette::complementary(&color);
    to_js(&c)
}

/// Generate analogous colors (5).
#[wasm_bindgen]
pub fn wasm_analogous(color_json: &str) -> Result<JsValue, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    let colors = palette::analogous(&color);
    to_js(&colors)
}

/// Generate triadic colors (3).
#[wasm_bindgen]
pub fn wasm_triadic(color_json: &str) -> Result<JsValue, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    let colors = palette::triadic(&color);
    to_js(&colors)
}

/// Generate tetradic colors (4).
#[wasm_bindgen]
pub fn wasm_tetradic(color_json: &str) -> Result<JsValue, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    let colors = palette::tetradic(&color);
    to_js(&colors)
}

/// Generate monochromatic palette.
#[wasm_bindgen]
pub fn wasm_monochromatic(color_json: &str, count: u32) -> Result<JsValue, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    let colors = palette::monochromatic(&color, count);
    to_js(&colors)
}

/// Generate shades (darker variations).
#[wasm_bindgen]
pub fn wasm_shades(color_json: &str, count: u32) -> Result<JsValue, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    let colors = palette::shades(&color, count);
    to_js(&colors)
}

/// Generate tints (lighter variations).
#[wasm_bindgen]
pub fn wasm_tints(color_json: &str, count: u32) -> Result<JsValue, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    let colors = palette::tints(&color, count);
    to_js(&colors)
}

// ---------------------------------------------------------------------------
// Color blindness simulation WASM exports
// ---------------------------------------------------------------------------

/// Simulate protanopia.
#[wasm_bindgen]
pub fn wasm_protanopia(color_json: &str) -> Result<JsValue, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    let c = palette::simulate_protanopia(&color);
    to_js(&c)
}

/// Simulate deuteranopia.
#[wasm_bindgen]
pub fn wasm_deuteranopia(color_json: &str) -> Result<JsValue, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    let c = palette::simulate_deuteranopia(&color);
    to_js(&c)
}

/// Simulate tritanopia.
#[wasm_bindgen]
pub fn wasm_tritanopia(color_json: &str) -> Result<JsValue, JsValue> {
    let color = parse_color_json(color_json).map_err(core_err)?;
    let c = palette::simulate_tritanopia(&color);
    to_js(&c)
}

// ---------------------------------------------------------------------------
// Contrast WASM exports
// ---------------------------------------------------------------------------

/// Compute contrast ratio between foreground and background colors.
/// Returns { ratio, aa_normal, aa_large, aaa_normal, aaa_large, level }.
#[wasm_bindgen]
pub fn wasm_contrast_ratio(fg_json: &str, bg_json: &str) -> Result<JsValue, JsValue> {
    let fg = parse_color_json(fg_json).map_err(core_err)?;
    let bg = parse_color_json(bg_json).map_err(core_err)?;
    let ratio = contrast::contrast_ratio(&fg, &bg);
    let level = contrast::wcag_level(&fg, &bg);
    let result = serde_json::json!({
        "ratio": (ratio * 100.0).round() / 100.0,
        "aa_normal": contrast::wcag_aa_normal(ratio),
        "aa_large": contrast::wcag_aa_large(ratio),
        "aaa_normal": contrast::wcag_aaa_normal(ratio),
        "aaa_large": contrast::wcag_aaa_large(ratio),
        "level": level,
        "level_label": level.label(),
        "level_label_zh_cn": level.label_cn(),
        "level_css": level.css_class()
    });
    to_js(&result)
}
