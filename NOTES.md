# Notes

Operating rules for writing lessons. Scan before authoring. Each rule states the fix and the evidence that produced it.

**Everything is written in English, including lesson prose.** A separate model translates lessons into Traditional Chinese and fixes the phrasing. Rules about Chinese wording are that model's job, not mine — they have been removed from this file. Write plain English: no idioms, no puns, no wordplay that depends on English. Set `lang="en"` on new lessons; the translator flips it.

## Setup

- Code and standard terms stay in English in every version. Prior material was based on the Chinese translation of *The Rust Programming Language*.
- Toolchain rustc/cargo 1.94.0, installed (verified 2026-08-04). No setup lesson.
- Treat as a true beginner, per explicit request (「從頭全部重來」). `hello_rust`, `branches`, `functions`, `variables` are historical artifacts; assume zero retention.

## Size limits (from 2026-08-14)

- **Lesson: 6–7 sections.** Lesson 20 was 10 KB; drift reached 22 KB by Lesson 26. Violates the teach skill's own rule that lessons stay short because working memory is small. Overflow splits into two lessons or moves to a reference card. Count sections, not bytes — English source is smaller than the Chinese output the user reads.
- **Learning record: 2–3 KB.** Decisions, reasons, open risks. Verification still happens; it gets one line of conclusion, not a checklist.
- `GLOSSARY.md` is 28 KB — grep it, never read it whole. Batch verification into one script instead of round-tripping.

## Sentences

- **One idea per sentence.** Lesson 9 was reported as hard to follow because single sentences chained three ideas. Avoid long clause chains, dashes that bolt on an afterthought, dramatized "the compiler is saying…" quotes, and restating a standard term as a description.
- **Short sentences are not compressed explanations. Spend more paragraphs.** Lesson 17 (`?`/`From`) did not land because each idea got exactly one tight paragraph. The fix was more short paragraphs: a traced walk through the code with real filenames, an adapter analogy, an explicit "which of these two errors is this" comparison. A new *mechanism* (not just new vocabulary) needs elaboration plus one concrete worked example.
- **Never use an undefined term.** Lesson 10 was reported as incomprehensible; the cause was vocabulary, not sentence length — scope, trait, memory deallocation, double free all used as if known. Check every non-obvious term against `GLOSSARY.md` first. If absent, define it inline with a runnable example or drop it. These failures look like style complaints and are usually missing definitions.

## Framing

- **Paragraph one describes a concrete situation the user cannot handle without the thing being taught — and one they hit themselves.** Lesson 23 failed this: the motivation given was "`count_kinds` returning a tuple only works when the keys are known", a problem visible only from the syllabus designer's seat. Check: if paragraph one says what the thing *is*, the order is wrong. (Record 0016)
- **Lead with the TypeScript analogy, don't append it.** Lesson 16's `impl Trait for Type` explained as filling in a contract didn't land; a side-by-side TS `interface`/`implements` landed immediately. For anything with no concrete mechanism yet on the page (traits, generics, contract-shaped things), reach for TS first.
- **Don't replace an already-established mechanism with a metaphor.** "Moving to `String` costs you" was rejected because the previous line had already said *allocate memory + copy the contents* — the metaphor traded something concrete for something vaguer. Metaphors earn their place only before the mechanism exists.
- **Never use a framework — especially Tauri — as scaffolding.** He asked directly: 「我更真心的是想學習這個語言」. A framework frame demotes concepts to usage: `Arc<Mutex<T>>` becomes "how Tauri stores state" instead of shared ownership plus interior mutability, and he'd still be unable to read a non-Tauri file. Tauri is background context only, one sentence at most. No conflict with the TS rule: TS is a *mental model* he already has; Tauri is a *framework* that would replace the language. (Record 0016)

## Lesson content

- **Every code block must run as written.** He typed `s.len()` from a Lesson 12 callout and got `unused_must_use`. I had written bare expressions with `// 12` comments meaning "this evaluates to 12", but the block reads as a program. Wrap value-producing expressions in `println!`, or say above the block that it isn't runnable. He types blocks in verbatim; every one is a promise.
- **Name compiler errors by message text, not error code.** People read the English sentence, not `E0502`. Mention the code only where it's used for lookup (`rustc --explain`, search, reference cards).
- **Never write a contested judgment as a conclusion.** A Lesson 21 draft said readability beat performance at this tool's scale. His position: the lesson's role is "you may do this", not "this is better". Three consequences: (1) list the costs of a performance/readability trade, don't decide for him; (2) 🔴/🟢 before-after boxes only when the after is genuinely better — use neutral `.compare` to show an alternative; (3) **any refactor that trades a compile-time error for a runtime one must say so** (tested: add a `LineKind` variant and the `match` version fails to compile while three `filter` calls pass silently). (Record 0018)

## Quizzes

- **Stem and options must ask the same question.** A Lesson 12 stem asked what `s.len()` returns; every option answered *why*. He read the reason as the thing being tested. Reasons go in feedback; options stay bare answers.
- **Never quiz what the tool already tells you**, especially error codes. Lesson 11's `E0502` question was pure recall with no payoff — the compiler prints it every time. Replaced with "here is the failing snippet, which fix is right?", which tests the non-obvious rule (a borrow ends at its last use, so reordering fixes it). Quiz only what he'd have to *decide*.
- **Never quiz a fact that appears only in a code comment or buried in a dense multi-fact paragraph.** Lesson 14 asked which `Option` variant carries data; it appeared only in a `//` comment and one clause of a paragraph also introducing `Result`/`Ok`/`Err`/`None`. Before quizzing a fact, confirm it has its own bolded standalone sentence.
- Keep option lengths equal in words. No formatting tells.

## Layout (from 2026-08-10, the Lesson 17 rewrite)

- Numbered `<h2>` (`1.`, `2.` …), not unnumbered thematic headings.
- Teaching a refactor: two before/after boxes, each with a one-line reason and the code, using `.compare.compare-before` / `.compare.compare-after` from `assets/style.css`.
- Anything enumerable (cases, steps, rules) goes in bullets with a bolded lead-in (`<strong>If it is Ok(v)</strong>: …`), not dense paragraphs.
- Recap a classification with a table once it's been explained in prose. `<table>` has default styling.
- Emoji are fine (🔴🟢💡).
- **Any reusable visual pattern goes into `assets/style.css` as a class. Never inline `style=` in a lesson.**
- Lesson 18 predates this and was never retrofitted. Ask before rewriting it.
