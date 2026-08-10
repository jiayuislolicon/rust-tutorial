# 0008 — Lesson 16 (error handling, Display trait) in progress

## What happened

Started Lesson 16: custom error enum (`AppError`) as `Result`'s `E`, plus implementing `std::fmt::Display` for it. This is the first lesson requiring `impl Trait for Type` syntax.

First explanation attempt (plain prose: "填契約", "遵守規格") did not land — user said they couldn't follow three paragraphs of it. Rewrote leading with a side-by-side TypeScript `interface`/`implements` example, then mapped each Rust line onto the TS equivalent, before any abstract description. This worked. Logged as a NOTES.md rule: for abstract "shape of a contract" concepts (traits, generics) with no concrete Rust mechanism yet on the page, lead with the TS analogy — don't append it after prose fails.

User then asked a good generalization check: does every std type get a custom `Display`? Answered in chat (not yet added to lesson): only types where printing as one line of text is meaningful implement `Display` (`i32`, `String`, `io::Error`); `Vec<T>` deliberately does not (only `Debug`, hence `{:?}`) because the standard library can't decide a sensible single-line rendering for arbitrary contents.

## Status

Update 2026-08-07: confirmed complete. `hello_cli/src/main.rs` has `AppError` (`ReadFailed`/`Empty`), `impl Display for AppError`, and `load_report` returning `Result<Report, AppError>`. Verified by running `cargo build` (clean) and `cargo run` against a real file, a missing file, and an all-whitespace file — all three produced the expected output. The exercise was done between sessions without a recorded learning record at the time.

## Next

Resume Lesson 16's hands-on exercise. Consider folding the "not every type implements Display, `Vec` only has Debug" point into the lesson prose itself next time it's touched — it's a natural follow-up question and currently lives only in chat.
