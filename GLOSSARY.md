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
A marker saying "this type has a certain capability." A type either carries a given trait or it doesn't (e.g. `Copy`). You *implement* a trait for a type with `impl TraitName for TypeName { ... }`, filling in whatever functions the trait requires. See [[Display trait]] for a concrete example.

**`Display` trait**:
The trait that controls what a value prints as with the `{}` placeholder. Implement it with `impl std::fmt::Display for TypeName { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "...", ...) } }` — `write!` works like `format!` but writes into `f` instead of returning a `String`. Once implemented, `println!("{}", value)` and `value.to_string()` both just work. `io::Error` (the type behind `Err(error)` from `fs::read_to_string`) already implements `Display`, which is why `{}` could print it directly.

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

**`struct`**:
Defines a new type by grouping related named values, each called a **field** (欄位), each with its own type. The definition alone produces no data — it's a blueprint. An instance is created with `TypeName { field: value, ... }`; access a field with `.`. Prefer `String` over `&str` for fields unless you're ready to name a lifetime (see [[String vs &str]]).

**`impl` block**:
`impl TypeName { ... }` — where functions "belonging to" a type are defined. Holds two kinds of function, told apart only by whether the first parameter is `self`: a [[method]] has it, an [[associated function]] doesn't.

**Method**:
A function defined inside an `impl` block whose first parameter is `self` (as `&self`, `&mut self`, or `self`). Called with `value.method()` — the value before the dot becomes `self` automatically, so it isn't passed again explicitly. `&self` reads fields, `&mut self` writes them, bare `self` consumes the value; same borrowing rules as any other [[borrow]].

**Associated function**:
A function defined inside an `impl` block with no `self` parameter. Called with `TypeName::function()`, not with a dot. `String::from`, `String::new`, and `Vec::new` are all associated functions — `new` is a naming convention for "build an instance," not a keyword.

**Lifetime (生命週期) / `'static`**:
An annotation describing *how long* a reference is valid for — a topic this workspace hasn't formally covered yet. `'static` is the special case meaning "valid for the entire program" (e.g. string literals like `"空行"`, baked into the executable). Named here only so it can be searched for; full lifetime syntax (`'a`, etc.) is a later lesson. Don't confuse with [[mutability]] — a `let mut` error (`E0596`) is about whether a value can be *changed*, not about how long a reference *lives*.

**Tuple（元組）**:
A fixed-size grab-bag of values, possibly of different types, written `(a, b, c)`. The type is written the same way: `(i32, i32)`. Access an element by position with `.0`, `.1`, `.2`, ... (zero-indexed) — there is no name, only a slot number. Useful for a function that needs to return more than one value without defining a whole [[struct]] for it: `fn f() -> (usize, usize)`. Destructure on the caller's side with `let (a, b) = f();`.

**Custom error type**:
An [[enum]] you define yourself, with one variant per way your program can fail, used as the `E` in `Result<T, E>`. Lets one function report several distinct failure reasons (e.g. "file not found" vs "file was empty") through a single return type, instead of every caller inventing its own strings. Give it a [[Display trait]] implementation so `Err(e)` prints a clean message with `{}`.

**`enum`**:
Defines a type by listing the fixed set of shapes a value can take — a **variant**. A value is always exactly one variant, never several at once. Contrast with [[struct]]: struct combines fields together (AND), enum picks one variant among several (OR). A variant can carry its own data, e.g. `Some(T)` in [[Option]] or `Ok(T)`/`Err(E)` in [[Result]] — both are ordinary enums you've used since early lessons, just without seeing the `enum` keyword behind them. [[match]] is how you branch on which variant a value is, pulling out any data the variant carries at the same time.

**`?` operator**:
Placed after an expression that returns [[Result]] (or `Option`): on `Ok(v)` the expression evaluates to `v` and execution continues; on `Err(e)` the whole function returns `Err(e)` immediately. Only legal inside a function whose own return type is `Result`/`Option`. Replaces the boilerplate `match ... { Err(e) => return Err(e), Ok(v) => v }`. If the error type doesn't match the function's declared error type, `?` converts it automatically via [[From trait]] — without a matching `impl From`, it fails to compile.

**From trait**:
`impl From<A> for B` teaches type `B` how to be built from a value of type `A`, via a `fn from(a: A) -> B` method. The [[`?` operator]] calls this automatically when the error type it sees doesn't match the function's declared error type — e.g. `impl From<std::io::Error> for AppError` lets `?` turn an `io::Error` into an `AppError` without writing the conversion by hand at every call site.

**`if let`**:
Shorthand for a [[match]] that only cares about one variant, e.g. `if let Err(error) = run() { ... }`. Unlike `match`, it doesn't require every variant to have a branch — the pattern either matches (run the block) or it doesn't (skip it, do nothing). Use it when every other branch would just be an empty `{}`.

**Standard streams**:
Two separate output channels a program writes to. `println!` writes to `stdout` (normal output); `eprintln!` writes to `stderr` (error/diagnostic output). A terminal shows both interleaved, so they look identical — the difference shows up when output is redirected, e.g. `program > out.txt` only captures `stdout`; error messages printed with `eprintln!` still show up on screen.

**`eprintln!`**:
Same as `println!`, but writes to [[Standard streams]] `stderr` instead of `stdout`. Use for error messages, so a script piping the program's normal output (`program > out.txt`) doesn't get error text mixed into the captured data.

**Exit code**:
The number a process reports to whoever ran it (usually the shell) when it finishes. `0` is the convention for success; any nonzero value means failure. A shell script can check it via `$?`, or directly in `if my_tool file.txt; then ...`. A program that fails but still exits with `0` will make calling scripts believe it succeeded.

**`std::process::exit(code)`**:
Immediately ends the program and reports `code` as its [[exit code]]. Needed because letting `main()` return normally after printing an error still exits with `0` — `process::exit` is how a program reports failure to whatever called it, not just to a human reading the terminal.

**`mod`**:
Declares that a file is part of the current [[Crate]] and should be compiled, e.g. `mod report;` in `main.rs` tells the compiler to compile `src/report.rs` as a module named `report`. Without this declaration, an existing `.rs` file is invisible to the compiler — Rust does not scan the `src/` directory automatically.

**`pub`**:
Marks an item (struct, function, enum, ...) as visible outside the file/module it's defined in. Everything is private by default, the opposite of TypeScript's "everything exported unless private." A struct's fields can stay private even when the struct itself is `pub`, as long as outside code only calls `pub` methods on it rather than reading fields directly.

**`use`**:
Brings a path into the current scope so it can be written short instead of in full, e.g. `use report::Report;` lets you write `Report` instead of `report::Report`. Purely a naming convenience — it doesn't change what's compiled or what's visible, that's [[`mod`]] and [[`pub`]]'s job. `crate::` at the start of a path means "starting from the project root."

**`#[test]`**:
Marks a function as a test — `cargo test` runs every function tagged with it and reports pass/fail. An ordinary function isn't executed automatically; this attribute is what makes it happen.

**`#[cfg(test)]`**:
Compiles the item below it only when running `cargo test`. Placed above `mod tests { ... }` so test code never ships inside the binary built by `cargo build`/`cargo run`.

**`assert_eq!` / `assert!`**:
Test-time checks. `assert_eq!(a, b)` panics if `a != b`, printing both actual values. `assert!(cond)` panics if `cond` is `false`. A panic inside a `#[test]` function doesn't crash `cargo test` as a whole — that one test is marked `FAILED` and the rest still run.

**Iterator（疊代器）**:
A value that produces elements one at a time on demand. `content.lines()` is one. It isn't a collection — nothing is computed or stored up front. Methods like `.filter()` and `.map()` are *lazy*: they only stack a rule on top and return a new iterator. Work happens when a consuming method (`.count()`, `.collect()`) or a `for` loop drains it.

**`.filter()` / `.map()` / `.count()` / `.collect()`**:
`.filter(|x| ...)` keeps only elements matching a condition (count may shrink). `.map(|x| ...)` replaces each element with another value (count unchanged). `.count()` drains the iterator and returns how many elements passed through. `.collect()` drains it into a collection, and needs the target type annotated (`let v: Vec<i32> = ...`). Same names and semantics as the JS array methods, except JS builds a new array at every step and Rust defers everything to the consuming call. See [[Iterator（疊代器）]].

**Closure（閉包）**:
An inline anonymous function written `|arg| expression`, e.g. `|n| n * 2`. Equivalent to TypeScript's `(n) => n * 2`. Argument types are usually inferred from context, so they're rarely written out.

**Match guard（守衛）**:
An extra `if` condition attached to a `match` arm: `_ if arg.starts_with('-') => ...`. The arm is taken only when the pattern matches *and* the guard is `true`. The guard can read any variable in scope, not just the value being matched. Arms are tried top to bottom with no precedence rules, so a specific literal arm must be written above a guard arm that would also accept it.

**`.as_str()`**:
Borrows a `&str` view of a `String` without copying. Needed when matching a `String` against string literals — `match arg { "--help" => ... }` fails with `expected `String`, found `&str`` because the scrutinee and the patterns must be the same type. See [[`String` vs `&str`]].

**`.skip(n)`**:
Iterator adapter that discards the first `n` elements. Lazy, like `.filter()`. `env::args().skip(1)` is the idiomatic way to drop the program's own path from the argument list — no need to `collect()` into a `Vec` first. See [[Iterator（疊代器）]].

**Flag（選項）**:
A command-line argument that names itself rather than relying on its position, e.g. `--count-only`. Parsing by content instead of by index is what lets `tool --count-only f.txt` and `tool f.txt --count-only` behave identically. An unrecognised flag should be a returned `Err`, not a printed hint — otherwise the user believes it took effect.

**`HashMap<K, V>`**:
A key-to-value collection from `std::collections` (not in the prelude — needs an explicit `use`). `.insert(k, v)` writes and overwrites, `.get(&k)` returns `Option<&V>` (a *reference*, so it compares as `Some(&2)`), `.len()` counts entries. Closest TypeScript equivalent is `Map<K, V>`; the difference is that `get` hands back an `Option` rather than a directly usable `V | undefined`. See [[`Option`]].

**`.entry(k).or_insert(v)`**:
The idiomatic way to count with a `HashMap`. `.entry(k)` grabs the slot for a key whether or not it holds anything; `.or_insert(v)` fills it with `v` when empty and returns `&mut V` either way. Hence `*counts.entry(k).or_insert(0) += 1;` — the leading `*` dereferences that mutable reference so the `+= 1` lands on the number rather than on the reference.

**HashMap iteration order**:
Unspecified. Not insertion order, not sorted order, and not even stable between runs of the same program — this is by design, since a hash map's job is key lookup. For human-facing output, `collect()` into a `Vec` and `sort()` it, or use `BTreeMap`, which iterates in key order.

**`.split_whitespace()`**:
Splits a `&str` into an iterator of words on any whitespace run. One of the string-specific iterator sources alongside `.lines()` and `.chars()`. Yields borrowed `&str`s, so storing them as `HashMap` keys needs `.to_string()`. See [[Iterator（疊代器）]].
