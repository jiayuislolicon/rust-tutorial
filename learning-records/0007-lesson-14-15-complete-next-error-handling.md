# 0007 — Lesson 14 (enum) & Lesson 15 (review) complete, moving to error handling

## What happened

Verified Lesson 14's exercise directly from `hello_cli/src/main.rs`: `enum LineKind { Empty, Comment, Code }` (data-less version, as the exercise asked for), `Report::classify_line` associated function, and `Report::count_kinds(&self) -> (usize, usize, usize)` method — all present and correct. `main` calls `report.count_kinds()` and destructures the tuple. This confirms the enum + tuple exercise was done correctly.

During Lesson 14, three real comprehension gaps surfaced and were fixed in the lesson content (not just chat):
- Quiz 3 tested a fact (`Some` carries data) that only appeared in a dense intro paragraph and a code comment — added as its own bolded sentence. Logged as a general NOTES.md rule.
- `'static` appeared in a code example with zero prior explanation — added a deferred/placeholder GLOSSARY entry (same style as `Deref coercion`).
- `tuple` was required for the exercise but never introduced — added a GLOSSARY entry plus an in-lesson explanation with a runnable example.

User then asked for a dedicated review lesson (Lesson 15) before moving to new content, citing several concepts blurring together: mutability vs lifetime, struct vs enum, method vs associated function, tuple vs struct. Built as a no-new-content, quiz-only lesson. User confirmed readiness to move on without reporting further confusion.

## Why this matters

The user explicitly asked for a review/consolidation lesson mid-course, unprompted by any assistant suggestion — this is the first time they've self-diagnosed concept interference rather than the assistant catching it via quiz feedback. Worth watching for: if this recurs, consider building review checkpoints proactively every ~4-5 lessons rather than waiting for the user to ask.

## Next

Lesson 16: error handling and custom error types (`enum` as `Result`'s `E`, `impl Display for T`). This is the first lesson introducing trait *implementation* syntax (`impl Trait for Type`) — GLOSSARY's `Trait` entry previously deferred this ("a later lesson"); updated it now that the lesson exists. Ties directly to MISSION.md's "handle errors without panicking" criterion.
