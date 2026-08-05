# Lesson 9 complete; ownership split across two lessons

Lesson 9 (`Option`) practice verified in `hello_cli`: `args.get(1)` with a `match` on `Some`/`None`, early `return` in the `None` arm. All three success criteria pass — valid filename, missing file, no argument — with no panic on any path.

Ownership is being taught as **two** lessons rather than one:

- **Lesson 10** — move semantics only: the three ownership rules, `E0382`, `Copy` vs non-`Copy`, moves on function call, `clone` as the fix.
- **Lesson 11** — borrowing: `&`, `&mut`, and the borrow rules.

## Why

Ownership and borrowing together exceed one lesson's working-memory budget, and borrowing can't be motivated until move is understood — `&` only makes sense as the answer to "the callee doesn't need to own this." Move-first also matches the official book's order (4.1 then 4.2).

## How to apply

- Lesson 10 deliberately ends unresolved, pointing at the compiler's own `consider changing this parameter type ... to borrow instead` note. Lesson 11 must open by answering it.
- The user has been writing `&` since Lesson 4 (`for name in &names`, `&args[1]`) without an explanation. Lesson 11 should explicitly close that debt rather than introduce `&` as if it were new.
- Defer the ownership/borrowing reference card until Lesson 11, so one card covers both halves. Do not ship a half card after Lesson 10.
