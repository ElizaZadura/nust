# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs` contains the entire eframe/egui application, including pane state, command palette, and file operations.
- `docs/` stores design notes such as `todo.md`; keep planning or research artifacts here.
- `run_nust.sh` launches the app with the recommended WSL-safe environment variables.
- Quick-save artifacts land in `target/quick_saves/`; they stay out of Git via `.gitignore`.

## Build, Test, and Development Commands
- `cargo +nightly run` — Build and launch the GUI; nightly is required because the crate uses `edition2024`.
- `./run_nust.sh` — Convenience wrapper that exports `LIBGL_ALWAYS_SOFTWARE=1` and `MESA_GL_VERSION_OVERRIDE=3.3` for WSL.
- `cargo +nightly fmt` — Format Rust sources with rustfmt; run before committing.
- `cargo +nightly check` — Type-check without running the GUI (first run may take longer while caching deps).

## Coding Style & Naming Conventions
- Follow Rust 2024 idioms; rely on `rustfmt` for layout (4-space indentation).
- Keep UI actions and state localized in `App`; add helper structs/enums when functionality grows.
- Status strings should be short and actionable ("Split view enabled").
- Name quick-save files with the existing `nust_<pane>_<timestamp>.txt` scheme for consistency.

## Testing Guidelines
- No formal test suite yet. Manually verify additions via `cargo +nightly run`, focusing on pane interactions, command palette navigation (Ctrl+Shift+P, Arrow keys, Enter/Esc), and file save/load flows.
- When adding future tests, colocate Rust integration tests under `tests/` and follow `snake_case` filenames.

## Commit & Pull Request Guidelines
- Use descriptive, sentence-style commit messages (e.g., `Add command palette with action shortcuts`).
- Each commit should format code and keep unrelated changes out; mention if `cargo +nightly check` succeeds.
- PRs should summarize user-facing tweaks, list key shortcuts affected, and include screenshots only when UI layout changes substantially.

## WSL & Configuration Tips
- Ensure `rustup override set nightly` inside the repo; otherwise edition 2024 builds fail.
- When GUI rendering breaks under WSL, fall back to `LIBGL_ALWAYS_SOFTWARE=1 MESA_GL_VERSION_OVERRIDE=3.3 cargo run` as documented in `README.md`.
