# 0010 — Lesson 17 verified complete, Lesson 18 (`run()`/`main()` split) assigned

## What happened

Session opened with `/teach`, asked to verify Lesson 17 and start Lesson 18. Read `hello_cli/src/main.rs`: the user had already added `impl From<std::io::Error> for AppError` and rewritten `load_report` to use `?`, matching the Lesson 17 exercise exactly. Ran `cargo build` (clean) and all three manual test cases (real file, missing file, empty file) — all matched pre-rewrite behavior. Lesson 17 confirmed complete.

Asked the user to pick Lesson 18's direction between two options (`main()` itself returning `Result` + `#[derive(Debug)]`, vs. multi-file processing). User said "you decide."

Chose neither option offered and picked a third instead, after reconsidering: `fn main() -> Result<(), AppError>` would print errors via `{:?}` (Debug) on failure — a real regression from the current clean `Display` message, and Debug derivation adds a concept with no payoff. The Rust Book's own I/O project chapter (ch 12.3/12.6) recommends the better pattern: split `run()` (returns `Result`, keeps using `?` and `Display`) from a thin `main()` that does `if let Err(error) = run() { eprintln!("{}", error); std::process::exit(1); }`. This teaches three genuinely useful, tightly-scoped things for a CLI tool — `if let` shorthand, `stdout`/`stderr` separation via `eprintln!`, and exit codes via `process::exit` — without sacrificing the nice error message, and ties directly to the mission ("ship a small real CLI tool" needs to behave correctly when piped/scripted).

Wrote Lesson 18 (`./lessons/0018-run-and-main-split.html`), its reference card, and five new glossary entries (`if let`, Standard streams, `eprintln!`, Exit code, `std::process::exit`). Linked Lesson 17 → Lesson 18 nav.

## Status

Lesson 18 is written; user has not yet done the hands-on exercise (extract `run()`, rewrite `main()` to the three-line form). `hello_cli/src/main.rs` still has the Lesson 17 version (single `fn main()`, no `run()`).

## Next

Resume Lesson 18's hands-on exercise. Success check: `cargo build` clean; same printed output for real/missing/empty file as before; `echo $?` after a failing run is now `1` (was `0` before this lesson, since `main()` never called `process::exit`).
