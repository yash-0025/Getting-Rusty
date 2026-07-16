# 📘 LEARNING.md — Living Progress Journal

> This file is the **source of truth for actual progress**. The AI tutor reads this before each session to calibrate pacing, but will **never edit this file without asking first.**
> Update this yourself as you go, or ask the AI to "log today's progress" at the end of a session — it will draft the entry and show you before writing anything.

**Status legend:** `[ ]` not started · `[~]` in progress · `[x]` done & understood · `[!]` done but shaky, needs revisit

### ⚙️ Environment
- **OS:** Windows 11 + **WSL** (Ubuntu) — Rust toolchain is installed in WSL, NOT native Windows
- **Toolchain:** rustc 1.96.0, cargo 1.96.0, rustup 1.28.2 (stable channel)
- **IDE:** VS Code on Windows, connected to WSL
- **Rule:** All `cargo`/`rustc` commands run in WSL. If the AI needs to execute commands, use WSL paths (`/mnt/c/Dev/Rust/...`), not Windows paths.

### 🗣️ Teaching Style
- **Use simple, everyday English.** No fancy words or dense academic writing. Talk like a friend explaining things, not like a textbook.
- **Show through code, not words.** Short code examples are better than long paragraphs. Let the code do the talking.
- **Use real-world analogies** from JS/TS/Solidity that the learner already knows. Make new things feel familiar first, then show how they're different.
- **One small step at a time.** Don't explain 5 things in one go. Break things into small, easy-to-digest pieces.
- **Always teach before assigning.** Explain the concept fully with examples before asking the learner to write code.

---

## 📊 Quick Progress Snapshot
*(Update this table as weeks complete — ask the AI to refresh it, it will propose the update and wait for approval.)*

| Week | Focus Area | Status |
|---|---|---|
| Week 1 | CLI Tools & Core Rust | `[x]` Done |
| Week 2 | Libraries, Generics & Type System Mastery | `[x]` Done |
| Week 3 | Concurrency, Async & Production Web Services | `[~]` In progress |
| Week 4 | Advanced Patterns & Production Capstone | `[ ]` Not started |

---

## Day-by-Day Log

> Format for each entry once you start a day:
> ```
> ### Day N — <topic title> — <date>
> **Status:** [~] in progress
> **What I actually understood:**
> -
> **What's still fuzzy / questions I had:**
> -
> **Code I wrote / project progress:**
> -
> **Mistakes the compiler caught that taught me something:**
> -
> ```

### Day 0 — Setup — 2026-06-25
**Status:** `[x]` done
**What I actually understood:**
- Set up the four governing files (`ROADMAP.md`, `LEARNING.md`, `LOGS.md`, `KICKOFF_PROMPT.md`) and the rules for how they interact.
**What's still fuzzy / questions I had:**
- N/A — pre-learning setup day.
**Code I wrote / project progress:**
- None yet — Day 1 starts the actual curriculum.
**Mistakes the compiler caught that taught me something:**
- N/A

### Day 1 — Build: "Hello Cargo" & Project Scaffold — 2026-06-26
**Status:** `[x]` done
**What I actually understood:**
- `rustup` manages versions, `rustc` compiles, `cargo` is the package manager/build tool.
- Incremental compilation makes `cargo run` fast after `cargo check`.
- `Option<String>` vs `String` — Rust makes null pointer errors impossible by forcing us to handle `None` via `match`.
- `rust-toolchain.toml` pins the version.
- `cargo fmt` to format, `cargo clippy -- -D warnings` for CI linting.
**What's still fuzzy / questions I had:**
- N/A
**Code I wrote / project progress:**
- Created `hello-rust` project, added `std::env::args()` parsing to accept a name argument.
**Mistakes the compiler caught that taught me something:**
- Moving out of a `Vec` index (e.g. `args[1]`) isn't allowed without a borrow (`&args[1]`) because `Vec` owns the strings.

---

### Day 2 — Build: Multi-Unit Converter CLI — 2026-06-28
**Status:** `[x]` done
**What I actually understood:**
- Variables are immutable by default (`let`). Use `mut` to allow changing values.
- Shadowing (re-declaring with `let`) allows changing data types (e.g. `String` to `f64`), unlike `mut` which only changes the value.
- Enums are powerful algebraic data types, not just string/number mappings. They force exhaustive handling via `match`.
- `std::io::stdin().read_line(&mut string)` borrows a mutable reference to fill a string with user input.
- `match` is an expression and can return a value (e.g. `let choice = match ...`).
- Functions implicitly return the last expression if you omit the semicolon (`;`).
**What's still fuzzy / questions I had:**
- Using `expect()` is fine for now but crashes the program on bad input. (Will learn better error handling soon).
**Code I wrote / project progress:**
- Built `unit-converter` project with a fully working CLI menu, input parsing, enums, match statements, and custom math functions.
**Mistakes the compiler caught that taught me something:**
- Unused variables and unused `mut` (caught by `cargo check`).
- Modifying immutable variables (E0384).
- Dead code / Unused imports warnings when things are defined but not used.
- Attempting to use a number as a type (`f: 64` instead of `f: f64`).
- Attempting to match on floats (parsing menu input as `f64` instead of `u32`), which Rust forbids due to precision issues.

---

### Day 3 — Build: File Duplicate Finder — 2026-07-01
**Status:** `[x]` done
**What I actually understood:**
- Stack vs Heap: Stack is fast/fixed size (like `i32`), Heap is for dynamic/growing data (like `String`).
- Move Semantics: Data has one owner. Passing a `String` moves ownership to avoid double-free bugs.
- `Copy` vs `Clone`: Simple stack types copy automatically. Heap types require explicit `.clone()` to duplicate data.
- Borrowing (`&T`): Looking at data without taking ownership (like looking at a library card).
- Mutable Borrowing (`&mut T`): "Many readers OR one writer, never both." Prevents data races.
- Reading directories and metadata using `std::fs`.
- Grouping data using `HashMap` and the `.entry().or_insert()` pattern.
- Hashing files using `DefaultHasher` to find exact content matches.
**What's still fuzzy / questions I had:**
- None for now.
**Code I wrote / project progress:**
- Completed the `duplicate-finder` CLI that finds duplicate files based on size and content hash.
**Mistakes the compiler caught that taught me something:**
- E0382 "borrow of moved value": Learned that passing a `PathBuf` to `fs::read` moves it, so we must pass it as a reference (`&path`).
- Unused variable warnings: The compiler points out when a variable (like `size`) is declared in a loop but never used.

### Day 4 — Build: Task Tracker (In-Memory CRUD) — 2026-07-02
**Status:** `[x]` done
**What I actually understood:**
- Structs group data together. `derive(Debug)` lets us print them.
- `impl` blocks contain methods (`&self`, `&mut self`) and constructors (`Self::new()`).
- Enums represent states (like `TaskStatus::Todo`).
- Vectors (`Vec<Task>`) store multiple tasks.
- `match` forces handling every enum variant exhaustively.
- `Option<T>` handles "maybe null" cases safely without null pointers.
- Closures (`|t|`) are like arrow functions.
- `if let` is a shorthand for matching on a single variant like `Some`.
**What's still fuzzy / questions I had:**
- None for now.
**Code I wrote / project progress:**
- Built `task-tracker` project (in-memory CRUD) using Structs, Enums, Vectors, Option, and iterator search.
**Mistakes the compiler caught that taught me something:**
- Dead code / Unused warnings (expected when building out enums/methods not used in `main` yet).
- `E0382: borrow of moved value` on vectors. Fixed by iterating over a reference `&task_list`.

### Day 5 — Build: Persistent Task Tracker with Error Handling — 2026-07-03
**Status:** `[x]` done
**What I actually understood:**
- `Result<T, E>` handles expected failures gracefully without crashing.
- `?` operator is a shortcut to automatically return `Err` if a function fails.
- `main()` can be changed to return a `Result` to bubble errors up to the OS.
- `serde` and `serde_json` handle converting structs to/from JSON strings.
- We must add `#[derive(Serialize, Deserialize)]` to our structs.
- `#[must_use]` warns us when we ignore a `Result` that might contain an error.
**What's still fuzzy / questions I had:**
- Type inference for generic functions like `serde_json::from_str` can be tricky when variables aren't strictly typed.
**Code I wrote / project progress:**
- Created `persistent-tracker`. Upgraded the Day 4 task tracker to load and save tasks to `tasks.json`.
**Mistakes the compiler caught that taught me something:**
- `E0282: type annotations needed`. Learned that `from_str` needs to know what type it's parsing into via the variable assignment.
- Unused `Result` warning. Learned to add `?` to `fs::write` to handle file writing errors.

### Day 6 — Build: Text Analytics Engine — 2026-07-04
**Status:** `[x]` done
**What I actually understood:**
- HashMaps are powerful key-value stores.
- The `Entry` API (`.entry().or_insert()`) makes updating HashMaps much cleaner than checking if a key exists first.
- Pointer/References (`&` and `*`): `&` is a reference (pointer) to data, and `*` dereferences it so we can read or write the actual value.
- `String` vs `&str`: `String` owns data on the heap, `&str` just points to a slice of a string. This saves huge amounts of memory.
- Iterator Adapters (`.map()`, `.filter()`, `.count()`, `.sum()`) allow us to process streams of data efficiently without writing manual loops.
- Closures (`|x|`) are like arrow functions in JS, allowing inline logic for iterators.
**What's still fuzzy / questions I had:**
- The concept of ownership feels slightly complex when sorting vectors of references, but chaining iterator methods is feeling much more natural.
**Code I wrote / project progress:**
- Created `text-analyzer` project that calculates word frequency, sentence counts, average length, and reading level using Iterators.
**Mistakes the compiler caught that taught me something:**
- Missed parentheses when defining a tuple inside a Vector (`Vec<&str, i32>` vs `Vec<(&str, i32)>`), causing completely unrelated allocator errors. Learning to read the `help:` line!

### Day 7 — Build: Polished CLI Task Manager (Capstone) — 2026-07-06
**Status:** `[x]` done
**What I actually understood:**
- Module system (`mod`, `pub`, `pub(crate)`) and refactoring into multiple files (`models.rs`, `storage.rs`, `cli.rs`).
- `clap` with the derive API simplifies building robust CLIs.
- Unit testing with `#[cfg(test)]`, `#[test]`, and `assert_eq!`.
- The Newtype pattern (`struct TaskId(pub u64)`) prevents passing the wrong type by mistake.
- The Builder pattern makes object creation readable and scalable using method chaining (`.name().build()`).
- Iterators (`.retain()`, `.filter()`, `.count()`) are incredibly powerful for manipulating and querying vectors safely.
- Modifying a struct's schema (adding an ID) will break parsing of older JSON files (data migration).
**What's still fuzzy / questions I had:**
- None for now.
**Code I wrote / project progress:**
- Completed `capstone-tracker` with a fully featured, modular CLI using `clap`. Implemented Add, List, Complete, Delete, and Stats commands using persistent JSON storage and unit tests.
**Mistakes the compiler caught that taught me something:**
- E0423/E0425: Variable vs module name collisions (deleting a variable and trying to use it).
- Traits must be in scope to use their methods (e.g., `use clap::Parser`).
- E0004: Non-exhaustive patterns in `match` (Rust forces us to handle new Enum variants).
- E0308: Mismatched types — passing a string slice `&str` when a heap-allocated `String` is required by the Builder pattern.

### Day 8 — Build: Generic Stack & Queue Collection Library — 2026-07-07
**Status:** `[x]` done
**What I actually understood:**
- **Library Crates (`lib.rs`)**: How to build code for other developers to use instead of a binary app.
- **Generic Structs (`<T>`)**: Building data structures that can hold any type.
- `VecDeque`: Rust's Double-Ended Queue for O(1) front/back insertions and removals.
- **Deriving Traits**: Using `#[derive(Debug, Clone)]` to automatically implement traits, with the rule that the inner generic type must also implement them.
- **Defining Traits**: Creating custom traits (interfaces) like `Collection`, complete with default method implementations (`is_empty`).
- **Associated Types vs Generic Parameters**: Why `Iterator` uses `type Item = T;` (a struct can only iterate one way) rather than `Iterator<T>` (where it could implement it multiple times).
- **Operator Overloading**: Using traits in `std::ops` (like `Add`) to redefine how operators like `+` work for custom structs.
**What's still fuzzy / questions I had:**
- None for now.
**Code I wrote / project progress:**
- Created a `collections` library with fully generic `Stack<T>` and `Queue<T>` data structures, complete with custom traits, standard trait implementations (`Iterator`, `Add`), and unit tests.
**Mistakes the compiler caught that taught me something:**
- N/A

### Day 9 — Build: Plugin-Based Shape Calculator — 2026-07-07
**Status:** `[x]` done
**What I actually understood:**
- **Static Dispatch (Monomorphization)**: Fast, zero-runtime-cost generics where the compiler copy-pastes a version of the function for every type used (e.g., `<T: Shape>`).
- **Dynamic Dispatch (`dyn Trait`)**: Using a vtable at runtime to determine which method to call, allowing heterogeneous collections (`Vec<Box<dyn Shape>>`). 
- **`Box` with Trait Objects**: Since trait objects have dynamic sizes, they must be boxed so the `Vec` can hold uniformly-sized pointers (8 bytes).
- **`impl Trait`**: Syntactic sugar for generics (`fn print_area(shape: &impl Shape)`).
- **Object Safety**: You cannot use `dyn Trait` if the trait returns `Self` or has generic methods (because the vtable cannot be constructed).
- **Enum vs Trait Object Tradeoff**: Use `enum` for closed, known sets of variants (faster, stack-based). Use `Box<dyn Trait>` for open, plugin-based architectures where outside code can add new variants.
**What's still fuzzy / questions I had:**
- None for now.
**Code I wrote / project progress:**
- Created the `shapes` binary project showcasing both static and dynamic dispatch.
**Mistakes the compiler caught that taught me something:**
- N/A

### Day 10 — Build: Zero-Copy Config Parser — 2026-07-08
**Status:** `[x]` done
**What I actually understood:**
- **Lifetimes (`'a`)**: A descriptive label (sticky note) for the compiler to prove a reference won't outlive its original data. It does not control memory, it only describes it.
- **Zero-Copy Architecture**: Parsing strings by pointing directly to slices of the original string in memory (`&str`), saving the slow heap allocations of `.clone()` or `String::from()`.
- **Struct Lifetimes (`struct Config<'a>`)**: A struct holding a reference must declare a lifetime so the compiler can track the expiration date of its contents.
- **`'static` Lifetime**: Data carved into the binary (like string literals) that lives for the entire duration of the program.
- **Lifetime Bounds (`T: 'a`)**: Forcing a generic type to have an expiration date at least as long as `'a`.
- **Ergonomic Conversions (`From`/`Into`)**: The blanket implementation where implementing `From` gives you `Into` for free, heavily used in function arguments (`impl Into<String>`).
**What's still fuzzy / questions I had:**
- None for now.
**Code I wrote / project progress:**
- Created the `config_parser` binary project showcasing a fast zero-copy parser.
**Mistakes the compiler caught that taught me something:**
- Missing lifetime specifier error: Taught me that when returning a reference from a function with multiple input references, the compiler needs explicit lifetime tags to know which input the output is tied to.

<!-- New day entries get appended below this line. Ask the AI to draft an entry at the end of each session; approve or edit before it's saved. -->

### Day 11 — Build: Expression Evaluator (Mini Calculator) — 2026-07-09
**Status:** `[x]` done
**What I actually understood:**
- **Recursive Enums and `Box<T>`**: Rust needs to know exact sizes at compile time. If an enum contains itself, it is infinite. `Box<T>` breaks the cycle by putting the data on the heap and storing a fixed-size pointer (8 bytes) on the stack.
- **Reference Counting (`Rc<T>`)**: Allows multiple variables to own the exact same data. `.clone()` doesn't copy the data, it just increments an integer counter (O(1) time). The data is freed when the counter hits 0. It only allows *immutable* sharing.
- **Interior Mutability (`RefCell<T>`)**: Rust usually stops you from mutating data if there are multiple owners. `RefCell` bypasses compile-time checks and moves them to runtime (using `.borrow()` and `.borrow_mut()`). Useful when you absolutely need shared state.
- **Deref Coercion**: The `Deref` trait lets the compiler automatically insert `*` (dereference) operators. This is why you can call `left.eval()` on a `Box<Expr>` instead of explicitly writing `(*left).eval()`.
**What's still fuzzy / questions I had:**
- None for now.
**Code I wrote / project progress:**
- Created `expression_evaluator` using an AST (Abstract Syntax Tree) to mathematically evaluate nested expressions like `5.0 + (3.0 * 2.0)`.
- Wrote a deep pattern matching `eval` method to parse the tree recursively.
- Explored `Rc<RefCell<T>>` to achieve Shared Mutable State.
**Mistakes the compiler caught that taught me something:**
- E0072 (infinite size): The compiler caught a recursive enum without indirection and correctly suggested wrapping it in a `Box`.

### Day 12 — Build: File System Tree Simulator — 2026-07-11
**Status:** `[x]` done
**What I actually understood:**
- **Trees in Rust**: Parent-child relationships are notoriously hard in Rust because of ownership rules. A parent owns its children (`Rc<RefCell<Node>>`), but children need to point back up to the parent for traversal. 
- **Reference Cycles**: If the child also strongly owns the parent (`Rc`), their internal `strong_count` will never reach 0. They keep each other alive forever, causing a permanent memory leak.
- **Weak Pointers (`Weak<T>`)**: To fix cycles, you downgrade an `Rc` into a `Weak` pointer (`Rc::downgrade(&parent)`). `Weak` doesn't stop memory from being freed. 
- **Upgrading**: You can't read a `Weak` pointer directly because the data might be gone. You must call `.upgrade()`, which safely returns an `Option<Rc<T>>`.
- **The `Drop` Trait**: Rust manages memory deterministically. When variables go out of scope, Rust automatically calls their `drop()` method to clean up memory. We proved our `Weak` pointer fixed the leak because we watched the `Drop` cascade destroy the entire tree successfully!
**What's still fuzzy / questions I had:**
- None for now.
**Code I wrote / project progress:**
- Built `file_system` simulator showcasing `Rc` and `Weak` pointer graphs.
**Mistakes the compiler caught that taught me something:**
- E0282 (type annotations needed): When I forgot to actually assign the `Rc::downgrade` result to `rust_weak`, Rust's type inference crashed because it couldn't guess the unassigned type.

### Day 13 — Build: Comprehensive Test Suite + Documentation — 2026-07-12
**Status:** `[x]` done
**What I actually understood:**
- **Unit Tests:** Run alongside the code in a `#[cfg(test)]` module and have access to private functions. 
- **TDD (Test-Driven Development):** Writing the test before the implementation (Red, Green, Refactor).
- **Integration Tests:** Live in a separate `tests/` directory and can only access the `pub` API of a crate.
- **Doc Tests:** `cargo test` compiles and runs markdown code blocks inside `///` comments, ensuring documentation never lies.
- **Edge Cases:** Using `#[should_panic]` to explicitly verify that code crashes correctly on bad input.
- **Closures vs Function Pointers:** `fn` is a raw memory address with no state. `Fn` (closures) are actually structs generated by the compiler under the hood, which allows them to capture and store variables from their environment.
**What's still fuzzy / questions I had:**
- None for now.
**Code I wrote / project progress:**
- Wrote unit tests, integration tests, and doc-tests for the `collections` library.
- Fixed a bug in `Stack` by implementing `peek()` via TDD.
- Tested edge cases and panics in `expression_evaluator`.
**Mistakes the compiler caught that taught me something:**
- E0308: Mismatched types. When writing the parser panic test, passed a `&str` directly into `Parser::new()` instead of passing it through the `Lexer` first.

---

### Day 14 — 🏁 Week 2 Capstone: Generic In-Memory Cache with TTL — 2026-07-16
**Status:** `[x]` done
**What I actually understood:**
- **PhantomData<T>**: Used the typestate pattern to enforce compile-time rules using a ZST (Zero-Sized Type) that doesn't actually exist in memory.
- **Default Type Parameters**: `<Context = ()>` lets users skip specifying generics if they just want the default behavior.
- **Const Generics**: `<const N: usize>` allows us to pass raw numbers into type signatures instead of types, which we used to enforce a strict hard-coded max capacity on the Cache.
- **Box<dyn Fn>**: Learned how to store an unknown-sized closure inside a struct using Dynamic Dispatch and a heap allocation.
- **Trait Bounds & Lifetimes for Closures**: Enforced that the eviction callback closure must be `Fn(&K, &V) + 'static` to prevent dangling pointers.
- **Unit Testing**: Wrote a `#[cfg(test)]` module and used `thread::sleep` to verify the TTL expiration logic actually worked.
**What's still fuzzy / questions I had:**
- None for now!
**Code I wrote / project progress:**
- Built `in_memory_cache`, a production-ready cache library featuring lazy TTL expiration, Const Generic capacity limits, and dynamic closure callbacks.
**Mistakes the compiler caught that taught me something:**
- The test panicked on `assert_eq!` because the Const Generic capacity limit of 2 blocked the 3rd and 4th items from being inserted. Fixed by increasing the limit to 4.

## 🧠 Concept Confidence Tracker
*(Self-rate honestly — this drives what the AI re-explains or drills. Update anytime, ask the AI to revise the table for you if you want.)*

| Concept | Confidence (1-5) | Last touched |
|---|---|---|
| Ownership & Move Semantics | — | — |
| Borrowing Rules | — | — |
| Lifetimes | — | — |
| Traits & Generics | — | — |
| Static vs Dynamic Dispatch | — | — |
| Error Handling (`Result`/`?`/thiserror/anyhow) | — | — |
| Iterators & Closures | — | — |
| Smart Pointers (`Box`/`Rc`/`RefCell`/`Arc`) | — | — |
| Threads & `Mutex`/`Arc` | — | — |
| Async/Await & Tokio | — | — |
| Axum Web Services | — | — |
| `sqlx` / Database Layer | — | — |
| `unsafe` Rust | — | — |
| Macros | — | — |
| Performance Profiling | — | — |

---

## 🏗️ Projects Built
*(One entry per project, filled in as you complete each one — ties back to the "Project hook" in ROADMAP.md)*

| Project | Week | Status | Repo/Path | Notes |
|---|---|---|---|---|
| CLI Task Manager (polished) | 1 | `[x]` | `capstone-tracker` | Week 1 capstone |
| Generic Cache Library with TTL | 2 | `[x]` | `in_memory_cache` | Week 2 capstone |
| Production REST API (Docker) | 3 | `[ ]` | — | Week 3 capstone |
| Final Capstone (TBD) | 4 | `[ ]` | — | Portfolio centerpiece |

---

## ❓ Open Questions / Parking Lot
*(Anything you're curious about that's off the main path — log it here instead of derailing the day's topic. Revisit on Day 30 or whenever there's slack time.)*

-

---

## 🔁 Topics Flagged for Revisit (`[!]` shaky)
*(Auto-suggested by the AI based on your day-log entries, but only added here after you confirm.)*

-
