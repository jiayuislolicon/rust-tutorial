# 0021 — English is now the source language; translation is external

Date: 2026-08-14. Follows record 0020.

## The change

The user said he will pass my output to a second model for translation and phrasing:「你負責產出英文，我會再請另一個 model 幫我翻譯跟調整語句，如果有任何跟語言或是中文流暢度有相關的內容，都可以刪掉」.

So **everything I write is English now, lesson prose included** — not just `NOTES.md` and `learning-records/` (record 0020). Code and standard terms were already English and stay that way.

## Consequences

- **Rules about Chinese wording are deleted from `NOTES.md`.** Two went: the 「借來改」 rule (Chinese 借 implies taking away, which is `move`) and the "keep quiz options equal in characters" clause. Both are now the translator's problem, and neither is actionable from an English source.
- **The rules underneath them survived.** "One idea per sentence" and "don't replace a mechanism with a metaphor" came from Chinese-fluency complaints but are language-independent; they stay, with the Chinese quotes rewritten as plain descriptions.
- **Two Chinese quotes are kept deliberately**: 「從頭全部重來」 (beginner framing) and 「我更真心的是想學習這個語言」 (the Tauri prohibition). These are load-bearing user positions, not phrasing feedback. Record 0020's rule holds — the exact wording is the evidence.
- **The lesson size limit is now counted in sections, not bytes.** 12–14 KB was calibrated on Chinese HTML. English source of the same lesson is smaller, so the byte number would silently permit a longer lesson. 6–7 sections is the rule that still means what it meant.
- **New lessons get `lang="en"`**; the translator flips it to `zh-Hant`.
- Write plain English — no idioms, no puns, nothing that depends on English wordplay, since it has to survive a translation pass.

## Not changed

Existing lessons 0001–0026 stay in Chinese. Retrofitting them would cost exactly the tokens record 0020 set out to save, and the user has not asked.

## Open risks

- **Untested pipeline.** No lesson has gone through the translate step yet. The layout conventions (numbered `<h2>`, `.compare` boxes, bolded lead-ins) assume the translator preserves HTML structure. Lesson 27 is the first test — worth asking how it came out before writing 28.
- Quiz options equal in English words may end up unequal in Chinese characters after translation. Nothing I can do from here; flag it to the user if he reports formatting tells.
- The glossary is Chinese. New terms defined in an English lesson need a Chinese glossary entry, which now depends on the translation landing first. Order of operations is unresolved.
