# Rust Glossary

Canonical terms for this workspace. Lessons and reference docs should use these terms consistently.

## Terms

**Crate**:
A compilation unit / package of Rust code — either a binary (produces an executable) or a library. `cargo new` creates a binary crate by default.

**Cargo**:
Rust's official build tool and package manager. Handles compiling, running, testing, and managing dependencies (crates.io packages) for a project.
_Avoid_: "the Rust package thing"

**`main` function**:
The entry point of a binary crate — execution always starts at `fn main()`. Required for any executable Rust program; a library crate does not have one.

**Macro**:
Code that generates code, invoked with a trailing `!` (e.g. `println!`). Distinct from a regular function call — covered only at the level of "recognize the `!` and know it's a macro" for now; macro internals are out of scope until later.

**Expression vs statement**:
An expression evaluates to a value (e.g. `a + b`, or an `if`/`else` block whose branches end without a semicolon). A statement performs an action and evaluates to nothing (`()`) — adding a semicolon to an expression turns it into a statement. Function bodies and `if`/`else` used as a value both rely on this distinction.

**`Vec<T>`**:
A growable list of values, all of the same type `T`. Created with `Vec::new()` (then `.push(...)`) or the `vec![...]` macro. Contrast with fixed-size [[arrays]].

**Array**:
A fixed-length list of values of the same type, e.g. `[i32; 3]` — the length is part of the type and can't change after creation. Stored on the stack (size known at compile time). Use when the count is truly fixed; otherwise prefer `Vec<T>`.

**`Result`**:
An enum representing either success (`Ok(T)`) or failure (`Err(E)`) of an operation (e.g. `read_line`, `parse`). `.expect("message")` is a shortcut that panics with the given message on `Err` — not proper error handling. Prefer `match` to handle both variants explicitly.

**`Option`**:
An enum representing either the presence of a value (`Some(T)`) or its absence (`None`). Rust has no `null` — "this might not exist" is expressed in the type, and the compiler forces you to handle `None` before you can use the inner value. Same two-variant shape as [[Result]]; the difference is meaning — `Result` is success/failure, `Option` is present/absent. Absence is not necessarily an error.

**`.get(i)` vs `[i]`**:
Two ways to index a `Vec`. `vec[i]` returns the element directly and panics if `i` is out of bounds. `vec.get(i)` returns `Option<&T>` — `None` instead of panicking. Use `.get()` whenever the index comes from outside the program (e.g. command-line arguments), so a missing value becomes a branch you handle rather than a crash.

**`match`**:
A control-flow construct that branches on which variant a value actually is (e.g. `Ok`/`Err`, or any enum). Exhaustive — every variant must have a branch, or the code fails to compile. Stricter than `if`/`else`, which only checks a boolean condition.

**`loop`**:
An unconditional loop that repeats forever until a `break` is hit. `break value` both exits the loop and makes `value` the result of the whole `loop` expression — the same "block as expression" pattern as `if`/`else`.

**Scope (作用域)**:
The region in which a variable exists — normally the pair of curly braces `{ }` enclosing it. Once execution leaves those braces the variable is gone; using it afterwards is `E0425: cannot find value in this scope`. When a variable holding a `String` goes out of scope, its memory is handed back to the system automatically ("freed"). No explicit free call is ever written — this is how Rust avoids needing a garbage collector.

**Trait**:
A marker saying "this type has a certain capability." Introduced at this level only: a type either carries a given trait or it doesn't (e.g. `Copy`). Defining and implementing traits is a later lesson.

**Ownership**:
Rust's memory management model, governed by three rules: every value has one owner; there is only ever one owner at a time; when the owner goes out of scope, the value is freed. This is why Rust needs neither a garbage collector nor manual `free` calls.

**Move**:
What assignment does for non-`Copy` types: `let s2 = s1;` transfers ownership to `s2` and invalidates `s1`. The data is neither copied nor cleared — only the owner changes. Passing a value into a function moves it the same way. Using the original afterwards is error `E0382: borrow of moved value` / `value borrowed here after move`. The single-owner rule exists to prevent double frees.

**`Copy` trait**:
Marks types that are duplicated on assignment instead of moved. The dividing line is **fixed size vs growable**, not "numbers vs everything else": all integer types, `f64`/`f32`, `bool`, and `char` are `Copy` because their size is known at compile time and copying is nearly free. Growable types (`String`, `Vec<T>`) are not `Copy` — their length is only known at runtime, so a copy means allocating and transferring the whole contents. Rust refuses to do that implicitly; you must write `.clone()` yourself.

**`.clone()`**:
Explicitly duplicates a value, giving you a second independent owner. The fix of last resort for a move error: it always works but really does copy the data. Prefer borrowing (`&`) when the callee only needs to read the value.

**Reference (引用)**:
A value that points at another value rather than being it. Written `&x`. Handing a reference to a function is called [[borrowing]]. A reference never owns what it points at, so when it goes out of scope nothing is freed.

**Borrow (`&`)**:
Passing a reference so a function can read a value without taking ownership — e.g. `for name in &names` looks at each element without consuming `names`. Contrast with [[move]]: the owner does not change. `&mut` is a *mutable borrow*, which also allows modification.

**Borrow rules**:
At any one moment a value may have either **any number of `&`** (read-only borrows) **or exactly one `&mut`** — never both. A borrow's range ends at its *last use*, not at the closing brace, so reordering statements often fixes a violation. Errors: `E0502` (`&` and `&mut` coexist), `E0499` (two `&mut`), `E0596` (missing `let mut`). The rules exist to prevent data races.

**Data race (資料競爭)**:
Two places touching the same data at the same time, at least one of them writing, with no defined ordering. In Rust this is a compile error rather than a runtime bug — e.g. holding a `&` to a `String` while something calls `push_str`, which may reallocate and free the old memory the reference still points at.

**`String` vs `&str`**:
Two string types with different jobs. `String` owns its contents, can grow (`push_str`), and is freed when its owner goes out of scope. `&str` (a *string slice*) is a [[borrow]] of a stretch of characters — it owns nothing and cannot grow. String literals (`"hello"`) are `&str`, baked into the executable at compile time. **Function parameters should default to `&str`**: a `&str` parameter accepts both `&String` and literals, while a `&String` parameter rejects literals (`expected &String, found &str`). Use `String` for return values, struct fields, and anything you need to keep.

**Deref coercion**:
The mechanism that lets a `&String` be passed where a `&str` is expected — the compiler converts automatically. One-directional: `&String` → `&str`, never the reverse. Named here only so it can be searched for; the details are a later topic.

**Byte (位元組)**:
The smallest unit memory is counted in. An ASCII character (English letter, digit, punctuation) takes 1 byte; a Chinese character takes 3 in UTF-8. `String::len()` and slice ranges both count bytes, never characters.

**String slice (`&s[a..b]`)**:
A `&str` pointing at part of an existing string. No data is copied, so it is a borrow — while the slice is alive the source string cannot be modified. **The range counts [[bytes]], not characters**, so `"你好世界".len()` is 12 while `.chars().count()` is 4. Cutting mid-character panics: `byte index 1 is not a char boundary; it is inside '你' (bytes 0..3)`. Legal cut points (*char boundaries*) for that string are 0, 3, 6, 9. Rule: never index non-ASCII strings — use `.chars()` to count, take, or iterate.

**`#[must_use]` warning**:
`warning: unused return value of ... that must be used` means a function's return value was computed and then discarded, so the line does nothing. Common when writing `s.len();` as a statement instead of using the number. The compiler suggests `let _ = ...` to silence it deliberately.

**`f64`**:
A floating-point number type (can hold a decimal point), as opposed to integer types like `i32`. Used whenever a value needs fractional precision (e.g. a temperature conversion).

**Debug format (`{:?}`)**:
A `println!`/`format!` placeholder that prints a value's internal structure (e.g. a whole `Vec`'s contents), useful for debugging or summarizing composite data. Contrast with `{}`, which requires the type to implement plain `Display` formatting and can't print a `Vec` directly.

**`env::args()`**:
Returns an iterator over the program's command-line arguments. `.collect()` turns it into a `Vec<String>`. `args[0]` is always the program's own path — the first real user-supplied argument is `args[1]`.

**`--` (argument separator)**:
The Unix convention for "end of options — everything after this is a value, not a flag." In `cargo run -- report.txt`, it guarantees `report.txt` reaches your program. Not strictly required: cargo forwards unrecognized trailing arguments anyway, so `cargo run report.txt` also works. It becomes essential when an argument starts with `-` (e.g. `cargo run -- --version`), which cargo would otherwise claim as its own option.

**`fs::read_to_string`**:
Reads an entire file's contents into a `String`. Returns a `Result` (file may not exist, may lack permissions), so handle it with `match` rather than `.expect(...)` when a missing file is an expected user error, not a bug.
