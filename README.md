# color_atlas

`color_atlas` is a browser-local color workbench for developers and designers. It keeps image data and color calculations in the browser through Rust/WASM, so a palette can be inspected without uploading an asset to a service.

> Alpha: the public API and visual output may change while real workflows are collected.

## What It Does

- Convert colors between hex, RGB, HSL, HSV, CIELAB, and CSS forms.
- Extract up to 20 dominant colors from a PNG, JPEG, or WebP image.
- Generate complementary, analogous, triadic, tetradic, monochromatic, shade, and tint variants.
- Check WCAG 2.1 contrast ratios for normal and large text.
- Preview protanopia, deuteranopia, and tritanopia approximations.
- Build copy-ready multi-stop CSS gradients.

The editor processes input locally. Fully transparent pixels are ignored during palette extraction, and palette requests are bounded to protect browser memory. Contrast uses the RGB channels; alpha compositing is outside the current scope.

## Try It Locally

Requirements: Rust 1.95+, the `wasm32-unknown-unknown` target, `wasm-pack`, and Python 3.

```bash
git clone https://github.com/Tinkora/color_atlas.git
cd color_atlas
rustup target add wasm32-unknown-unknown
wasm-pack build --target web --release crates/color_atlas_web --out-dir static/pkg
python3 -m http.server 8080 --directory crates/color_atlas_web/static
```

Open <http://localhost:8080>. The hosted preview, when enabled, is <https://tinkora.github.io/color_atlas/>.

## Development Checks

```bash
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo check -p color_atlas_web --target wasm32-unknown-unknown --locked
```

## Scope and Accuracy

`color_atlas` is a focused color inspection and generation tool, not a design-file editor, image editor, color-management suite, or medical accessibility test. WCAG results follow the relative-luminance and contrast-ratio definitions in the [W3C WCAG 2.1 Understanding document](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html). Color-vision results are visual approximations and must not be treated as clinical advice.

The agent-facing files in `skills/` describe callable input/output contracts for integrations. They are documentation and schemas, not an MCP server or a network transport.

## Documentation

- [Product specification](docs/product_spec.md)
- [中文说明](README.zh-CN.md)
- [Contributing](CONTRIBUTING.md)
- [Security policy](SECURITY.md)
- [Support](SUPPORT.md)
- [Changelog](CHANGELOG.md)

## Support

[Support Tinkora on Ko-fi](https://ko-fi.com/tinkora)

## License

MIT. See [LICENSE](LICENSE).
