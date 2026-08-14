# 0020 — Lesson size limits, and NOTES.md rewritten in English

Date: 2026-08-14. Triggered by the user reporting token usage was too high, not by a lesson.

## Diagnosis

Producing Lesson 26 cost roughly: 22 KB lesson HTML, 6.3 KB learning record, 4.5 KB reference/glossary (output); 26 KB of opening reads (NOTES + MISSION + two records) and ~15 KB of Bash output (input). The multiplier is that the whole conversation is resent every turn, and that session had 20+ tool calls.

Three causes, in order:

1. **Lesson length drifted on its own.** Lesson 20 was 10 KB, Lesson 26 was 22 KB — doubled in six lessons. The teach skill itself says lessons must be short because working memory is small, so this is a teaching-design problem first and a cost problem second.
2. **NOTES.md had grown to 12 KB and was read in full every session**, with several entries covering the same ground.
3. **Verification round-tripped too much** — one compile was run against edition 2021 before noticing the project is 2024, printing the same E0382 twice.

## Measurement: bytes are the wrong unit

The first pass at this analysis used byte counts, which understated Chinese-heavy files. Measured properly (CJK ≈ 1 token/char, ASCII ≈ 4 chars/token):

| NOTES.md version | bytes | est. tokens |
|---|---|---|
| original, Chinese | 11,992 | ~3,222 |
| restructured, Chinese | 7,482 | ~2,360 |
| restructured, English | 7,476 | ~1,871 |

The middle two rows are the useful comparison: **same information, same byte count, 21% fewer tokens in English.** Restructuring saved 27%; the language switch saved a further 21%; 42% total.

This also means lesson HTML costs more than its byte size suggests, since lesson prose is Chinese and stays that way.

## Decisions

- **Lessons: 12–14 KB, 6–7 sections**, from Lesson 27. Overflow splits into two lessons or moves to a reference card.
- **Learning records: 2–3 KB**, decisions and open risks only. Verification still happens, but gets one line of conclusion.
- **NOTES.md and learning records are English.** `learning-records/` was already effectively English (0017–0019 each contain ~30 CJK chars, all quoted user phrases); NOTES.md was the outlier, and the first draft of this record broke the convention too. Lesson prose stays Traditional Chinese — that is a user requirement, not a cost decision.
- `GLOSSARY.md` (28 KB) is grepped, never read whole. Verification batched into one script.

## Not changed

Empirical verification stays. Several NOTES rules exist because I wrote something without testing it and got it wrong; Lesson 26 caught another (`: usize` turns out not to be required). Saving tokens here trades lesson correctness.

Lesson 26 is not being rewritten to the new limit — it is verified and complete, and rewriting would spend ~10 KB of exactly the resource being conserved.

## Open risks

- Whether 12–14 KB holds for an implementation lesson built on three compiler errors. Lesson 26 under the new rule would have split in two; untested.
- `GLOSSARY.md` keeps growing. Grep avoids the cost now, but a consistency sweep would hit it. May need splitting.
- Quoted user feedback inside English records is the one place CJK still belongs — the exact wording is the evidence. Don't translate those.
