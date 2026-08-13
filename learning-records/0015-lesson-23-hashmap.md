# 0015 — Lesson 23 assigned: HashMap

Lesson 22 verified and committed as `d9584c0`.

Lesson 23 assigned: `HashMap`, taught through a `--words` flag that counts word frequencies. Chosen because `count_kinds` returning `(usize, usize, usize)` only works when the set of keys is known ahead of time, and that limit is now visible to the user — word counting is the smallest problem that breaks it. Also gives the tool a capability it genuinely lacked.

Reuses the last two lessons directly: the flag goes into the Lesson 22 `match`, `split_whitespace()` is another string iterator source from Lesson 21, and `.get()` returning `Option<&V>` re-exercises `Option` — this time in the *reading* direction, days after the user constructed one for the first time. That spacing is deliberate.

The one hard line is `*counts.entry(k).or_insert(0) += 1;` — it's broken into three bullets (slot handle, `&mut V`, deref) and gets a quiz.

**Second teaching point: `HashMap` has no iteration order.** Given learning record 0002 (user verifies claims by experiment), the lesson shows real evidence rather than asserting it — three runs of the same program printing three different key orders, pasted verbatim. The exercise then has them `collect()` into a `Vec` and `sort()`, which is also what makes the verification table deterministic.

Verified before shipping, in a scratchpad copy:
- Full target (`word_counts` on `Report`, a 5th test, the `--words` branch in `run()`) builds clean; `cargo test` 5/5.
- `--words words.txt` prints exactly `apple 3` / `banana 2` / `cherry 1`; `notes.txt` and `--count-only` unaffected.
- Every standalone snippet compiled and run under `rustc`, with printed output matching the `//` comments (`Some(3)`, `None`, `2`).
- The unordered-iteration evidence is real output from three consecutive runs, not invented.

**Fixture note:** the exercise asks the user to create `words.txt`, because `notes.txt` is Chinese prose with no interword spaces — `split_whitespace()` on it yields whole sentences, every count is 1, and the feature looks broken. Worth remembering generally: this tool's word-level features need an ASCII fixture to demo at all, and 「詞頻」 on Chinese text would need segmentation, which is far out of scope.

**Ordering trap the lesson calls out explicitly:** the `--words` block must sit *before* `report.add_title()`, since `add_title` mutates `content` and its `=== filename ===` would be counted as words.
