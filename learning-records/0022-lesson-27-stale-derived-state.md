# 0022 — Lesson 27: the stale derived field

Date: 2026-08-14. First lesson written under the rules from records 0020 and 0021.

## Why this topic

Found it while verifying Lesson 26 before committing: `--count-only notes.txt` prints 共 3 行 / 程式碼 4 行. `Report::new` caches `line_count`, `add_title` mutates `content` afterwards, and nothing reconciles them. Pre-existing, unrelated to Lesson 26, in the user's own code.

That last part is why it displaced traits. The NOTES rule says paragraph one must describe a situation the user hit himself — this is the strongest opener available in the workspace, and it cost nothing to find. Traits move to Lesson 28, still paired with generics and trait bounds.

It also serves the mission directly. Reading comprehension means knowing what a `struct` is promising you; a derived field is a promise nothing enforces.

## Evidence, all reproduced

1. The mismatch itself, from the real binary.
2. An invariant test failing: `left: 3, right: 4`.
3. Deleting the field → `E0560` at the constructor, `E0609` at the reader. The compiler finds every dependent, it just can't judge the value.
4. Fix A (method) → 6/6 tests, output 4 and 4.
5. **Fix B plus a new mutating method (`add_footer`) → 0 errors, 0 warnings, 6/6 tests green, output 4 vs 5.** The bug returns in full. This is the load-bearing experiment; Fix A under the same probe gives 5 and 5.

## Design choices

- **Both fixes are taught, and the choice is left open**, per the Lesson 21 rule about contested judgments. The costs go in a table. The one thing stated flatly is factual, not preference: Fix B's failure mode is silent and was reproduced.
- **The primary source teaches Fix B**, not Fix A — TRPL 18.1 `AveragedCollection`. Citing a source that disagrees with the lesson's easier answer is the honest version, and it supplies the missing condition: Fix B is safe when fields are private.
- **The test asserts a relationship, not a number.** `assert_eq!(line_count(), empty + comment + code)`. Called out explicitly because a test pinned to `3` would have passed throughout.
- 7 sections, the new cap. Fix A and Fix B are separate sections rather than one comparison, because the user types both.

## Convention notes

- First lesson authored in English with `lang="en"`, per record 0021. Untested through the translation step.
- New material added to `GLOSSARY.md` and `reference/structs-and-custom-types.html` is English inside otherwise-Chinese documents. Glossary bodies were already English, so only the reference card is genuinely mixed. Record 0021 left this ordering unresolved; it stays unresolved.
- Lesson is 17.5 KB of English source. Sections, not bytes, is the cap now — but if the translated output reads long, the split point is between sections 5 and 6.

## Open risks

- The user has to apply Fix A to `hello_cli` himself; `report.rs` is unchanged in the repo. If he stops at Fix B, the tool keeps a latent bug.
- `add_footer` exists only as a probe in the lesson. If he leaves it in the real code, it prints a footer nobody asked for.
