# terminal_graphics

A zero-dependency graphics API for drawing in a terminal, written in Rust.

## Non-negotiable rules

1. **100% Rust, zero dependencies.** Source, tests, examples and tooling are Rust.
   Do not add other languages (no shell/Python/JS helper scripts, no build
   wrappers) and do not add crates to `[dependencies]` or `[dev-dependencies]`.
   `tests/pure_rust.rs` enforces this; CI fails if it breaks. The only non-Rust
   files allowed are config/docs/assets listed in that test's allow-list.
2. **TDD, always.** No production code without a failing test first:
   - RED: write one test for one behaviour, run it, confirm it fails for the
     expected reason.
   - GREEN: write the minimum code to pass.
   - REFACTOR: clean up with the suite green.
   Bug fixes start with a test that reproduces the bug.
3. **Follow the test triangle** (many fast tests at the bottom, few at the top):
   - **Unit tests** (the bulk): `#[cfg(test)] mod tests` beside the code in
     `src/`. Pure logic, no I/O. Pixel/colour/shape behaviour, bounds, edge
     cases.
   - **Integration tests** (some): `tests/*.rs`, using only the public API
     (`Display`, `Container`, `Graphics`, ...) and asserting on rendered output.
   - **End-to-end tests** (few): `tests/e2e.rs` runs the real `main` binary and
     checks its stdout.
   Doc-tests on public items are a bonus layer, not a substitute.
4. **Keep it testable.** Rendering goes through `Display::render()` /
   `Display::write_to(&mut impl Write)`; only `print()` touches stdout. Never
   assert on stdout from unit tests.
5. **Library code must not panic on bad input** (out-of-range coordinates,
   zero-size displays, unknown names). Clip or no-op instead.

## Commands

```
cargo test --all-targets && cargo test --doc   # everything
cargo clippy --all-targets                      # must be warning-free
cargo fmt --all                                 # LF line endings, see .gitattributes
cargo llvm-cov --fail-under-lines 90            # coverage gate (CI)
```

## Layout

- `src/display/` - `Colour`, `Pixel`, `Display` (the terminal buffer + ANSI output)
- `src/graphics/` - `Graphic` trait, `Graphics` collection, `Container`, `Text`,
  `shapes/` (`Rect`, `Ball`)
- `src/bin/main.rs` - small demo binary
- `tests/` - integration, e2e and repo-hygiene tests

## Conventions

- Edition 2021, `#![forbid(unsafe_code)]`.
- Keep the public API backwards compatible unless the change is called out.
- Warnings are errors in CI (`RUSTFLAGS=-D warnings`).
