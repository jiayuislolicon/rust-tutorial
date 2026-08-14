# 0023 — Lesson 28: generic functions and trait bounds

Date: 2026-08-14. Follows directly from record 0022, which deferred this topic from Lesson 27.

## Why this topic

Record 0022 named the plan explicitly: traits, paired with generics and trait bounds, once the stale-field bug was out of the way. No new bug drove this one — it's scheduled material, justified by the mission's reading-comprehension goal rather than a fresh incident.

## Framing choice

Paragraph one is a signature the user cannot yet parse (`fn largest<T: PartialOrd>(list: &[T]) -> &T`), not a bug he hit. This is a deliberate exception to the record-0016 rule (paragraph one = a situation hit himself): the rule's purpose is avoiding a designer's-eye-view motivation, and "you'll meet this in real docs and can't read it" is itself the mission, not an abstraction of it.

Section 3 anchors the new term to code he already wrote and understands (`impl fmt::Display for AppError` in `errors.rs`), so "trait" isn't a cold abstraction — he already produced the pattern once by copying a shape.

## Evidence, all reproduced

- `largest<T>` (no bound) compiled and failed with `E0369`, verified via `rustc` in scratch.
- `largest<T: PartialOrd>` compiled and ran against both `Vec<i32>` and `Vec<&str>`, output `100` and `rust`, both verified.

## Content decisions

- TRPL's own `largest` example (ch10.1/10.2) is used verbatim rather than inventing a hello_cli-specific one — it's the primary source's canonical case, and reusing it means the citation and the taught code are the same example.
- No forced refactor of `hello_cli` this time. `word_counts` doesn't need a generic helper; inventing one would violate simplicity-first. The BTreeMap `K: Ord` connection (section 6) is pointed out, not applied as an exercise.
- New reference card `reference/traits-and-generics.html` (topic didn't fit existing cards). `GLOSSARY.md` gained one new entry (`Trait bound`) and one extension (`泛型參數` now cross-links to it).
- 7 sections, holding the record-0020 cap.

## Open risks

- The lesson does not cover writing your own trait's default methods, associated types, or `dyn Trait` — scoped to reading a bound and knowing why it's there. Those are separate lessons if the mission needs them later.
- No exercise has the user apply this inside `hello_cli` itself, unlike Lesson 27's Fix A. If retention is weak next session, that's the first thing to check.
