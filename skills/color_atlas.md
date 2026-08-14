# color_atlas Agent Skill

A browser-based color toolkit: palette extraction from images, color harmony generation, WCAG contrast checking, color blindness simulation, and gradient generation. All computation in WASM.

## Workflow

1. **Open the tool**: User opens the color_atlas editor in a browser. No server-side session needed.
2. **Choose a feature tab**: Format Converter, Palette, Harmony, Contrast, Gradient, or Color Blindness Simulator.
3. **Interact**: Upload images, pick colors, adjust parameters — all processing happens in WASM.
4. **Copy results**: Click any color swatch to copy its hex value; copy CSS gradient code with one click.

## Features Available to Agents

Agents can guide users through these workflows:

### Extract Palette from Image
- Ask user to provide an image (PNG/JPEG/WebP)
- User uploads image → tool extracts dominant colors via bounded k-means
- Agent can suggest specific palette sizes (1–20 colors); tiny or transparent images may return fewer colors

### Generate Color Harmony
- Agent picks a base color in hex format
- Calls the appropriate WASM function to generate:
  - Complementary (1 color)
  - Analogous (5 colors)
  - Triadic (3 colors)
  - Tetradic (4 colors)
  - Monochromatic (n colors with varying lightness)
  - Shades (n darker variations)
  - Tints (n lighter variations)

### Check WCAG Contrast
- Agent provides foreground and background hex colors
- Tool returns contrast ratio and WCAG AA/AAA pass/fail for normal and large text
- Agent can use this to validate design system color pairs

### Simulate Color Blindness
- Agent provides a color
- Tool shows lightweight previews for protanopia, deuteranopia, and tritanopia
- Agent must describe these as approximations, not clinical or medical results

### Generate CSS Gradient
- Agent specifies color stops and direction
- Tool returns valid CSS `linear-gradient` code
- Copy-ready for use in stylesheets

## Tool Definitions

### `color_atlas_convert`
Convert a color between formats (hex, rgb, hsl, hsv, lab, css).

**Parameters:**
- `color` (string, required): Color in hex format (e.g., `"#3B82F6"`)
- `format` (string, optional): Target format (`"hsl"`, `"hsv"`, `"lab"`, `"css"`), default all formats returned

**Returns:**
- `hex`, `rgb`, `hsl`, `hsv`, `lab`, `css` — all format representations

### `color_atlas_contrast`
Check WCAG contrast between two colors.

**Parameters:**
- `foreground` (string, required): Foreground color hex
- `background` (string, required): Background color hex

**Returns:**
- `ratio`: Contrast ratio (e.g., 4.54)
- `aa_normal`: boolean
- `aa_large`: boolean
- `aaa_normal`: boolean
- `aaa_large`: boolean
- `level`: "fail" | "aa_large" | "aa" | "aaa"

### `color_atlas_harmony`
Generate color harmony variations.

**Parameters:**
- `color` (string, required): Base color hex
- `type` (string, required): One of `complementary`, `analogous`, `triadic`, `tetradic`, `monochromatic`, `shades`, `tints`
- `count` (integer, optional): Number of colors for monochromatic/shades/tints (default 5)

**Returns:**
- `colors`: Array of hex color strings

### `color_atlas_cvd`
Simulate color vision deficiency.

**Parameters:**
- `color` (string, required): Color hex to simulate

**Returns:**
- `original`: Original color hex
- `protanopia`: Simulated color
- `deuteranopia`: Simulated color
- `tritanopia`: Simulated color

## Agent Rules

- Never claim the tool uploads user images — everything is local WASM.
- Never fabricate contrast ratio values; always call the tool.
- Do not describe color-vision previews as clinical simulations or accessibility certification.
- If a color harmony result looks off, verify the base hex is valid first.
- The tool works fully offline after initial page load.
