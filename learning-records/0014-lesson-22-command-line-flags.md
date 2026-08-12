# 0014 — Lesson 22 assigned: command-line flags

Lesson 21 (iterators) verified and committed as `cf89901`. User's `count_kinds` rewrite used plural names (`empty_counts`) — noted, no correction made beyond a passing remark.

Lesson 22 assigned: parsing a `--count-only` flag by hand. Chosen because it's the first thing that makes `hello_cli` behave like a real tool rather than an exercise — argument meaning stops depending on position — and it's directly on the mission's "parse command-line arguments" line. It also pays forward from three earlier lessons at once: `skip(1)` uses Lesson 21's iterators, the `Option<String>` filename slot uses Lesson 9, and `AppError::UnknownFlag(String)` reuses the Lesson 16/17/18 error pipeline without touching `main()`.

Three genuinely new things, each small: `.skip(n)`, `match arg.as_str()` (why the type mismatch happens), and match guards (`_ if cond`). Guards are the only real new *concept*; `match` itself is familiar from Lessons 6 and 14, so the concept load stays inside working memory.

**Design decision — no `clap`.** Hand-writing the parse keeps the lesson about the language (matching, guards, error variants) rather than about a crate's API, and the user has never added a dependency yet. The reference card names `clap` and says when to switch, so this isn't presented as the permanent answer.

Verified before shipping, in a scratchpad copy of the project (not the user's files):
- Full target `main.rs` + `errors.rs` build clean, `cargo test` 4/4.
- All seven CLI cases behave as the lesson's table promises, including flag-before-filename and flag-after-filename producing identical output, `--verbose` → `不認識的選項：--verbose` exit 1, and no regression on `nope.txt` / `empty.txt`.
- Both standalone snippets compile and run under `rustc`; the `--count-only --verbose notes.txt` run prints exactly the three lines the lesson claims.
- Confirmed the `as_str()` error message text by compiling the broken version: `expected `String`, found `&str``. Quoted by message, not by code number, per NOTES.md.

**Completed and verified the same day.** User's implementation matched the target shape exactly, unaided. `cargo test` 4/4 (re-run after `touch` to force a rebuild), `cargo build` clean, all seven CLI cases correct — including flag-before and flag-after producing byte-identical output, `--verbose` → exit 1, and no regression on `nope.txt`/`empty.txt`/no-args. Two personal choices, both fine and left alone: `starts_with("-")` with a `&str` instead of a `'-'` char, and 「未知旗標」 instead of 「不認識的選項」 as the Display text.

Mid-lesson the user asked two questions about `Some`, in sequence: first "how do you decide to write `Some(arg)` there", then — after an answer framed around the variable's declared type — "I meant, what does `Some` actually *do*". Signal worth remembering: **the first question was about the mechanism, not the decision procedure.** The useful answer was that `Some` is an enum variant name that doubles as a constructor, anchored to their own `AppError::UnknownFlag(String)` rather than to a metaphor. `Option` had been taught back in Lesson 9 in a *reading* posture (`match` to unwrap); this was their first time *constructing* one, and the constructing direction hadn't transferred. When a previously-taught type reappears in the opposite direction (build vs. destructure), don't assume it's a recap.

One thing to watch during verification: `return Err(AppError::UnknownFlag(arg))` moves `arg` while the `arg.as_str()` borrow is still the match scrutinee. This compiles (the borrow isn't used afterwards), and the lesson deliberately doesn't explain it — but if the user restructures the loop and hits a borrow error there, that's the cause.
