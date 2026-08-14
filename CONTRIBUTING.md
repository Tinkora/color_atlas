# Contributing to color_atlas

Thanks for your interest in color_atlas! Here's how to contribute.

## Development Environment

- Rust 1.95+ (stable)
- wasm-pack 0.15+
- wasm32-unknown-unknown target (`rustup target add wasm32-unknown-unknown`)

## Project Structure

```text
color_atlas/
├── crates/
│   ├── color_atlas_core/       # Color model, palette extraction, contrast, harmony
│   └── color_atlas_web/        # WASM bridge + HTML editor
├── docs/                        # Product spec
├── skills/                      # Agent Skill definitions
└── index.html                   # Landing page
```

## Local Development

```bash
# Run tests
cargo test --workspace --locked

# Format & lint
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

# Build Web WASM
wasm-pack build --target web --release crates/color_atlas_web --out-dir static/pkg

# Start local editor
cd crates/color_atlas_web/static && python3 -m http.server 8080
```

## Commit Convention

- Prefix: `feat:` / `fix:` / `docs:` / `refactor:` / `test:` / `chore:`
- Each commit should contain one logically complete change

## Pull Request Process

1. Fork the repo
2. Create a feature branch (`git checkout -b feat/your-feature`)
3. Commit your changes
4. Ensure the formatting, locked tests, Clippy, and WASM checks in the README pass
5. Push to your fork (`git push origin feat/your-feature`)
6. Create a focused Pull Request with the problem, validation, and screenshots for UI changes

## Code of Conduct

Please read [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).
