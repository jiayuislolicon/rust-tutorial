# 0019 — Lesson 26: `--min N`, the first implementation lesson of the new cadence

Date: 2026-08-14. Written after Lesson 25 was committed (`1661034`).

## Why this lesson, and a note on plan order

Record 0016 agreed a cadence with the user: **two concept lessons, then one implementation lesson** that installs them into `hello_cli`. Lessons 24 (type map) and 25 (closures) were the two concept lessons, so 26 is implementation.

Note the plan table in record 0016 listed "24·25 = closures; traits" — traits got displaced when the user asked for the type map instead (record 0017). The *cadence* is being honoured, not the content list. **Traits therefore have no lesson number yet** and should be picked up in the 27·28 concept slot alongside generics & trait bounds, which is a natural pairing anyway (`impl<T: Trait>` needs both).

## The stuck point

Per the NOTES rule, the opener is a situation the user hits themselves: `let min = 2;` is hardcoded in their own `main.rs`, so changing the threshold means editing source and recompiling. Record 0018 predicted this exact question ("if the user asks why it's hardcoded, that's a good sign and a natural next exercise") — the lesson cashes that prediction.

## Three pieces of compiler evidence, all reproduced

Record 0002 says this user verifies claims by experiment, so the lesson is built around errors the toolchain produces rather than assertions from me. All three were run against a scratch copy of `hello_cli` on edition 2024:

1. **`for` + `next()` → `error[E0382]: borrow of moved value: args`.** This is the load-bearing one. It carries three separate teaching points in the compiler's own words: `a for loop advances the iterator for you`, `args moved due to this implicit call to .into_iter()`, and a `consider using while let` help block with the rewritten line. It also lands ownership (Lesson 10) on a type the user did not think of as a value.
2. **Not consuming the value → an order-dependent bug, not an error.** With `"--min" => min = 2,` (no `args.next()`), `--words --min 2 words.txt` works, but `--words words.txt --min 2` gives `讀不到檔案：No such file or directory (os error 2)` because `2` overwrites `filename`. Both runs verified. This is the strongest argument in the lesson: the failure is silent and positional, so "eat the value" is correctness, not style.
3. **`value.parse()?` without the impl → `error[E0277]: ? couldn't convert the error to AppError`.** Straight replay of Lesson 17. The message ends with `but trait From<std::io::Error> is implemented for it`, which points the user at the impl they already wrote — a better prompt than anything I'd have written.

## Design choices

- **Two error variants, not one.** `MissingValue(String)` and `BadNumber(ParseIntError)` separate "no value at all" from "value isn't a number". The point made in prose is that the first is forced on you by `Option`'s exhaustiveness, so you can't forget it.
- **Default `min = 1`, not 2.** 1 means no filtering, which is the right default behaviour when the flag is absent. Called out in a tip box because it's a silent behaviour change to their existing `--words` output.
- **The `: usize` annotation is described as optional, and that was tested.** Removing it still compiles; probing with `let _probe: i32 = min;` gives `expected usize, found i32`, confirming the comparison `*pair.1 >= min` pins the type. The lesson says the annotation is for the reader, not the compiler. Worth being careful here — the first instinct was to claim it was required.
- **Section 8 closes the loop back to Lesson 25 explicitly.** `retain(|pair| *pair.1 >= min)` did not change one character, but `min` is now a runtime value. That turns Lesson 25's `can't capture dynamic environment in a fn item` from a compiler curiosity into the reason the tool can accept user input at all. This is the cheapest available demonstration that concept lessons pay off later — the same concern record 0017 raised about revising lessons.
- **Exercise 2 is a non-exhaustive-patterns probe, not a new feature.** Add a variant to `AppError`, don't touch `Display`, read `error[E0004]: non-exhaustive patterns: &AppError::TooManyFiles not covered`, then delete it. Verified. Chosen over a feature because the implementation in section 7 is already a lot of typing, and this is the third time exhaustiveness has been asserted (Lessons 14, 21) without the user watching it fire on their own enum.

## Verified before shipping

Full implementation in a scratchpad copy of `hello_cli`:

- `--words words.txt` → `apple: 3` / `banana: 2` / `cherry: 1` (default `min = 1`, nothing filtered)
- `--words --min 2 words.txt` → `apple: 3` / `banana: 2`
- `--min 3 --words words.txt` → `apple: 3` (flag before `--words`, proving the value is consumed)
- `--words --min abc words.txt` → `數字格式錯誤：invalid digit found in string`, exit 1
- `--words words.txt --min` → `--min 後面少了一個數字`, exit 1
- `notes.txt`, `--count-only notes.txt`, `--nope notes.txt` all unchanged
- `cargo test` 5/5 with zero test changes
- Both book URLs (ch12-01, ch19-01) and the Chinese mirror return 200

## Things to watch

- **This lesson has more typing than any recent one** — two enum variants, two `Display` arms, a `From` impl, and a restructured loop. If the user reports it as heavy, the fix is to split the error-variant work into its own step rather than to cut the explanation.
- **`min` is now read even when `--words` is absent.** No effect today (nothing else uses it), but `cargo build` stays clean because it *is* used inside the `--words` branch. If the user reorganises `run()` later this could become a dead-code warning.
- **The user tends to write `return counts;` over trailing expressions** (record 0017). Still not corrected. The new code has `return Err(...)` inside a match arm, where `return` is genuinely required — worth mentioning only if they ask why this one is different.

## Files

- `lessons/0026-flags-with-values.html`
- `reference/command-line-flags.html`: new 帶值的旗標 section + fragment table + `From<ParseIntError>`; lesson link added
- `GLOSSARY.md`: `while let`, `parse`, 帶值的旗標
- `lessons/0025-closures.html`: next-lesson link filled in
