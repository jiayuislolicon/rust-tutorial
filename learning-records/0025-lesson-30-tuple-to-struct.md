# 0025 — Lesson 30: tuple → struct with #[derive], the implementation lesson

Date: 2026-08-17. Follows record 0024, which delivered the #[derive] and attribute concept lesson (Lesson 29). Per record 0016's agreed cadence, two concept lessons (28, 29) are followed by one implementation lesson (30).

## Why this lesson

Record 0016 mandates: **two concept lessons, then one implementation lesson that installs those concepts into `hello_cli`.** Lessons 28 (generics & trait bounds) and 29 (#[derive] and attributes) were the concept pair. Lesson 30 is the scheduled implementation.

## The stuck point

`count_kinds()` in `report.rs` returns `(usize, usize, usize)`. In the existing test, `assert_eq!(counts, (1, 1, 1))` — all three values are identical, so misordering the tuple elements is undetectable. Even in `main.rs`, the destructuring `let (empty, comment, code) = report.count_kinds()` uses names chosen by the caller, not enforced by the type. This is a real latent bug in the user's codebase, not a contrived scenario.

## Evidence, all reproduced

1. Swapping two elements in the tuple return → `cargo test` still passes (because `(1,1,1) == (1,1,1)`). Confirmed.
2. After introducing `LineStats` with `#[derive(Debug, PartialEq)]`, `assert_eq!` works with struct literal comparisons. 6/6 tests pass.
3. `#[derive(Debug)]` on `AppError` compiles cleanly.
4. `#[derive(Debug, PartialEq)]` on `AppError` fails with `error[E0277]: can't compare std::io::Error` — confirmed: `io::Error` doesn't implement `PartialEq`.
5. Binary output unchanged: `空行 1 行，註解 0 行，程式碼 5 行`.

## Design choices

- **The refactoring is applied to the repo.** Unlike concept lessons (where fixes are left for the user), implementation lessons commit the working state so the codebase evolves. This follows Lesson 26's pattern.
- **`LineStats` fields are `pub`** for simplicity. Private fields + accessor methods are valid but would double the lesson's code changes for no teaching payoff at this stage.
- **`line_stats` replaces `count_kinds` entirely** rather than keeping both. One function, one name. The test is renamed correspondingly (`line_stats` and `summary_agrees_with_line_stats`).
- **`AppError` gets `#[derive(Debug)]` only** — PartialEq's failure is shown and explained in prose, then we leave it at Debug. This reinforces Lesson 29's "every field must also have the trait" rule without adding dead code.
- **No new generic function is written by the user.** The generic aspect is exercised through *reading*: `assert_eq!` is itself a generic macro requiring `T: PartialEq + Debug`, and the derives satisfy those bounds. Writing a new generic function felt forced for this codebase; the connection is made in the "win" section instead.
- 7 sections, holding the cap.

## Files changed

- `hello_cli/src/report.rs`: `LineStats` struct + `#[derive(Debug, PartialEq)]`, `count_kinds` → `line_stats`, tests refactored
- `hello_cli/src/errors.rs`: `#[derive(Debug)]` on `AppError`
- `hello_cli/src/main.rs`: `report.line_stats()` + field access
- `lessons/0030-tuple-to-struct-derive-in-practice.html`: new lesson
- `lessons/0029-derive-and-attribute-macros.html`: next-lesson link updated
- `learning-records/0025-lesson-30-tuple-to-struct.md`: this file

## Open risks

- The user might wonder why no generic function was added. The win section connects the dots verbally; if they ask for a hands-on generic exercise, the natural follow-up is extracting a trait or writing a generic summary function in a later lesson.
- `LineStats` fields are pub, so nothing stops external code from constructing one with wrong values. This is fine for the current single-binary crate.
