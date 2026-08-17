# 0024 — Lesson 29: #[derive] and attribute macros

Date: 2026-08-14. Follows record 0023, which paired generics/trait bounds (Lesson 28) with `#[derive]` and attribute macros for a later lesson — the original plan in record 0016 had them together, split apart when Lesson 27 (stale derived field) displaced traits from Lesson 27 to 28.

## Why this topic

No new bug this time; scheduled material per the record-0016 plan. Confirmed with the user directly before writing (they picked it over jumping straight to an implementation lesson).

## Framing choice

Paragraph one is a situation already sitting in the user's own `report.rs`: the three `classify_line` tests use `assert!(matches!(...))` instead of `assert_eq!`, which the user has known how to write since Lesson 20. Trying the obvious rewrite and hitting two compiler errors (`E0369`, `E0277`) is the hook — no invented scenario.

Section 2 leans on `#[test]` and `#[cfg(test)]`, both already in `GLOSSARY.md` and typed by the user since Lesson 20, to introduce "attribute" as the umbrella name before `#[derive]` is presented as one member of that category rather than a wholly new kind of syntax.

No TS lead-in this lesson — attribute syntax and code-generating macros have no clean TS analogue (decorators are structurally different: they wrap/observe, they don't get to write a whole trait impl by inspecting fields). Forcing one would violate the "don't replace a mechanism with a vaguer metaphor" rule, so it's skipped entirely rather than stretched.

## Evidence, all reproduced

1. `assert_eq!(Report::classify_line(""), LineKind::Empty)` against un-derived `LineKind` → both `error[E0369]` (no `PartialEq`) and `error[E0277]` (no `Debug`) in the same compile, verified via `cargo test`.
2. `#[derive(PartialEq)]` alone (no `Debug`) → `E0277` alone survives. Confirms `Debug` is load-bearing for `assert_eq!`, not just decorative.
3. `#[derive(Debug, PartialEq)]` → all three `classify_line` tests compile and pass with `assert_eq!`.
4. `format!("{:?}", LineKind::Comment)` → `"Comment"`, verified, used as the concrete answer to "what does derived Debug actually print."

## Content decisions

- **The fix is not applied to `hello_cli/src/report.rs` in this repo.** Per the record-0022 pattern (Fix A left for the user to type), the two-line derive and the `assert_eq!` rewrite are the exercise. `report.rs` is committed unchanged.
- **`Display` on `AppError` is the running counterexample for "not derivable."** Reused from Lesson 28's section 3 rather than inventing a new one — same file, same trait-impl shape, now explaining why it *can't* be generated.
- New reference card `reference/derive-and-attributes.html` (attribute-as-category didn't fit `traits-and-generics.html`, which is scoped to generics/bounds). `GLOSSARY.md` gains `Attribute (#[...])` and `#[derive(...)]`, and the existing `Debug format ({:?})` entry gets one added sentence noting custom types need the derive.
- 7 sections, holding the record-0020 cap.
- Written in English (`lang="en"`), per record 0021 — lesson body, reference card, and GLOSSARY additions all in English; translation is external.

## Open risks

- The user still has to type the derive line and the two rewritten assertions into `report.rs` themselves. If skipped, the repo's tests keep using `matches!` and the lesson's payoff is untested against the real file.
- `Eq`, `Default`, `Hash` are named in the recap table but never exercised — same "read, not yet applied" scope as Lesson 28's generics-are-read-only choice. Fine for now; flag if a later lesson needs a `HashMap` key that isn't already `Hash`.
