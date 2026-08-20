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
A `println!`/`format!` placeholder that prints a value's internal structure (e.g. a whole `Vec`'s contents), useful for debugging or summarizing composite data. Contrast with `{}`, which requires the type to implement plain `Display` formatting and can't print a `Vec` directly. Standard library types like `Vec` already implement `Debug`; a type you defined yourself needs `#[derive(Debug)]` (or a hand-written `impl`) before `{:?}` will compile for it. See [[`#[derive(...)]`]].

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

**Lifetime (生命週期) / `'a`**:
An annotation describing *how long* a reference is valid for. Written `'a` (pronounced "lifetime a") — it's a label, not a type. It tells the compiler that certain references share the same validity period. Most of the time the compiler infers lifetimes via three **elision rules**: (1) each reference parameter gets its own lifetime, (2) if there's exactly one reference input, the output uses its lifetime, (3) if there's `&self`/`&mut self`, the output uses `self`'s lifetime. When all three rules fail to determine an output lifetime (e.g. two `&str` inputs returning a `&str`), you must annotate manually: `fn longer<'a>(a: &'a str, b: &'a str) -> &'a str`. A struct holding a reference also requires a lifetime parameter: `struct Foo<'a> { text: &'a str }` — meaning the struct can't outlive what it borrows from. `'static` is the special case meaning "valid for the entire program" (e.g. string literals baked into the executable). Don't confuse with [[mutability]] — `E0596` is about whether a value can be *changed*, not about how long a reference *lives*. See [[Lesson 34]].

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

**Attribute (`#[...]`)**:
A note attached to the item directly below it, read by the compiler before compilation — not code that runs when the program executes. `#[test]` and `#[cfg(test)]` are both attributes; so is [[`#[derive(...)]`]], which is the one kind that writes code for you instead of just marking or conditionally including an item.

**`#[test]`**:
Marks a function as a test — `cargo test` runs every function tagged with it and reports pass/fail. An ordinary function isn't executed automatically; this attribute is what makes it happen.

**`#[cfg(test)]`**:
Compiles the item below it only when running `cargo test`. Placed above `mod tests { ... }` so test code never ships inside the binary built by `cargo build`/`cargo run`.

**`#[derive(...)]`**:
An [[Attribute (`#[...]`)]] that writes an `impl Trait for Type` block for you, following one fixed mechanical rule per trait — e.g. `#[derive(PartialEq)]` on an enum means "equal if same variant, and any data inside it is also equal." Only works when the fields alone determine that one rule; `Display` can't be derived because the exact text to print is a human choice, which is why `impl fmt::Display for AppError` in `hello_cli/src/errors.rs` is hand-written. A struct or enum can only derive a trait if every one of its own fields also implements that trait. See [[Trait]].

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

**Slice（切片）`&[T]`**:
A read-only window onto a run of contiguous elements — just a pointer and a length (16 bytes), no capacity, because a borrowed view can't grow. Both `[T; N]` and `Vec<T>` can hand one out, which is why the convention is *take `&[T]` as a parameter, return `Vec<T>`*: the parameter form accepts more callers, the return form gives back full ownership. `&str` is the text-specific version of the same idea. See [[`String` vs `&str`]].

**大小在編譯期必須已知**:
The single constraint that shapes Rust's whole type system: the compiler must know how many bytes a value occupies before it can emit code that stores it. `size_of::<Vec<i32>>()` is 24 no matter how many elements it holds, because a `Vec` value *is* three 8-byte fields (pointer, length, capacity) and the data lives elsewhere. A type whose size can't be computed is rejected outright — `recursive type 'List' has infinite size` — and the fix the compiler suggests is `Box`, which replaces the unknown-sized thing with a pointer.

**`BTreeMap<K, V>`**:
Same interface as `HashMap` (`insert`, `get`, `entry().or_insert()`), one behavioural difference: iteration is always in key order, and it's stable across runs. Costs slightly slower insert and lookup than `HashMap`. Use it when the output is for a human or when you need range queries; use `HashMap` when only lookup speed matters. Swapping one for the other is usually a three-line change. See [[HashMap iteration order]].

**`HashSet<T>`**:
A `HashMap` with no value column — it records only whether something is present. `.insert(x)` returns `bool`: `true` the first time, `false` if it was already there, which makes dedup and "have I seen this?" a single line. `.contains(&x)` reads it back.

**`VecDeque<T>`**:
A sequence that can push and pop at *both* ends quickly (`push_front` / `push_back` / `pop_front` / `pop_back`). `Vec` is fast only at the back. Reach for it when you need a queue.

**泛型參數（`<T>`, `<K, V>`)**:
The letters in `Vec<T>` / `HashMap<K, V>` are placeholders meaning "any type goes here, and I'll remember which one you chose". Identical in role to TypeScript's `Array<string>`. `T` is conventionally "type", `K`/`V` are "key"/"value" — the names carry no special meaning. Writing your own generic functions is a separate skill from reading them; when reading, treat the angle brackets simply as "what this container holds". A plain `<T>` accepts any type at all — it can only be moved around or stored, never compared or printed. See [[Trait bound]] for how a function demands more than that.

**Trait bound (`<T: SomeTrait>`)**:
A restriction narrowing a generic `<T>` down to "any type, as long as it implements this [[Trait]]". `fn largest<T: PartialOrd>(list: &[T]) -> &T` reads as "for any `T` that supports `>`/`<`, give me its largest element". Without the bound, `item > largest` fails to compile with `error[E0369]: binary operation `>` cannot be applied to type `&T``, because the compiler refuses to assume a comparison exists for an unconstrained type. Multiple bounds stack with `+`: `T: Display + PartialOrd`. This is the same shape as TypeScript's `function largest<T extends Comparable>(list: T[]): T` — the `extends` clause is TypeScript's trait bound. See [[Lesson 28]].

**閉包（Closure）`|x| ...`**:
A function value that also remembers the local variables around where it was written. That memory is the *only* essential difference from a plain `fn` — writing a `fn` that reads a surrounding local fails with `can't capture dynamic environment in a fn item`, and the compiler itself suggests ``use the `|| { ... }` closure form instead``. Syntactically it's TypeScript's arrow function with different brackets: `|x: i32| -> i32 { x + 1 }` collapses to `|x| x + 1` because the compiler can see the call site. That inference happens once — after the first call fixes `x` to `i32`, a later float argument won't compile.

**捕捉環境（Capturing the environment）**:
What a closure does to the variables it uses from the surrounding scope. How it captures follows the same three options as [[所有權（Ownership）]]: read-only (`&T`), mutable (`&mut T`), or by value. You never declare which — the compiler reads the closure body and picks the least demanding one that works.

**`Fn` / `FnMut` / `FnOnce`**:
The three categories a closure falls into, named after what it does to what it captured. `Fn` only reads it, `FnMut` modifies it (the closure variable itself must then be `mut`), `FnOnce` consumes it and can therefore only be called once — `closure cannot be invoked more than once because it moves the variable out of its environment`. These names matter for *reading* signatures, not writing closures: `impl Fn(&str) -> bool` in a parameter means "give me anything callable that takes a `&str`, returns a `bool`, and won't mutate what it captured".

**`move` 閉包**:
`move ||` forces the closure to take ownership of everything it uses instead of borrowing. Needed when the closure outlives the variables it captured — most commonly `thread::spawn(move || ...)` and async code. After it, the outer scope can no longer use those variables. Not something you write by choice yet; it's something you'll read.

**`sort_by` / `retain` / `sort_by_key`**:
The `Vec` methods that take a closure. `sort_by(|a, b| a.cmp(b))` sorts ascending; swapping the operands to `b.cmp(a)` reverses it. `sort_by_key(|x| ...)` is the more readable form when you're sorting on one field. `retain(|x| ...)` deletes non-matching elements in place. `retain` is the one that shows off capture — its closure can test against a local variable, which a plain `fn` could never do. See [[HashMap iteration order]] for why sorting a `Vec` is the standard move for human-facing output.

**`while let`**:
`if let` that repeats. `while let Some(x) = iter.next() { ... }` keeps destructuring until the pattern stops matching — for an iterator that means until `next()` returns `None`. The loop's exit condition is a failed pattern match, not a counter. Reach for it over `for` when the loop body needs to advance the iterator itself; `for` takes ownership of the iterator via `into_iter()`, so calling `next()` inside a `for` body gives `borrow of moved value`.

**`parse`**:
`"5".parse()` turns text into a number (or anything implementing `FromStr`). It returns `Result<T, ParseIntError>`, never a bare number, because the text may not be a number. The target type comes from the left-hand side — `let n: usize = value.parse()?` — or from a turbofish, `value.parse::<usize>()`. Everything a program receives from the command line or from a file starts as text; `parse` is the boundary crossing.

**帶值的旗標（Flag with a value）**:
A command-line option whose information sits in the *next* argument (`--min 3`, `head -n 20`), as opposed to a boolean switch (`--count-only`). The parsing loop must consume that next argument itself, or it falls through to the filename branch. Failing to consume it is not a compile error — it produces a bug that depends on argument order, which is why the pattern is worth knowing by shape. See [[`while let`]].

**Derived field（衍生欄位）**:
A struct field whose value can be computed from another field — a line count beside the text it counts, a total beside the list it sums. It is a cache, and nothing in the type refreshes it. `&mut self` lets a method change the source field and leave the derived one behind, and that compiles cleanly. Two ways out: delete the field and compute in a method (nothing left to go stale), or keep it and update it in every mutating method (needs private fields and discipline). See [[Invariant]].

**Invariant（不變式）**:
A rule about a value that must hold at all times — "`line_count` equals the number of lines in `content`", "this `Vec` is always sorted". Rust enforces invariants about *memory* (ownership, borrowing, exhaustive `match`) but has no way to express one about *meaning*. Those are held up by private fields, by methods that are the only way in, and by tests. A test that pins an invariant asserts a relationship (`a == b + c`), not a literal value — a test asserting the literal passes while the code is still wrong. See [[Derived field（衍生欄位）]].

**`.fold(初始值, |acc, x| ...)`**:
An iterator consumer that carries an accumulator through every element. The closure receives the current accumulator and the current element, and must return the (possibly updated) accumulator. After all elements are consumed, `.fold()` returns the final accumulator. Same concept as JavaScript's `Array.prototype.reduce()`, but the initial value is always required. Reach for it when you find yourself writing multiple `.filter().count()` or `.filter().map().collect()` passes over the same iterator — `.fold()` lets you combine them into a single traversal. Closure parameter `mut acc` is a `let mut` binding (ownership), not a `&mut` reference. See [[Iterator（疊代器）]], [[Closure（閉包）]].

**Struct destructuring（結構解構）**:
`let TypeName { field1, field2, .. } = expr;` — pulls named fields out of a struct into local variables, like TypeScript's `const { field1, field2 } = obj`. The type name is required (the compiler needs to know which struct). Use `..` to ignore fields you don't need. Same mechanism as tuple destructuring (`let (a, b) = ...`) but with names instead of positions — which means swapping the field order in the `let` pattern doesn't change semantics, because fields are matched by name, not position.

**Field init shorthand（欄位初始化縮寫）**:
When a variable has the same name as a struct field, `TypeName { field }` is shorthand for `TypeName { field: field }`. Works in struct literals and patterns. Same idea as JavaScript's `{ x }` shorthand for `{ x: x }`. Applies to any named-field struct or enum variant.

**Type placeholder `_`（型別佔位符）**:
An underscore in a type annotation tells the compiler "infer this part yourself." `Vec<_>` means "I want a Vec; figure out the element type from context." The compiler needs *some* type information from you (e.g. the container is `Vec`, not `HashSet`) but can fill in the rest from the iterator or return type. Common with `.collect()`, which can produce many different collection types. Not the same as `_` in a pattern (which means "ignore this value").

**Trait object (`Box<dyn Trait>`)**:
A value whose concrete type is unknown at compile time — only the trait it implements is known. Written `Box<dyn Formatter>`, meaning "a heap-allocated value that implements `Formatter`; I don't know (or care) which struct it is." The compiler attaches a **vtable** (a small table of function pointers) so method calls can be resolved at runtime. Use trait objects when the choice of implementation depends on runtime data (e.g. user input selecting a format). Contrast with [[泛型參數]] + [[Trait bound]], which resolve at compile time. See [[Static dispatch vs dynamic dispatch]], [[Lesson 35]].

**`dyn`**:
Short for "dynamic." Placed before a trait name to form a trait object type: `dyn Formatter`. It tells the compiler that method calls on this value go through a vtable lookup at runtime, rather than being resolved statically. `dyn Trait` alone is unsized — you can't put it on the stack directly; wrap it in `Box`, `&`, or `Arc`. See [[Trait object]].

**`Box<T>`**:
A smart pointer that allocates `T` on the heap and owns it. The `Box` itself is a fixed-size pointer living on the stack; when it goes out of scope, the heap memory is freed. Two main uses: (1) storing a [[Trait object]] (`Box<dyn Trait>`) because the concrete type's size is unknown at compile time, and (2) building recursive data structures whose size would otherwise be infinite.

**Static dispatch vs dynamic dispatch（靜態派發 vs 動態派發）**:
Two ways to call a trait method. *Static dispatch* (generics, `fn f<T: Trait>(x: T)`) — the compiler generates a separate copy of the function for each concrete type; calls are direct and can be inlined; decided at compile time. *Dynamic dispatch* (trait objects, `Box<dyn Trait>`) — a single copy of the code exists; calls go through a vtable pointer at runtime; slightly slower but supports choosing the implementation based on runtime data. Rule of thumb: if the type is known at compile time, use generics; if it depends on user input or config, use a trait object. See [[Lesson 35]].

**`.ok_or_else(|| error)`**:
Converts an `Option<T>` into a `Result<T, E>`. `Some(v)` becomes `Ok(v)`; `None` becomes `Err(error)`, where `error` is produced by the closure you provide. Useful when a function returns `Option` but you're inside a function that returns `Result` and want to use `?` to propagate the failure. The sibling `.ok_or(error)` evaluates the error eagerly; `.ok_or_else` takes a closure so the error is only constructed when actually needed. See [[`?` operator]].

**`impl Trait`（回傳位置）**:
Written as `fn foo() -> impl Iterator<Item = i32>`. Means: "this function returns a concrete type that implements the trait, but I'm not naming it in the signature." The compiler still knows the exact type internally and performs static dispatch (no vtable, no heap). The concrete type is decided by the function body, not by the caller. Primary use case: returning iterator chains or closures whose types are anonymous and impossible to spell out. Limitation: the function must return exactly one concrete type — you cannot return different types from different branches. If you need that, use `Box<dyn Trait>` instead. See [[Lesson 36]], [[Static dispatch vs dynamic dispatch]].

**`impl Trait`（參數位置）**:
Written as `fn foo(val: impl Display)`. Syntactic sugar for a generic parameter: equivalent to `fn foo<T: Display>(val: T)`. Each call site can pass a different concrete type. Prefer this shorthand when the type parameter appears only once and you don't need to refer to it elsewhere in the signature. See [[Trait bound]], [[Lesson 36]].
