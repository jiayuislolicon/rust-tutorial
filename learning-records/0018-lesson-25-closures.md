# 0018 — Lesson 25: closures

Date: 2026-08-13. Written immediately after Lesson 24 was committed (`ff1a823`). Back on the record-0016 plan, which had 25 = closures.

## Why closures now

Two things make this the right next lesson rather than a plan obligation:

- **The user has been writing closures since Lesson 20 without being told what they are.** `report.rs` is full of `.filter(|line| matches!(...))`. Under the revised mission (reading comprehension), an unexplained construct sitting in their own code is exactly the debt worth paying off.
- **Lesson 24 left a promise.** Its exercise-2 tip said `collect` + `sort` is still needed the moment you sort by *value* rather than key, which `BTreeMap` can't do. Lesson 25's exercise 2 is that exact case. This matters beyond convenience: record 0017 flagged the risk that a revising lesson reads as "the earlier one was wrong". Cashing the promise one lesson later is the cheapest available demonstration that it wasn't.

## The stuck point

Per the NOTES rule (課 23 post-mortem), the opener has to be a situation the user hits *themselves*, not one visible only from the syllabus. Chosen: they want `--words` sorted by count descending, keeping only words appearing ≥ 2 times. `BTreeMap` sorts by key, `sort()` has one order, and the filter threshold `min` is a local variable.

Then the load-bearing evidence — a plain `fn` reaching for `min` fails:

```
error[E0434]: can't capture dynamic environment in a fn item
  = help: use the `|| { ... }` closure form instead
```

This error is unusually good for teaching: the compiler names the concept (*capture dynamic environment*) and prescribes the exact syntax. Per NOTES, quoted verbatim by message text, code mentioned only in passing. Record 0002 says this user verifies claims by experiment, so exercise 1 is just "reproduce this error yourself, then fix it as instructed" — the concept arrives from the toolchain, not from me.

## Design choices

- **One claim carries the lesson: a closure is a function plus the variables it captured.** Everything else (syntax forms, `Fn`/`FnMut`/`FnOnce`, `move`, `impl Fn`) is downstream of it. Stated as its own bolded standalone sentence so quiz 1 is fair under the NOTES rule about not quizzing facts that only appear in comments.
- **TS arrow functions lead the syntax section, not follow it** (NOTES: the TS background is a lever, use it first for anything abstract). Three-line side-by-side showing the same collapse from full annotation to `|x| x + 1`. Used two neutral `.compare` boxes with no `-before`/`-after` modifiers — neither language is better here, same call as Lesson 24.
- **`Fn`/`FnMut`/`FnOnce` taught as a *reading* skill, explicitly.** The lesson says outright that you never declare which one; the compiler infers it. What's actionable is decoding `impl Fn(&str) -> bool` in a signature, so that gets its own section with a four-row breakdown of the fragments.
- **`move` is framed as read-only knowledge too**, with a callout saying so. The honest motivation (a closure outliving what it borrowed, i.e. threads) can't be demonstrated without threads, so it's stated and deferred rather than faked with a contrived example. `thread::spawn(move || ...)` named so the shape is recognisable.
- **`Box<dyn Fn>` gets two rows in a table and nothing more.** It connects back to Lesson 24's `Box` (breaking an unknown size), with `dyn` explicitly deferred to Lesson 30. The reason each closure has its own anonymous type is given, since without it the `Box` requirement looks arbitrary.
- **Quiz 3 tests `b.1.cmp(a.1)` direction** — the one thing here you actually have to *decide* rather than look up (NOTES: never quiz what the tool tells you). Stems and options are bare answers with reasons in feedback.

## Verified before shipping

Every code block and every error message was compiled and run, not recalled:

- `fn` capturing `min` → E0434 with the `|| { ... }` help line, quoted exactly.
- The closure fix printing `[2, 3]`.
- Three syntax forms all yielding `2`.
- `FnMut` accumulator printing `total = 3`.
- `move || owned` called twice → `use of moved value`, including the `closure cannot be invoked more than once because it moves the variable out of its environment` note.
- `count_if` with `impl Fn(&str) -> bool` returning 1 and 1 on the sample text.
- Every reference-card snippet, including `sort_by_key(|pair| *pair.1)` and `find`.
- **Exercise 2 end to end in a scratch copy of `hello_cli`**: `--words words.txt` → `apple 3` / `banana 2` (cherry dropped at count 1), `cargo test` 5/5 with zero test changes.

One claim was wrong on the first draft and got corrected by testing: I wrote that omitting the deref gives ``can't compare `&usize` with `{integer}` ``. The real message is `expected `&usize`, found integer` plus `help: consider dereferencing the borrow`. Lesson updated to the real text. Worth noting as a pattern — comparison-operator type errors are plain `E0308 mismatched types`, not a special "can't compare" diagnostic.

## Things to watch

- **Exercise 2 hardcodes `let min = 2;`.** That's deliberate: parsing a flag *with a value* (`--min 3`) is a real jump from their current all-boolean flag loop, and mixing it in would blow working memory. If the user asks why it's hardcoded, that's a good sign and a natural next exercise.
- **The `count_if` example is one step from refactoring their own `count_kinds`.** The lesson points at this ("也正好可以拿來簡化你 count_kinds 裡那三段幾乎一樣的程式碼") but deliberately doesn't assign it — three near-identical `filter` blocks collapsing into one closure-taking helper is a better standalone exercise later, and Lesson 21's record warns against presenting that refactor as unambiguously better.
- The `countsMap` → `counts_map` lint in `report.rs:122` is still open. The user renamed it to `counts_map` in the test at some point (verified: line 122 now reads `counts_map`), so this appears resolved.

## Files

- `lessons/0025-closures.html`
- `reference/closures.html`
- `GLOSSARY.md`: 閉包, 捕捉環境, `Fn`/`FnMut`/`FnOnce`, `move` 閉包, `sort_by`/`retain`/`sort_by_key`
- `lessons/0024-the-type-map.html`: next-lesson link filled in
