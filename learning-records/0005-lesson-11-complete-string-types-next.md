# Lesson 11 complete; string types taught next

Lesson 11 (borrowing) practice verified in `hello_cli`: `add_title(content: &mut String, filename: &String)` with `insert_str(0, ...)`. All three paths run clean — valid file, missing file, no argument, zero panics.

**Partially skipped:** step 3 of the exercise (deliberately triggering `cannot borrow ... as mutable because it is also borrowed as immutable`) was not performed. The user reused the name `peek` for the *mutable* binding (`let mut peek = content;`) rather than as the immutable borrow the step called for. The `E0502` recognition practice is therefore still owed — worth folding into a later lesson's warm-up rather than re-assigning it.

Lesson 12 (`0012-string-and-str.html`) written and verified, plus `reference/string-types.html`.

## Two things worth remembering

**The argument for `&str` parameters is the type asymmetry, not a lint.** I assumed clippy's `ptr_arg` would flag `filename: &String` and planned to cite it. It did not fire — verified on rustc/clippy 1.94.0 with a minimal repro where other clippy lints (`len_zero`, `useless_vec`) did fire. The lesson makes the case purely from the verified `expected &String, found &str` error instead. Do not cite tooling behaviour without running it, even when the lint is well known.

**UTF-8 byte indexing needs a prominent warning for this user.** Their test data and all their `println!` strings are Traditional Chinese, so `&s[0..1]` is a live hazard, not a footnote. Lesson 12 carries the real panic message (`byte index 1 is not a char boundary; it is inside '你' (bytes 0..3)`) and the `.len()` vs `.chars().count()` contrast. Expect to reinforce this whenever slicing or truncation comes up.

## How to apply

- Continue the pattern that is working: each lesson's exercise starts from the user's *own existing code*, and includes one deliberate-failure step. Lesson 12's step 1 is a no-op at the call site, which is itself the lesson (deref coercion).
- Next: `struct` and custom types. This is where `MISSION.md`'s "multi-file Cargo project with structs/enums/pattern matching" starts, and it's the natural payoff for having settled `String` vs `&str` — struct fields are the first place the user must choose deliberately.
