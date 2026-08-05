# User verifies lesson claims by experiment

While working through Lesson 8, the user noticed that `cargo run notes.txt` works without the `--` separator, contradicting the lesson's claim that `--` is required. The claim was imprecise (cargo forwards unrecognized trailing args automatically; `--` only matters when an argument starts with `-`), and Lesson 8 was corrected.

## Why this matters

The user tests statements against the actual toolchain rather than accepting them. Lessons must therefore be precise about *why* a rule exists, not just state it as a ritual — an unexplained "you must do X" invites a counterexample and costs trust when it turns out to be a habit rather than a requirement.

## How to apply

- State the mechanism, then the recommended habit. Distinguish "the compiler/tool requires this" from "this is a convention that avoids a class of mistake."
- Prefer runnable comparisons (A vs B vs the case where it breaks) over assertions. The user responds well to seeing the failing case, not just the working one.
- Do not soften into vagueness to stay safe — be precise enough to be falsifiable.
- Watch loose Chinese wording that collides with a technical term. Lesson 9 originally said `Option<String>` 「才可能是空的」; the user correctly challenged it, since a `String` can be `""`. Absence (`None`) and emptiness (`Some("")`) are distinct states — say 「沒有值」/「不存在」 for absence, reserve 「空」 for empty content. The user then trimmed the fix further: use the standard term 「空字串」 directly instead of paraphrasing it as 「有字串，內容是空的」. Prefer established terminology over descriptive circumlocution — the user reads Chinese technical prose fluently and doesn't need a term unpacked.

## Status

Also confirms Lesson 8 practice complete: `hello_cli/src/main.rs` reads `args[1]`, uses `fs::read_to_string` with a `match` on `Result`, and does not panic on a missing file. Both success criteria verified.
