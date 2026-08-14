# color_atlas

`color_atlas` 是面向开发者和设计师的浏览器本地色彩工作台。图片和色彩计算通过 Rust/WASM 在浏览器中完成，不需要把素材上传到第三方服务。

> Alpha 版本：在收集真实工作流反馈期间，公开 API 和视觉输出仍可能调整。

## 功能

- 在 hex、RGB、HSL、HSV、CIELAB 和 CSS 表示之间转换颜色。
- 从 PNG、JPEG 或 WebP 图片中提取最多 20 个主色。
- 生成互补色、相似色、三角色、四角色、单色、暗色和明色变体。
- 检查普通文字和大文字的 WCAG 2.1 对比度比率。
- 预览红色盲、绿色盲和蓝色盲的近似效果。
- 生成可以直接复制的多色标 CSS 渐变。

编辑器在本地处理输入。提取调色板时会忽略完全透明的像素，并限制请求规模以保护浏览器内存。对比度计算只使用 RGB 通道；alpha 合成不在当前版本范围内。

## 本地运行

需要 Rust 1.95+、`wasm32-unknown-unknown` 目标、`wasm-pack` 和 Python 3。

```bash
git clone https://github.com/Tinkora/color_atlas.git
cd color_atlas
rustup target add wasm32-unknown-unknown
wasm-pack build --target web --release crates/color_atlas_web --out-dir static/pkg
python3 -m http.server 8080 --directory crates/color_atlas_web/static
```

打开 <http://localhost:8080>。启用 Pages 后的在线预览地址为 <https://tinkora.github.io/color_atlas/>。

## 开发检查

```bash
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo check -p color_atlas_web --target wasm32-unknown-unknown --locked
```

## 范围与准确性

`color_atlas` 是专注的色彩检查和生成工具，不是设计文件编辑器、图像编辑器、完整色彩管理套件，也不是医学无障碍测试工具。WCAG 结果遵循 [W3C WCAG 2.1 对比度说明](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html) 中的相对亮度和对比度比率定义。色觉差异结果只是视觉近似，不能作为临床结论。

`skills/` 中的 Agent 文件描述集成所需的输入/输出契约。它们是文档和 schema，不是 MCP server，也不提供网络传输。

## 文档

- [English documentation](README.md)
- [产品规格](docs/product_spec.zh-CN.md)
- [贡献指南](CONTRIBUTING.md)
- [安全策略](SECURITY.md)
- [支持](SUPPORT.md)
- [变更记录](CHANGELOG.md)

## 许可证

MIT，详见 [LICENSE](LICENSE)。
