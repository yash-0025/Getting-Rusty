# 🦀 RUST MASTERY ROADMAP — 30-Day Intensive (Project-Based)

> **Created:** 2026-06-25 · **Restructured:** 2026-06-26 · **Target completion:** 2026-07-25 · **Pace:** 5+ hrs/day
> **Learner profile:** Full-stack (MERN) engineer, Web3/Solidity security background, prior light exposure to Rust via Solana/Anchor. Goal: become a production-grade Rust engineer capable of using Rust as core backend/systems architecture.
> **Rust edition target:** 2024 Edition · **Toolchain:** stable (1.96.0 as of 2026-06-25 — always run `rustc --version` and treat that as ground truth)
> **Philosophy:** Every day starts with **"Build ___"**. Concepts are taught just-in-time because the project demands them. You never learn a concept in a vacuum.

---

## 🔒 GOVERNANCE RULES (read before touching any file)

1. **No silent edits.** Neither `ROADMAP.md` nor `LEARNING.md` is ever modified by the AI without explicitly asking the learner first and getting a yes. This applies to checking off a topic, reordering, rewording, adding, or deleting anything.
2. **Every change is logged.** Any edit to any file in this workspace — once made — gets a corresponding collapsible entry in `LOGS.md` with full before/after diffs (see that file's format). No exceptions, no batching silently.
3. **`LEARNING.md` is the source of truth for progress.** `ROADMAP.md` is the curriculum. `LEARNING.md` is the living journal of what's actually been learned, built, and understood.
4. **The roadmap cross-checks the learning log.** Before starting a new topic, the AI should glance at `LEARNING.md` to see what's marked done, what's marked shaky/needs-review, and adapt pacing — but it still asks before changing the roadmap itself.
5. **Status markers used consistently across both files:**
   - `[ ]` Not started
   - `[~]` In progress
   - `[x]` Completed & understood
   - `[!]` Completed but shaky / needs revisit
6. **One concept at a time, in project context.** Don't dump 10 concepts at once. Teach one concept fully — explanation, real-world "why", code in the project, optimized vs naive version — before moving on, unless the learner explicitly asks to move faster.

---

## 📐 HOW THIS ROADMAP IS STRUCTURED

Each day has:
- **You build** — the project/deliverable for the day
- **Concepts you learn** — what the project teaches you (just-in-time, not in advance)
- **Deliverable** — what you should have working by end of day

---

## WEEK 1 — CLI Tools & Core Rust (Days 1–7)

> Goal: Learn Rust's fundamentals by building real CLI tools. Every day ships a working binary.

### Day 1 — Build: "Hello Cargo" & Project Scaffold
- [ ] **You build:** A CLI tool that takes your name as a command-line argument and prints a formatted greeting. Then scaffold a reusable project skeleton with `fmt`, `lint`, `test` targets.
- [ ] **Concepts:** `rustup`/`rustc`/`cargo` as three separate tools · `cargo new`/`cargo check`/`cargo run`/`cargo build --release` · `Cargo.toml` vs `Cargo.lock` (lockfile = reproducibility, like `package-lock.json`) · Workspaces (`[workspace]`) overview · Editions (2015/2018/2021/2024 — we use **2024**) · `rustfmt` + `clippy` setup from minute one · `rust-toolchain.toml` for pinning versions
- [ ] **Reality check:** Every real Rust repo has a `rust-toolchain.toml` pinning the version, and CI runs `cargo fmt --check && cargo clippy -- -D warnings` before anything else
- [ ] **Deliverable:** A `hello-rust` project + a reusable project skeleton you'll clone all month

### Day 2 — Build: Multi-Unit Converter CLI
- [ ] **You build:** A CLI that converts between units (temperature, distance, weight). User picks a category, enters a value, gets the result. Uses `match` on an enum of conversion types.
- [ ] **Concepts:** `let` vs `let mut` (immutability by default — deep, not shallow like JS `const`) · Scalar types: `i32`, `f64`, `usize`, `bool`, `char` (4 bytes, full Unicode scalar) · Integer overflow behavior: panics in debug, wraps in release — use `checked_add`/`saturating_add` · Compound types: tuples, arrays `[T; N]`, slices `&[T]` · `if`/`match`/`loop`/`for` as **expressions** (everything returns a value) · Shadowing vs mutation · Enums as tagged unions (real algebraic data types, not like TS enums) · `std::io` for reading user input
- [ ] **Anti-pattern → Pattern:** Using `unwrap()` everywhere in a learning script (fine for Day 2) → returning `Result` properly (Day 5 onward)
- [ ] **Deliverable:** Working unit converter. Clippy-clean.

### Day 3 — Build: File Duplicate Finder
- [ ] **You build:** A CLI that walks a directory, groups files by size, then by content hash, and reports duplicates. This is the day ownership/borrowing clicks — because you *can't build this* without understanding them.
- [ ] **Concepts:** Stack vs heap — why Rust cares at compile time (no GC, no runtime tracing) · **Move semantics**: `String` moves, `i32` copies — you'll hit this passing file paths around · `Copy` vs `Clone`: what makes a type eligible for `Copy` (all-Copy fields, no heap pointers) · **Borrowing**: `&T` (shared/immutable) vs `&mut T` (exclusive/mutable) — **many readers OR one writer, never both** · The borrow checker as a compile-time data-race preventer · `String` vs `&str` — owned vs borrowed, the #1 "fighting the compiler" topic for JS devs · `Vec<T>` — growth strategy, `with_capacity` to avoid reallocations · `HashMap<K, V>` for grouping files · `std::fs` for directory walking, file reading
- [ ] **Reality check:** This is why Rust has no segfaults/use-after-free/double-free without `unsafe`, and why AWS, Discord, and Cloudflare rewrote hot paths in Rust — memory bugs were costing them outages and CVEs
- [ ] **Anti-pattern → Pattern:** Cloning everything to "make the borrow checker happy" (works, but O(n) extra allocs) → designing function signatures that borrow correctly the first time
- [ ] **Deliverable:** Working duplicate finder. You'll deliberately break borrow rules, read compiler errors, and fix them.

> ⚠️ **This is the hardest day.** Budget 2x time. Ownership is THE concept that makes Rust Rust. Everything else builds on it.

### Day 4 — Build: Task Tracker (In-Memory CRUD)
- [ ] **You build:** A CLI task tracker — add, list, complete, delete tasks. Tasks have name, description, status (enum: Todo/InProgress/Done), created-at timestamp. All in-memory for now.
- [ ] **Concepts:** Struct types: named-field, tuple structs, unit structs · `impl` blocks, `Self::new()` vs `&self`/`&mut self`/`self` methods · Enums with variants for task status · `Option<T>` replacing `null`/`undefined` — eliminates null pointer dereference bugs · `match` exhaustiveness — compiler forces you to handle every variant · `if let`, `while let`, `let...else` · Pattern matching on nested structures, guards (`n if n > 5`), `@` bindings · `Vec<Task>` as in-memory storage, iterator-based filtering
- [ ] **Reality check:** In a Solana program (which you've touched), account state is basically structs + enums for instruction dispatch — this is the same discipline, just without the runtime
- [ ] **Deliverable:** Working CRUD task tracker. No persistence yet — that's tomorrow.

### Day 5 — Build: Persistent Task Tracker with Error Handling
- [ ] **You build:** Take Day 4's task tracker, add JSON persistence (load from file on start, save on every change) and proper error handling. No more `unwrap()` — every failure is typed.
- [ ] **Concepts:** `Result<T, E>` as the explicit alternative to exceptions/try-catch · `?` operator — propagation, how it desugars (`From::from` on the error type) · `panic!` vs `Result` — when each is appropriate (panic = programmer bug; Result = expected failure) · Custom error enums with `thiserror` (for libraries) · `anyhow` (for applications) — and **when to use which** (real interview question) · Converting between error types, `From`/`Into` for errors · `unwrap()`, `expect()`, `unwrap_or`, `unwrap_or_else`, `unwrap_or_default` — why `unwrap()` in production is a smell · `serde` + `serde_json` for serialization/deserialization · File I/O with proper error propagation
- [ ] **Anti-pattern → Pattern:** Stringly-typed errors (`Err("something broke".to_string())`) → structured error enums callers can `match` on
- [ ] **Deliverable:** Task tracker that persists to `tasks.json`, with typed errors for file-not-found, invalid JSON, etc.

### Day 6 — Build: Text Analytics Engine
- [ ] **You build:** A CLI that reads a text file (or stdin) and produces: word frequency counts, most/least common words, average word length, sentence count, reading-level estimate. Processes large files efficiently.
- [ ] **Concepts:** **Iterators**: the `Iterator` trait, laziness — nothing runs until you call a consuming method (`.collect()`, `.sum()`, `.for_each()`) · **Closures**: `Fn`, `FnMut`, `FnOnce` — capture-by-reference vs capture-by-value vs consuming captures · Chaining: `.map()`, `.filter()`, `.fold()`, `.zip()`, `.enumerate()`, `.flat_map()`, `.take()`, `.skip()`, `.chain()` · **Zero-cost abstraction**: compare iterator chain vs hand-rolled loop in `--release` mode · `collect::<Vec<_>>()`, turbofish syntax · `HashMap` entry API (`.entry().or_insert()`) · `BTreeMap` for sorted output vs `HashMap` for speed
- [ ] **Anti-pattern → Pattern:** Index-based `for i in 0..v.len() { v[i] }` loops (bounds-checked, easy to get wrong) → iterator chains (often faster, definitely safer, definitely more idiomatic)
- [ ] **Deliverable:** Working text analyzer. Benchmark iterator vs loop version with `std::time::Instant`.

### Day 7 — 🏁 Week 1 Capstone: Polish & Ship the CLI Task Manager
- [ ] **You build:** Take Days 4-5's task tracker, add `clap` for proper CLI parsing (subcommands: add, list, complete, delete, stats), refactor into modules, add unit tests, make it portfolio-ready.
- [ ] **Concepts:** `clap` derive API for CLI argument parsing · Module system: `mod`, `pub`, `pub(crate)`, file-based modules (modern `foo.rs` + `foo/` style) · Refactoring into `models.rs`, `storage.rs`, `cli.rs`, `errors.rs` · Unit tests (`#[test]`, `assert_eq!`, `#[cfg(test)] mod tests`) · Iterator-based filtering/sorting for the `list` command · The **builder pattern** — refactor `Task::new()` into `TaskBuilder` · The **newtype pattern** — `struct TaskId(u64)` for type safety
- [ ] **Deliverable:** A clippy-clean, tested, modular CLI task manager. First portfolio piece.

**🏁 Week 1 Deliverables Summary:**
- `hello-rust` scaffold
- Multi-unit converter CLI
- File duplicate finder
- Persistent task tracker with error handling
- Text analytics engine
- **Polished CLI Task Manager** (portfolio piece #1)

---

## WEEK 2 — Libraries, Generics & Type System Mastery (Days 8–14)

> Goal: Learn to write reusable Rust libraries. Move from "Rust user" to "Rust library author."

### Day 8 — Build: Generic Stack & Queue Collection Library
- [ ] **You build:** A library crate with `Stack<T>` and `Queue<T>`, both implementing the `Iterator` trait for consumption, with push/pop/peek operations and capacity management.
- [ ] **Concepts:** Defining traits, default methods, implementing for multiple types · Trait bounds: `fn foo<T: Display>(x: T)`, `where` clauses · Deriving common traits: `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Default` · Generic structs, generic methods · Associated types (`type Item;` in `Iterator`) vs generic type params — when a trait should use which · Implementing a custom `Iterator` — internalizing associated types by writing one · Operator overloading via `std::ops` (`Add`, `Index`, etc.)
- [ ] **Reality check:** `Iterator::Item` is the textbook example of "why associated type, not generic param" — a type can only iterate one way
- [ ] **Deliverable:** A `collections` library crate with full tests and doc comments.

### Day 9 — Build: Plugin-Based Shape Calculator
- [ ] **You build:** A shape calculation system where shapes implement a `Shape` trait. Build both: (1) a generic/monomorphized version, (2) a `Vec<Box<dyn Shape>>` trait-object version. Compare them.
- [ ] **Concepts:** **Static dispatch**: generics + monomorphization (compiler generates specialized copies — zero runtime cost, larger binary) · **Dynamic dispatch**: `dyn Trait` / trait objects (vtable-based, runtime cost, but heterogeneous collections) · `impl Trait` in argument and return position — when it's sugar for generics vs necessary · `Box<dyn Trait>` for heterogeneous collections · Object safety rules — why some traits can't become `dyn Trait` · **The enum-vs-trait-object design tradeoff** — a real senior-level architecture question
- [ ] **Anti-pattern → Pattern:** `Box<dyn Trait>` everywhere "to be safe" (loses inlining, adds heap alloc) → generics when types are known at compile time, `dyn` only for genuine runtime polymorphism
- [ ] **Deliverable:** Both versions working, with a written note on when you'd choose each approach.

### Day 10 — Build: Zero-Copy Config Parser
- [ ] **You build:** A config file parser (INI-style or custom format) that returns borrowed references into the original file content instead of allocating new `String`s for every value. Measure allocations saved.
- [ ] **Concepts:** **Lifetimes**: what `'a` actually means — a description for the compiler, not a control mechanism · Lifetime elision rules — why 90% of functions need zero explicit annotations · Explicit lifetimes: `fn get_value<'a>(&'a self, key: &str) -> Option<&'a str>` · Lifetimes in structs: `struct Config<'a> { entries: Vec<(&'a str, &'a str)> }` · `'static` lifetime — what it really means · Lifetime bounds on generics: `T: 'a` · `From`/`Into`/`TryFrom`/`TryInto` for ergonomic conversions
- [ ] **Reality check:** Lifetimes show up constantly in parsers, zero-copy deserialization (`serde` with `&'a str` fields), and any API that returns borrowed data — this is how you write genuinely fast Rust
- [ ] **Anti-pattern → Pattern:** `.clone()` to dodge a lifetime error → reading the borrow-checker message and fixing the structural issue
- [ ] **Deliverable:** Zero-copy parser with benchmarks showing allocation savings vs a clone-based version.

> ⚠️ **This is the day lifetimes finally make sense** — because you're using them to solve a real performance problem (avoiding allocations), not learning them in the abstract.

### Day 11 — Build: Expression Evaluator (Mini Calculator)
- [ ] **You build:** An expression evaluator that parses and evaluates mathematical expressions like `(2 + 3) * 4 / 2`. Build an AST using recursive enums, evaluate it with pattern matching.
- [ ] **Concepts:** `Box<T>` for recursive types (`enum Expr { Add(Box<Expr>, Box<Expr>), Num(f64) }`) · `Rc<T>` — reference-counted shared ownership (single-threaded, deterministic, not a GC) · `RefCell<T>` — runtime-checked borrowing (interior mutability) · Deref coercion — why `Box<T>`, `Rc<T>` let you call `T`'s methods directly · Deep pattern matching on nested enum structures
- [ ] **Reality check:** Recursive enums + Box is how every parser, compiler, and interpreter in Rust works
- [ ] **Deliverable:** Working expression evaluator handling `+`, `-`, `*`, `/`, parentheses, with error handling for malformed input.

### Day 12 — Build: File System Tree Simulator
- [ ] **You build:** A simulated file system with directories containing files and subdirectories. Support: mkdir, touch, ls, tree (recursive display), rm. Uses `Rc<RefCell<Node>>` for shared ownership and `Weak` for parent pointers.
- [ ] **Concepts:** `Rc<RefCell<T>>` combo — the "shared mutable state, single-threaded" pattern, and its real costs (runtime panics, not compile-time safety) · `Weak<T>` — breaking reference cycles (parent/child tree structures) · When `Rc<RefCell<T>>` vs `Arc<Mutex<T>>` (single-threaded vs multi-threaded) · Tree traversal algorithms in Rust
- [ ] **Reality check:** `Rc<RefCell<T>>` is everywhere in GUI/tree/graph code, but in concurrent code you reach for `Arc<Mutex<T>>` instead (Week 3)
- [ ] **Deliverable:** Working file system simulator with tree display.

### Day 13 — Build: Comprehensive Test Suite + Documentation
- [ ] **You build:** Take all Week 2 libraries (Days 8-12), write comprehensive tests and documentation. Learn testing as a *practice*, not a topic.
- [ ] **Concepts:** Unit tests (`#[cfg(test)] mod tests`), `#[test]`, `assert!`/`assert_eq!`/`assert_ne!` · Integration tests (`tests/` directory) — testing your crate's public API like an external consumer · Doc tests — code examples in `///` comments that are compiled and run as tests · `#[should_panic]`, `Result`-returning tests · Test organization, `mod` and `pub(crate)` visibility rules · Module system deep dive: `mod.rs` vs `foo.rs` + `foo/` (modern style) · Function pointers (`fn` type) vs closures (`Fn` traits)
- [ ] **Reality check:** Doc tests are why crates.io docs show "this example actually compiles" — a real trust signal when picking a dependency
- [ ] **Deliverable:** 90%+ test coverage across all Week 2 crates, with doc comments containing working examples.

### Day 14 — 🏁 Week 2 Capstone: Generic In-Memory Cache with TTL
- [ ] **You build:** A generic, thread-safe-ready cache library with TTL expiration: `Cache<K, V>` backed by `HashMap`, with `get`, `set`, `delete`, `cleanup_expired`. Trait-based API so the backing store is swappable.
- [ ] **Concepts:** Combining everything from Week 2: generics, trait bounds, lifetimes, builder pattern · Multiple type parameters, default type parameters · Const generics for fixed-capacity variants · `PhantomData<T>` — marking unused type parameters · Feature flags (`[features]`) for optional capabilities · `Box<dyn Fn(...) -> ...>` for storing heterogeneous closures · Full unit + integration + doc tests · Published as its own crate structure
- [ ] **Deliverable:** A polished library crate — your "I can write a real Rust library" proof. Portfolio piece #2.

**🏁 Week 2 Deliverables Summary:**
- Generic Stack & Queue library
- Plugin-based Shape Calculator (static + dynamic dispatch)
- Zero-copy Config Parser (lifetimes mastery)
- Expression Evaluator (recursive types)
- File System Tree Simulator (smart pointers)
- Full test suite + documentation
- **Generic Cache Library with TTL** (portfolio piece #2)

---

## WEEK 3 — Concurrency, Async & Production Web Services (Days 15–21)

> Goal: Go from "I understand ownership" to "I can build and deploy a production async service."

### Day 15 — Build: Parallel File Word Counter
- [ ] **You build:** A CLI that counts word frequencies across many files in parallel using OS threads. Compare: (1) shared state with `Arc<Mutex<HashMap>>`, (2) thread-per-file with results joined.
- [ ] **Concepts:** `std::thread::spawn`, `JoinHandle`, `.join()` · `move` closures for thread ownership transfer · `Arc<T>` — the multi-threaded `Rc` (atomic refcounting) · `Mutex<T>` and `RwLock<T>` — exclusive vs read-heavy shared access; poisoning on panic · `Arc<Mutex<T>>` as the standard "shared mutable state across threads" pattern · Why Rust's type system makes data races a **compile-time error** (`Send`/`Sync` marker traits)
- [ ] **Reality check:** This `Send`/`Sync` guarantee is why companies trust Rust for systems where a C++ data race would cause a silent, unreproducible production bug
- [ ] **Deliverable:** Parallel word counter with timing comparison: single-threaded vs multi-threaded.

### Day 16 — Build: Multi-Stage Data Pipeline with Channels
- [ ] **You build:** A pipeline: Reader → Parser → Aggregator, connected by channels. Reader reads log files, Parser extracts structured data, Aggregator computes statistics. Compare channel-based vs mutex-based architectures.
- [ ] **Concepts:** `std::sync::mpsc` channels — multiple-producer, single-consumer · `Sender`/`Receiver`, `.send()`, `.recv()`, iterating a receiver · "Share memory by communicating" vs "communicate by sharing memory" — when each fits · Bounded vs unbounded channels — backpressure considerations
- [ ] **Reality check:** Channel-based pipelines are how you build producer/consumer/worker-pool architectures without manual locking — directly transferable to job queues, log processors, etc.
- [ ] **Deliverable:** Working pipeline processing real log files, with architecture comparison doc.

### Day 17 — Build: Async URL Health Checker
- [ ] **You build:** A CLI that takes a list of URLs, checks them all concurrently with configurable concurrency limit and timeout per request, reports status/latency for each.
- [ ] **Concepts:** **Why async exists**: I/O-bound concurrency without OS-thread-per-connection cost · `async fn`, `.await`, `Future` trait — futures are **lazy state machines**, not promises that start running immediately (critical difference from JS!) · Why you need a runtime (Tokio) — Rust's std lib ships no executor, by design · `#[tokio::main]`, `tokio::spawn`, tasks vs OS threads (green threads / cooperative scheduling) · `.await` points as the only place a task can be preempted · `reqwest` for HTTP requests · `tokio::sync::Semaphore` for concurrency limiting
- [ ] **Reality check:** "Futures don't run until polled" — in JS, once you call an async function, it starts executing synchronously up to the first await. In Rust, nothing happens until something polls the future.
- [ ] **Deliverable:** Async health checker with configurable concurrency and timeout. Table output with status codes + latency.

### Day 18 — Build: Rate-Limited Web Scraper
- [ ] **You build:** A web scraper that crawls a site (or set of sites), extracts data, respects rate limits, handles errors gracefully, outputs structured results. Handles timeouts, retries, cancellation.
- [ ] **Concepts:** `tokio::select!` — racing multiple futures, cancellation patterns · Timeouts (`tokio::time::timeout`), intervals, sleep · Native async fn in traits (check current 1.96 status) · Structured concurrency: ensuring spawned tasks don't outlive their scope · `tokio::task::spawn_blocking` for CPU-bound or blocking I/O work · **Anti-pattern → Pattern:** `std::thread::sleep` inside async (starves the runtime) → `tokio::time::sleep`
- [ ] **Anti-pattern → Pattern:** Heavy synchronous computation inside `async fn` (starves the runtime) → `spawn_blocking` or dedicated thread pool
- [ ] **Deliverable:** Working scraper with rate limiting, retry logic, and structured JSON/CSV output.

### Day 19–20 — Build: REST API with Database (2-day build)
- [ ] **You build:** A full CRUD REST API for a "bookmarks" service — save, tag, search, delete URLs with metadata. Axum + SQLite/Postgres via sqlx. Proper error-to-HTTP-status mapping, JSON request/response.

**Day 19 concepts:**
- [ ] Axum's design: built on `tokio` + `hyper` + `tower` — middleware is `tower::Service`
- [ ] Routing, handlers, extractors (`Path`, `Query`, `Json`, `State`)
- [ ] Shared application state (`Arc<AppState>`)
- [ ] Request/response JSON with `serde`
- [ ] Custom error type implementing `IntoResponse`
- [ ] Middleware via `tower-http`: CORS, compression, tracing

**Day 20 concepts:**
- [ ] `sqlx` — **compile-time checked SQL** (query errors caught at `cargo build`)
- [ ] Connection pooling (`SqlitePool` / `PgPool`)
- [ ] Migrations
- [ ] Mapping rows to structs (`FromRow` / `query_as`)
- [ ] Transactions, DB-layer error handling

- [ ] **Reality check:** This Axum + Tower + sqlx stack is the de facto standard for production Rust web services as of 2026
- [ ] **Deliverable:** Working REST API with 5+ endpoints, database persistence, proper errors.

### Day 21 — 🏁 Week 3 Capstone: Production-Ready API Deployment
- [ ] **You build:** Take Days 19-20's API, add observability, config management, graceful shutdown, Docker deployment, and a README. Make it look like a real take-home assignment.
- [ ] **Concepts:** `tracing` + `tracing-subscriber` — structured, async-aware logging/spans · Log levels, `env-filter`, structured JSON logs · Configuration: env vars, `.env` files, layered config (`config` crate or `serde` + `envy`) · Graceful shutdown (`tokio::signal`, draining in-flight requests) · `Dockerfile` with multi-stage build — static linking, `scratch`/`distroless` base images
- [ ] **Reality check:** `tracing` spans are how you correlate a single request across async task boundaries — `println!` debugging falls apart with concurrent requests interleaving
- [ ] **Deliverable:** Dockerized, instrumented, documented REST API. README with setup instructions. Portfolio piece #3.

**🏁 Week 3 Deliverables Summary:**
- Parallel File Word Counter (threads + Arc + Mutex)
- Multi-Stage Data Pipeline (channels)
- Async URL Health Checker (tokio + reqwest)
- Rate-Limited Web Scraper (select!, spawn_blocking)
- **Production REST API** with DB, tracing, Docker (portfolio piece #3)

---

## WEEK 4 — Advanced Patterns & Production Capstone (Days 22–30)

> Goal: The topics that separate "intermediate" from "exceptional." Then build something that gets you hired.

### Day 22 — Build: Safe Ring Buffer (Learning `unsafe`)
- [ ] **You build:** A fixed-size ring buffer backed by raw pointers, wrapped in a 100% safe public API. Write `// SAFETY:` comments for every `unsafe` block.
- [ ] **Concepts:** What `unsafe` actually unlocks: raw pointer deref, calling `unsafe fn`/FFI, mutable statics, union field access, implementing unsafe traits — and nothing else · Raw pointers `*const T`/`*mut T` vs references — no aliasing guarantees · Writing a safe abstraction over unsafe internals (small, justified `unsafe` blocks) · UB categories Rust protects you from: use-after-free, data races, invalid memory · FFI basics: `extern "C"`, `#[repr(C)]`
- [ ] **Reality check:** `unsafe` is not "turn off safety" — it's "I am personally proving this invariant instead of the compiler." Production crates like `bytes`, `tokio`, and `serde` all contain small, heavily-commented `unsafe` blocks surrounded by safe APIs
- [ ] **Anti-pattern → Pattern:** `unsafe` to "shut up the borrow checker" → treating borrow-checker errors as design signals, `unsafe` only as last resort with written safety proof
- [ ] **Deliverable:** Safe ring buffer with full tests proving the safe API can't trigger UB.

### Day 23 — Build: Custom `hashmap!{}` Macro + Derive Exploration
- [ ] **You build:** (1) A `macro_rules!` `hashmap!{}` literal macro. (2) A logging macro with `file!()`, `line!()` info. (3) Use `cargo expand` to demystify `#[derive(Serialize)]`.
- [ ] **Concepts:** Declarative macros (`macro_rules!`): token tree pattern matching, repetition (`$(...).*`) · When a macro is the right tool vs generic function/trait · Procedural macros overview: derive macros, attribute macros, function-like macros (conceptual) · How `serde`'s `#[derive(Serialize, Deserialize)]` works under the hood · `cargo expand`
- [ ] **Reality check:** Most app-level Rust devs never write a proc-macro from scratch, but every senior engineer can read one and understand what code it expands to
- [ ] **Deliverable:** Working macros, plus a written explanation of what `#[derive(Serialize)]` expands to for one of your structs.

### Day 24 — Build: Profile & Optimize a Hot Path
- [ ] **You build:** Take the slowest part of any prior project (duplicate finder, text analyzer, or word counter), profile it, optimize it, and document before/after numbers.
- [ ] **Concepts:** `criterion` for statistically rigorous benchmarking (not just `Instant::now()` subtraction) · `cargo flamegraph` / profiling for finding real hotspots · Allocation awareness: every `.clone()`, `.to_string()`, `Vec::push` without capacity is a potential hidden cost · `#[inline]` hints, when they matter (rarely) · Release profile tuning: `opt-level`, `lto`, `codegen-units`, `panic = "abort"` in `Cargo.toml` · SIMD awareness (high-level)
- [ ] **Reality check:** The actual senior-engineer skill is "profile first, fix the actual bottleneck" — not premature optimization
- [ ] **Deliverable:** Before/after benchmark numbers, flamegraph screenshots, written optimization writeup. Portfolio-worthy.

### Day 25 — Build: Typestate Connection Manager
- [ ] **You build:** A TCP/database connection type where invalid states are uncompilable: `Connection<Disconnected>` can call `.connect()` but not `.query()`. `Connection<Connected>` can `.query()` but not `.connect()` again.
- [ ] **Concepts:** **Typestate pattern** — encoding valid state transitions into the type system so invalid states are literally uncompilable · Sealed traits (preventing external implementations) · Marker traits, blanket implementations (`impl<T: Display> MyTrait for T`) · Trait objects with generics combined carefully (object-safety edge cases) · API design: `#[non_exhaustive]`, semantic versioning, when to expose fields vs force constructor
- [ ] **Reality check:** This is what exceptional Rust engineers reach for — e.g., a TCP connection that makes it impossible to call `.send()` before `.connect()` at compile time
- [ ] **Deliverable:** Typestate connection manager where misuse literally doesn't compile.

### Day 26 — Build: Multi-Crate Workspace
- [ ] **You build:** Restructure your Week 2 cache library + Week 3 API into a proper workspace: `core`, `cli`, `api` as separate crates sharing types.
- [ ] **Concepts:** Multi-crate workspace layout (`[workspace]` in root `Cargo.toml`) · Feature flags (`[features]`) — conditional compilation, optional dependencies · `cargo audit`, `cargo vet`/`cargo deny` — supply-chain security awareness · Semantic versioning and `Cargo.toml` version constraints (`^`, `~`, exact) · Publishing structure (even if not actually on crates.io)
- [ ] **Reality check:** This is how a real engineering org structures a Rust codebase — directly relevant to working at any company using Rust at scale
- [ ] **Deliverable:** Clean workspace with shared types, feature flags, and dependency audit.

### Days 27–29 — 🏆 FINAL CAPSTONE: Production Portfolio Project (3 days)

Choose one — discuss with me before locking in. This is your **portfolio centerpiece**.

#### Option A: Blockchain Transaction Indexer & API
- [ ] A service that connects to a blockchain (Solana or EVM via RPC), listens for transactions/events in real-time, indexes them into a database, and exposes the indexed data via REST + WebSocket API.
- **What it proves:** async I/O, concurrency, database, API design, error handling, observability
- **Why it gets you hired:** every Web3 infra company (Helius, Jito, Alchemy, Quicknode) builds exactly this. Showing you built one from scratch in Rust is a killer signal.
- **Tech:** tokio, reqwest/jsonrpc, sqlx, axum, serde, tracing, Docker

#### Option B: Smart Contract Security Scanner CLI
- [ ] A CLI that takes Solana program source/bytecode and runs static analysis checks for common vulnerability patterns: unchecked arithmetic, missing signer checks, PDA seed collisions, missing owner checks.
- **What it proves:** parsing, pattern matching, trait-based plugin architecture, CLI design, testing
- **Why it gets you hired:** directly leverages your audit background. Shows deep Rust + security + systems thinking. Unique in the market.
- **Tech:** clap, syn (for parsing), serde, trait-based rule engine, comprehensive tests

#### Option C: High-Performance API Gateway
- [ ] A reverse-proxy with configurable routing, per-client rate limiting (token bucket/sliding window), request/response transformation, health checking, and metrics.
- **What it proves:** async networking, concurrency, Tower middleware, benchmarking, systems design
- **Why it gets you hired:** pure systems-engineering signal. Relevant to any infra/platform team (Cloudflare, AWS, Fastly).
- **Tech:** tokio, hyper, tower, dashmap, serde, tracing, Docker

**All options must include:** proper error handling, tests (unit + integration), tracing/logging, README, at least one deliberate performance optimization with before/after numbers, and a short "design decisions" doc explaining your architectural choices.

> 💡 **This capstone is your portfolio piece — structure it like something you'd link from your GitHub profile next to your Solidity audit work.**

### Day 30 — Review, Gaps & Next Steps
- [ ] Full review pass across `LEARNING.md` — flag any `[!]` (shaky) topics for one more focused session
- [ ] Mock technical discussion: AI tutor asks you to explain ownership, lifetimes, `Send`/`Sync`, and async to "a curious senior engineer" out loud/in writing — teaching it back is the real test of mastery
- [ ] Plan Month 2: candidates include `tonic`/gRPC, WASM (`wasm-bindgen`), embedded (`embassy`), or going deeper into Solana/Anchor now that core Rust is solid — **do not start this without discussing it together first**

**🏁 Week 4 Deliverables Summary:**
- Safe Ring Buffer (unsafe mastery)
- Custom Macros + derive exploration
- Profiling & optimization writeup
- Typestate Connection Manager
- Multi-crate workspace
- **Production Capstone Project** (portfolio centerpiece)

---

## 🧰 TOOLING & ECOSYSTEM REFERENCE (verified June 2026)

| Category | Crate/Tool | Why |
|---|---|---|
| Async runtime | `tokio` | The de facto standard async runtime |
| Web framework | `axum` | Tower-based, type-safe extractors, the current standard choice |
| HTTP client | `reqwest` | Built on hyper, ergonomic async client |
| Serialization | `serde` + `serde_json` | The universal (de)serialization framework |
| Error handling (lib) | `thiserror` | Structured, derivable error enums for library APIs |
| Error handling (app) | `anyhow` | Convenient `Result<T>` + context chaining for applications |
| CLI parsing | `clap` (derive API) | The standard CLI argument parser |
| Database | `sqlx` | Compile-time checked SQL, async, Postgres/SQLite/MySQL |
| Logging/tracing | `tracing` + `tracing-subscriber` | Structured, span-based, async-aware observability |
| Benchmarking | `criterion` | Statistically rigorous micro-benchmarking |
| Testing helpers | `rstest`, `proptest` | Parameterized tests, property-based testing |
| Linting | `clippy` (built-in) | Non-negotiable, run it constantly |
| Formatting | `rustfmt` (built-in) | Non-negotiable |
| Supply chain | `cargo-audit`, `cargo-deny` | Dependency vulnerability/license auditing |

> ⚠️ Always verify exact current major versions of these crates yourself (e.g. via `cargo add <crate>` which fetches the latest, or crates.io) before pinning in a real project. Rust stable is at **1.96.0** as of June 25, 2026.

---

## ✅ Approval Log for This File
*(Don't edit the roadmap above without logging the change in `LOGS.md` — see that file's format.)*
- 2026-06-25 — Initial roadmap created and approved by learner.
- 2026-06-26 — Full restructure from topic-based to project-based curriculum. Approved by learner.
