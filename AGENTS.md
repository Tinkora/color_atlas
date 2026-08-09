# Repository Guide for AI Agents

## Project Overview

color_atlas is a browser-first color toolkit. It provides palette extraction from images, color harmony generation, WCAG contrast checking, color blindness simulation, and gradient generation — all in Rust/WASM running in the browser.

## Architecture

```
color_atlas/
├── crates/
│   ├── color_atlas_core/        # Color model, palette extraction, contrast, harmony
│   └── color_atlas_web/         # WASM bridge + HTML editor
├── docs/                         # Product spec
├── skills/                       # Agent Skill definitions (MCP tools)
└── index.html                    # Product landing page
```

## Key Files for AI Context

| File | Purpose |
|------|---------|
| `crates/color_atlas_core/src/color.rs` | Color type, hex/rgb/hsl/hsv/lab conversions |
| `crates/color_atlas_core/src/palette.rs` | k-means palette extraction, color harmony |
| `crates/color_atlas_core/src/contrast.rs` | WCAG contrast ratio and level classification |
| `crates/color_atlas_core/src/error.rs` | CoreError with stable machine codes |
| `crates/color_atlas_core/src/wasm.rs` | WASM bindings for JS interop |
| `crates/color_atlas_web/src/lib.rs` | Web crate WASM exports |
| `crates/color_atlas_web/static/index.html` | Full-featured editor UI |
| `skills/color_atlas.md` | Agent usage workflow |
| `skills/mcp-tools.json` | MCP tool definitions |

## Build & Test Commands

```bash
# Run all tests
cargo test --workspace

# Format check
cargo fmt --all -- --check

# Lint (strict)
cargo clippy --workspace --all-targets -- -D warnings

# WASM compilation check
cargo check -p color_atlas_web --target wasm32-unknown-unknown

# Build Web WASM for deployment
wasm-pack build --target web crates/color_atlas_web
```

## Design Principles

1. **Browser-first**: All color computation runs in-browser via WASM; no server required.
2. **Color-space aware**: Internally use linear sRGB for WCAG luminance calculations per the spec.
3. **K-means on raw bytes**: Palette extraction uses a simple k-means implementation on raw RGBA pixels — no image crate dependency needed.
4. **Stable error codes**: Every CoreError variant has a machine-readable `code()` for JS consumers.
5. **All WASM**: Every feature (format conversion, harmony, contrast, palette extraction) is backed by Rust/WASM, not JavaScript.

## Color Model

- `Color` struct: `r`, `g`, `b`, `a` (all `u8`)
- Supports hex parsing: `#RGB`, `#RRGGBB`, `#RRGGBBAA`
- HSL/HSV: hue in degrees [0, 360), saturation and lightness/value in [0.0, 1.0]
- WCAG relative luminance uses sRGB → linear RGB conversion per the spec

## Error Codes (Stable Machine-Readable)

| Code | Meaning |
|------|---------|
| `INVALID_HEX_FORMAT` | Hex string did not match expected pattern |
| `INVALID_HEX_LENGTH` | Hex string length != 3, 6, or 8 after `#` |
| `INVALID_HEX_CHAR` | Non-hex character in hex string |
| `INVALID_JSON` | JSON deserialization failed |
| `EMPTY_PIXELS` | Pixel slice is empty |
| `INSUFFICIENT_PIXELS` | Fewer unique pixels than requested palette count |
| `ZERO_COUNT` | Palette/enumeration count must be >= 1 |

## Frontend Design Requirement

- Before creating, modifying, reviewing, or debugging any HTML page or user-facing frontend, invoke the `ui-ux-pro-max` skill.
- Run the skill's required `--design-system` search before editing, followed by relevant stack and UX searches.
- If `ui-ux-pro-max` is unavailable, stop frontend work and report the missing prerequisite.
- Verify the rendered result in a real browser at 375, 768, 1024, and 1440 pixel widths, including console, keyboard, accessibility, and overflow checks.
