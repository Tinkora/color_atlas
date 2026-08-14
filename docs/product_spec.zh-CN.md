# color_atlas 产品规格

## 产品定位

`color_atlas` 是一个浏览器本地色彩检查工作台。它把常见的颜色转换、图片主色提取、色彩组合、WCAG 对比度检查、色觉差异近似预览和 CSS 渐变生成放在同一个离线可用的工具中。

## 真实工作流

- 前端开发者在提交设计系统前检查文字与背景的 WCAG 比率。
- 设计师从受版权或隐私约束的本地图片中提取候选主色，不上传素材。
- 设计和实现人员需要快速复制 CSS 颜色、渐变和色彩组合，而不是打开完整设计套件。
- Agent 或脚本需要通过稳定输入/输出契约执行颜色转换和对比度检查。

成熟替代方案包括 WebAIM Contrast Checker、Adobe Color、Coolors、浏览器开发者工具和各语言颜色库。`color_atlas` 的差异是把这些窄工作流合并到一个无需账号、默认离线、Rust/WASM 核心驱动的页面，并明确声明其结果边界。

## 核心契约

1. 支持 `#RGB`、`#RRGGBB`、`#RRGGBBAA` 以及不带 `#` 的 ASCII hex 输入。
2. 图片只在浏览器中解码；核心 API 要求 RGBA 缓冲与声明尺寸精确匹配，并限制最多 16,777,216 个像素。
3. 调色板请求最多返回 20 个颜色。小图或透明图可能返回少于请求数量的颜色，不使用黑色填充伪造结果。
4. 完全透明的像素不参与调色板聚类；部分透明像素当前按其 RGB 通道参与。
5. WCAG 计算使用 sRGB 相对亮度和对比度比率。输入 alpha 不参与合成，调用方必须在需要时先决定合成背景。
6. 色觉差异功能只提供轻量级 sRGB 近似预览，不表示真实视觉体验或医疗建议。
7. Agent schema 描述调用契约，不宣称本仓库提供 MCP transport 或远程服务。

## 功能范围

- hex、RGB、HSL、HSV、CIELAB 和 CSS 颜色表示转换。
- k-means++ 初始化与最多 10,000 个采样点的主色提取。
- 互补、相似、三角色、四角色、单色、暗色和明色方案。
- WCAG 2.1 AA/AAA 普通文字和大文字检查。
- 最少两个色标的 CSS linear-gradient 生成。
- protanopia、deuteranopia、tritanopia 近似预览。

## 非目标

- 不上传、保存或同步用户图片。
- 不提供 Figma/Sketch/XD 设计文件导出。
- 不执行色彩配置文件、CMYK、OKLCH 或完整色彩管理。
- 不提供 AI 调色板推荐、色彩命名或品牌语义匹配。
- 不把色觉差异预览描述为医学检测。

## 参考依据

- [W3C WCAG 2.1 Contrast (Minimum)](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html)
- [W3C WCAG 2.1 Relative Luminance](https://www.w3.org/WAI/GL/wiki/Relative_luminance)
- [MDN: CSS color values](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value)

## 验收标准

- 非 ASCII 或非法 hex 输入返回稳定错误，不触发 panic。
- HSL/HSV 往返转换在取整后误差不超过 1。
- 黑白对比度返回约 `21:1`，`#767676` 在白底上通过 AA 普通文字而 `#777777` 不通过。
- 超过资源上限、尺寸不匹配的像素缓冲和零色数请求返回稳定错误码。
- 完全透明像素不污染主色结果。
- 页面在无网络情况下（首次资源已缓存或本地运行）不需要上传服务即可工作。
