# Mission: Rust

## Why
User is a front-end (TypeScript) developer who wants to learn the Rust *language* itself, not the usage patterns of any one framework. The core motivation is **being able to read Rust code** — code written by others, generated code, and code inside libraries. Writing code is one part of that, not the whole of it. CLI tools are the vehicle for practice, not the destination.

*(Revised 2026-08-12 — see learning record 0016. The original framing was "ship CLI tools and backend services", which under-described the goal.)*

## Success looks like
- Can write, compile, and run a multi-file Rust project with Cargo from scratch
- Can design and use structs, enums, and pattern matching to model a CLI tool's data
- Can read/write files, parse command-line arguments, and handle errors without panicking
- Can ship a small real CLI tool (e.g. a file search or text-processing utility) end to end
- Can read an unfamiliar Rust file and say what each type annotation, trait bound, and attribute macro is doing — and why it has to be written that way

## Constraints
- User previously worked through hello_rust (ch1–7 of the Chinese translation of "The Rust Programming Language": hello world, guessing game, variables/functions/branches, ownership, structs, enums, packages) but has forgotten most of it and wants a full restart — treat as a true beginner, don't assume retention of prior material.
- Existing practice folders (hello_rust, branches, functions, variables) are kept as historical artifacts, not part of the active teaching path.
- User is already writing a Tauri project, so they have real Rust exposure. **That is the source of the motivation, not the teaching frame** — they explicitly asked not to structure the course around the Tauri ecosystem, on the grounds that it would teach usage patterns instead of the language. Tauri may be referenced in passing as a place a concept shows up; it must never be the scaffolding a lesson hangs on.

## Out of scope
- WebAssembly, embedded/systems programming.
- async: not scheduled yet — revisit once the language fundamentals are solid. (Previously written as "not chasing"; that was too strong, since reading `async fn` is within the reading-comprehension goal.)
