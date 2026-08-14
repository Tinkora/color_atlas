# Security Policy

## Supported Versions

| Version | Supported |
| --- | --- |
| 0.1.0-alpha.x | Yes |

## Reporting a Vulnerability

If you discover a security vulnerability, please **do not** open a public issue.
Use GitHub private vulnerability reporting for this repository. If that private
channel is unavailable, do not publish vulnerability details; contact the
Tinkora organization owner through an already established private channel. No
response-time guarantee is made until a monitored security contact is published.

### Scope

The following areas are within scope:

- Input validation bypasses (hex strings, image pixel data, JSON)
- WASM sandbox escapes
- Denial-of-service through crafted inputs (e.g., excessive pixel arrays)
- Integer overflow in color computation or palette extraction

### Out of Scope

- Issues already documented as known limitations
- Theoretical attacks requiring physical access
- Issues in dependencies (please report upstream)

## Security Model

The color_atlas project follows these security principles:

1. **Browser-local by default**: All color processing and palette extraction happens
   in-browser via WASM. No user data touches any server.

2. **No code execution from user input**: All inputs (hex strings, JSON, pixel bytes)
   are parsed through safe Rust primitives. No `eval`-style execution paths exist.

3. **Safe rendering**: User-controlled values are assigned through DOM APIs such
   as `textContent`; they are never interpolated into executable markup.

4. **Input size limits**: Image pixel arrays are bounded to 16,777,216 pixels,
   palette requests to 20 colors, and sampling to 10,000 pixels to limit memory
   and CPU use during k-means extraction.
