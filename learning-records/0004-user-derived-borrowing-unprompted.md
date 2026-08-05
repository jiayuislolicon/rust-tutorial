# User derived borrowing before it was taught

Lesson 10's exercise asked for a `.clone()` fix to `E0382`. The user did not use `clone`. They changed the signature to `fn summarize(content: &String)` and the call to `summarize(&content)` — reaching Lesson 11's answer from the compiler's own `consider changing this parameter type ... to borrow instead` note.

They also independently discovered that a move with no subsequent use produces **no error at all** (their first attempt compiled cleanly because they had omitted the second `println!`).

## Why this matters

Two signals:

1. **The user reads compiler notes, not just error headlines.** The `note:` line is where they found `&`. Lessons should keep quoting full compiler output verbatim, including the notes — that is where this user actually learns.
2. **Zone of proximal development is wider than assumed for mechanism-level content.** They can generalise from an error message to a fix without being shown the syntax. What they cannot do is absorb undefined vocabulary (see `NOTES.md`). The bottleneck is terminology, not conceptual difficulty.

## How to apply

- When a lesson deliberately ends unresolved with a compiler hint, expect the user to solve it early. Lesson 11 was rewritten to open by acknowledging their own code as the answer rather than presenting `&` as new. Do this again rather than pretending the hint wasn't taken.
- Keep raising mechanism depth; keep vocabulary discipline strict. These are separate dials.
- "Moving is fine until you use the original again" turned out to be the load-bearing sentence of Lesson 10, and it surfaced only because the user's incomplete attempt compiled. Consider designing more exercises where the *first* attempt succeeds for an instructive reason.

## Status

- Lesson 10 complete. Lesson 11 (`0011-borrowing-and-references.html`) written, exercise solution and deliberate `E0502` both verified against rustc 1.94.0.
- `reference/ownership-and-borrowing.html` shipped, covering both halves as planned in [[0003-lesson-9-complete-ownership-split]].
- Next: `String` vs `&str`. The user is currently writing `&String` parameters, which is what Lesson 11 taught, but idiomatic Rust is `&str`. Lesson 12 should close that gap — the motivation is already in their own code.
