# 0013 — Lesson 20 verified; Lesson 21 (iterators) assigned

User completed the Lesson 20 exercise (unit tests for `classify_line` and `count_kinds`) unaided. Verified with `cargo test` (4/4 pass), `cargo build` (clean), the deliberate-failure check the lesson asked for (broke the expected tuple, confirmed `FAILED` with a real diff, restored), and a re-run of all three Lesson 18 CLI cases to confirm no regression. Committed as `40f02b3`.

**Notable: the user routed around a missing trait rather than reaching for a derive.** Their first attempt was `assert_eq!(Report::classify_line(""), LineKind::Empty)`, which failed because `LineKind` has neither `PartialEq` nor `Debug`. I explained both errors and offered `#[derive(Debug, PartialEq)]`. The user instead used `assert!(matches!(...))` for the three variant tests — which needs neither trait — and kept `assert_eq!` only for the `(usize, usize, usize)` tuple, where both traits already exist. That's the choice the lesson's exercise steps had modeled, and it means `LineKind` still carries no derives. Worth remembering: if a future lesson needs `assert_eq!` on an enum, `Debug`/`PartialEq` will have to be introduced then, and it will land as new material rather than a recap.

Lesson 21 assigned: iterators (`.filter()` / `.map()` / `.count()` / `.collect()`, closures, laziness). Chosen because it lets the user do their **first test-backed refactor** — rewrite `count_kinds`'s `for` loop plus three `mut` counters as three `lines().filter(...).count()` chains, and let `cargo test` prove behavior is unchanged. That pays off Lesson 20 immediately instead of leaving tests as a one-off exercise. The TS lever is unusually direct here (`Array.filter`/`map`/`length`), so the lesson leads with it; the one genuinely new idea is laziness, which has no JS counterpart and gets its own section.

Verified before shipping: the refactored `count_kinds` compiles and passes all 4 tests (tried in a scratch copy of the project, not the user's files), and both standalone snippets in the lesson run clean under `rustc`. `classify_line(line)` works without an explicit `*line` deref inside `filter` — the lesson's tip mentions the `&&str` possibility but doesn't make the user pre-emptively write the deref.

**修正（同一天，使用者提出質疑後）**：Lesson 21 第一版把「三次掃描不重要、可讀性更值得」寫成結論。使用者讀到後直接反駁，說這跟他學到的觀念衝突，並問這是不是 Rust 的特性。他的判斷是對的：三次掃描是事實（惰性與零成本抽象都不會把三條 chain 合成一次），但我漏了更重要的一項成本 —— `filter` 版本失去 `match` 的窮盡性檢查。實測確認：在 `LineKind` 加一個 `Doc` 變體，迴圈版本 `error[E0004]: non-exhaustive patterns` 編譯失敗，`filter` 版本只有一個 `never constructed` 警告、照常編譯，計數靜默漏掉新變體。

使用者選擇**保留 `filter` 練習**，只要求修掉那段話 —— 理由是課文的角色是告訴他「可以這麼做」，不是「這樣更好」。課文已改：兩個代價明列成清單，明說這個練習的目的是在熟悉的程式碼上練 `filter`/`count`、不是升級它，並加了一個提示框把「重構不該把編譯期錯誤換成執行期錯誤」提升為一般原則。比較框從 `.compare-before`/`.compare-after`（🔴/🟢）改成中性的 `.compare`，因為紅綠配色本身就在下判斷。

這是使用者第二次靠自己的觀念抓出課文問題（見 0002），而且這次抓的是判斷失當而非事實錯誤 —— 值得記住：他會讀課文的立場，不只讀語法。
