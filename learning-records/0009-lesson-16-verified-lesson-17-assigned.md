# 0009 — Lesson 16 verified complete, Lesson 17 (`?` operator / `From`) assigned

## What happened

Session opened with `/teach` and no specific topic named. Found `hello_cli/src/main.rs` already had the full Lesson 16 exercise done (`AppError` enum, `Display` impl, `load_report`), contradicting learning record 0008 which said it was still pending. Verified with `cargo build` plus three manual runs (real file, missing file, empty file) — all correct. Updated 0008's status rather than leaving it stale.

Asked the user whether to do a quick trait/Display review before moving on, or go straight to the next lesson. User chose to go straight ahead.

Picked the next lesson myself, since none was requested: Lesson 16's `load_report` still hand-writes `Err(e) => Err(AppError::ReadFailed(e))` to forward a `io::Error` as an `AppError`. That's exactly the boilerplate the `?` operator removes, and it requires `impl From<io::Error> for AppError` for the type conversion — a direct, in-ZPD extension of the trait-impl pattern from Lesson 16, not a new topic. Wrote Lesson 17 on `?` + `From`, added its reference card, and added glossary entries for `` `?` operator `` and `From trait` (neither existed yet, per the "never use an undefined term" rule).

## Status

Lesson 17 (`./lessons/0017-question-mark-operator.html`) is written and linked from Lesson 16's nav. User has not yet done the hands-on exercise (add `impl From<std::io::Error> for AppError`, rewrite `load_report` to use `?`). `hello_cli/src/main.rs` still has the Lesson 16 `match`-based version.

## Next

Resume Lesson 17's hands-on exercise. Behavior should be identical to Lesson 16 post-rewrite (same three test cases: real file, missing file, empty file) — this lesson only changes the code's shape, not its output.
