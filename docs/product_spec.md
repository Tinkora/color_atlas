# color_atlas Product Specification

## Positioning

`color_atlas` is a browser-local color inspection workbench. It combines common color conversion, image palette extraction, harmony generation, WCAG contrast checks, color-vision preview, and CSS gradient generation in one account-free tool that can run offline after the assets are available.

## User Workflows

- A frontend developer checks text/background contrast before shipping a design system.
- A designer extracts candidate colors from a local image without uploading an asset.
- A developer copies CSS colors, gradients, and harmony variants without opening a full design suite.
- An agent or script uses stable input/output contracts for color conversion and contrast checks.

Mature alternatives include WebAIM Contrast Checker, Adobe Color, Coolors, browser developer tools, and language color libraries. `color_atlas` is differentiated by combining these narrow workflows in one no-account, browser-local page backed by an auditable Rust/WASM core, with explicit accuracy boundaries.

## Core Contract

1. Accept `#RGB`, `#RRGGBB`, `#RRGGBBAA`, and unprefixed ASCII hex strings.
2. Decode images in the browser only. The core API requires an RGBA buffer that exactly matches the declared dimensions and limits input to 16,777,216 pixels.
3. Palette extraction returns at most 20 colors. Tiny or transparent images may return fewer colors than requested; the API does not pad results with black.
4. Fully transparent pixels are excluded from clustering. Partially transparent pixels currently contribute their RGB channels.
5. WCAG calculations use sRGB relative luminance and contrast ratio. Input alpha is not composited; callers must choose a background when compositing is required.
6. Color-vision features are lightweight sRGB preview approximations, not a clinical simulation or medical advice.
7. Agent schemas document invocation contracts. They do not claim that this repository provides an MCP transport or hosted service.

## Scope

- Convert hex, RGB, HSL, HSV, CIELAB, and CSS color representations.
- Extract dominant colors using deterministic k-means++ initialization and at most 10,000 samples.
- Generate complementary, analogous, triadic, tetradic, monochromatic, shade, and tint variants.
- Check WCAG 2.1 AA/AAA thresholds for normal and large text.
- Generate copy-ready CSS `linear-gradient` values with at least two stops.
- Preview protanopia, deuteranopia, and tritanopia approximations.

## Non-goals

- No upload, persistence, or synchronization of user images.
- No Figma/Sketch/XD design-file export.
- No ICC profile, CMYK, OKLCH, or complete color-management implementation.
- No AI palette recommendations, color naming, or brand-semantic matching.
- No claim that color-vision previews diagnose accessibility or medical conditions.

## References

- [W3C WCAG 2.1 Contrast (Minimum)](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html)
- [W3C Relative Luminance](https://www.w3.org/WAI/GL/wiki/Relative_luminance)
- [MDN CSS color values](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value)

## Acceptance Criteria

- Non-ASCII and invalid hex input returns a stable error without panicking.
- HSL/HSV round trips stay within one channel value after rounding.
- Black/white contrast is approximately `21:1`; `#767676` on white passes normal-text AA while `#777777` does not.
- Oversized requests, mismatched pixel buffers, and zero-count requests return stable error codes.
- Fully transparent pixels do not pollute the extracted palette.
- Once assets are available locally, the page does not require an upload service to operate.
