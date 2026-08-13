# 0017 — Lesson 23 verified; Lesson 24 (type map) assigned out of plan order

Date: 2026-08-13. Lesson 23 verified and committed as `4bf7879`.

## Verification of Lesson 23

`cargo test` 5/5. `--words words.txt` gives `apple 3` / `banana 2` / `cherry 1`; `notes.txt`, `--count-only`, and the unknown-flag path all unaffected. The user placed the `--words` block *before* `report.add_title()` — the ordering trap the lesson called out explicitly (record 0015) — correctly.

One compiler warning left in place and surfaced rather than silently fixed: `let countsMap = ...` in `report.rs:122` trips `non_snake_case`. This is a front-end habit crossing over, and it's the first time the user has hit a *style* lint rather than a hard error. Worth noting that Rust enforces naming conventions in the compiler, which has no TypeScript equivalent — ESLint is opt-in, this isn't.

Also observed: the user wrote `return counts;` rather than a trailing `counts` expression. Not wrong and not corrected. If it recurs it's worth one sentence, but correcting a working idiom mid-course costs more attention than it's worth.

## Plan changed: Lesson 24 is the type map, not closures

Record 0016's plan had 24 = closures. The user asked directly instead: 「我想要下一堂課了解像是 Vec 這種型別定義的東西，還有哪些型別？他們的概念是什麼，為什麼要這樣設計」.

Taken as-is rather than redirected, for two reasons. It is a near-verbatim restatement of the mission's new success criterion (read an unfamiliar file and say what each type annotation is doing *and why it has to be written that way*), so it's more on-mission than closures. And it arrived immediately after the `Vec<(&String, &usize)>` question, which suggests the annotation soup is the live source of friction. Closures move to 25; the two-concepts-then-implementation cadence is unchanged.

## Design of Lesson 24

The organising claim is that **one constraint — sizes must be known at compile time — explains the whole shape of the type system**. This gives a "why designed this way" answer that is causal rather than a list of features, which is what the user actually asked for.

Deliberate choices:

- **Leads with a signature the user can't read** (`fn render(items: &[Item], index: &BTreeMap<String, usize>) -> Result<String, Box<dyn Error>>`), per the NOTES rule that a lesson must open with a situation you genuinely can't handle without the thing being taught. The stuck point named is specifically *not being able to tell signal from noise* — which is the reading problem, not a writing problem.
- **`size_of` as the evidence.** Record 0002 says this user verifies claims by experiment, so the abstract "sizes must be known" is anchored to six numbers they can reproduce. `Vec<i32>` = `Vec<String>` = `Vec<[u8; 99]>` = 24 is the load-bearing one: it makes "the Vec value is not the data" concrete in a way prose doesn't.
- **The three questions (誰擁有 / 大小固不固定 / 能不能改) are the real deliverable**, not the type table. A table is lookup; the questions transfer to user-defined types the course will never cover.
- **The recursive-`List` compile error is quoted verbatim** rather than described, including the compiler's own `insert some indirection (e.g., a Box, Rc, or &)` hint. Per NOTES, errors are referred to by message text. This also plants `Box` for Lesson 30 without teaching it.
- **`HashMap` vs `BTreeMap` is the "why so many types" argument**, with three real consecutive runs pasted in. Framed explicitly as a trade-off with a stated cost (`BTreeMap` is slower), not as "BTreeMap is better" — the record-0016 / NOTES rule against writing contested judgments as conclusions.
- **Generics are read-only here.** Angle brackets get explained via the TS `Array<string>` analogy and then explicitly deferred to Lesson 27. Teaching the user to *write* generics in the same lesson would blow working memory.

## The second exercise, and its risk

Exercise 2 has the user swap `word_counts` from `HashMap` to `BTreeMap` and delete the `collect` + `sort` lines. Verified end to end in a scratchpad copy with a forced rebuild: `cargo test` 5/5 with **zero test changes**, `--words` output byte-identical, `notes.txt` unaffected.

The point is that the tests don't change — `.get()` and `.len()` are common to both — which demonstrates that swapping a container is a small, safe move. And that picking the right type deletes code rather than adding it.

The risk is that this reads as "Lesson 23 taught you the wrong way." A tip box addresses it directly: `collect` + `sort` is still required the moment you want to sort by *value* rather than by key, which `BTreeMap` can't do. Watch whether that lands — if the user comes back thinking the earlier lesson was wasted, the framing needs work, and the same risk applies to every future lesson that revises earlier code.

## Lesson 24 outcome

Both exercises done. Exercise 2 landed exactly as designed: all three `BTreeMap` edits correct, `collect` + `sort` removed, `cargo test` 5/5 with zero test changes.

Two things worth carrying forward.

**The bonus step in exercise 1 didn't parse for the user, and the cause was a copying gap.** The step said "change `size_of::<Vec<i32>>()` to `size_of::<Vec<String>>()`", but their `types.rs` only contained the section-6 programs — the whole `size_of` block from section 2 was never copied, so the instruction referred to a line that didn't exist. Lesson for future exercises: an exercise step that says "modify line X" must not assume the learner assembled the file exactly as the lesson listed it. Either restate the line in full inside the step, or make the step self-contained. The prediction itself worked well once restored — `Vec<i32>` = `Vec<String>` = `Vec<[u8; 99]>` = 24 is the thing that makes "the Vec value is not the data" actually land.

**The user wired a scratch experiment into production code and every test still passed.** They kept section 4's slice demo as `fn total` + `fn test`, and called `test()` from the top of `run()`, so every invocation printed `6` `6` before its real output. `cargo test` stayed 5/5 green because the unit tests exercise `Report` methods and never call `run()`. Surfaced rather than silently fixed, and the user removed it themselves.

This is the first time a *green test suite has coexisted with a visibly broken tool* in this course, and it's a better demonstration of "tests only cover the paths they actually execute" than any lesson could have staged deliberately. Worth calling back to when integration testing or `assert_cmd` comes up — the user has a real memory of it now. Also flagged that naming a plain function `test` is asking for confusion with `#[test]`, since only the attribute makes something a test.

The user chose to keep `hello_cli/src/types.rs` in the repo as a record of the experiments. Cargo ignores it (not `main.rs`, not declared with `mod`), verified with `cargo build`. Root `.gitignore` added for `.DS_Store`; the user removed the `hello_cli/types` binary entry themselves after deleting the binary.

## Files

- `lessons/0024-the-type-map.html`
- `reference/std-types.html`
- `GLOSSARY.md`: slice, 大小在編譯期必須已知, `BTreeMap`, `HashSet`, `VecDeque`, 泛型參數
- `lessons/0023-hashmap.html`: next-lesson link filled in
