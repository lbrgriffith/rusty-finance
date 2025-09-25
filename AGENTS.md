# Repository Guidelines

## Project Structure & Module Organization
Rusty Finance is a Cargo workspace with a single binary crate. `src/main.rs` wires the CLI into `src/lib.rs`, while `src/cli.rs` defines command parsing and option routing. Calculation logic lives in `src/calculations/` with focused modules such as `loan.rs`, `interest.rs`, and `statistics.rs`. Presentation helpers are split across `src/display.rs` and `src/display_patched.rs`. Unit tests reside inline beside the code under `#[cfg(test)]`; there is no `tests/` harness.

## Build, Test, and Development Commands
- `cargo check` — type-check fast during active development.
- `cargo fmt` — apply the canonical Rust style; run before every commit.
- `cargo clippy --all-targets --all-features` — catch lint issues; respect lint warnings unless waived in review.
- `cargo test` — run unit tests, including module-level calculators.
- `cargo run -- --help` — verify CLI wiring and flag documentation. Use `cargo run -- <command>` to validate new flows.
- `cargo deny check` — audit dependencies using the policy in `deny.toml` (run before release branches).

## Coding Style & Naming Conventions
Follow Rust 2021 defaults with four-space indentation. Use `snake_case` for modules, functions, and variables; reserve `PascalCase` for types and structs, and all-caps for compile-time constants. Prefer expressive `Result<T, anyhow::Error>` returns with `?` for propagation. Keep modules focused—split large features into submodules inside `src/calculations/`. Document user-facing commands with `///` comments so `--help` output stays accurate.

## Testing Guidelines
Every new calculator or formatter addition needs targeted unit tests adjacent to the implementation, using `mod tests` blocks and descriptive function names like `calculates_monthly_payment`. Exercise both happy paths and validation errors. When adding CLI flags, cover parsing via integration-style tests under `src/cli.rs`. Run `cargo test -- --nocapture` if you need to inspect formatted output.

## Commit & Pull Request Guidelines
Write commits in the imperative mood (e.g., “Add amortization summary table”) and keep them scoped to a single vertical slice. Reference GitHub issues with `Fixes #123` when closing work. Pull requests must include: a concise summary, screenshots or sample output for display changes, and a checklist of commands run (`cargo fmt`, `cargo clippy`, `cargo test`). Mention any follow-up tasks to help reviewers plan subsequent iterations.

## Security & Configuration Tips
Consult `SECURITY.md` for disclosure steps. Before shipping, run `cargo deny check bans advisories` to confirm dependency health, and ensure sensitive values never enter logs—`env_logger` respects `RUST_LOG`, so sanitize anything emitted at debug level.
