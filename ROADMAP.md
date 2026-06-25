# 🦀 RUST MASTERY ROADMAP — 30-Day Intensive

> **Created:** 2026-06-25 · **Target completion:** 2026-07-25 · **Pace:** 5+ hrs/day
> **Learner profile:** Full-stack (MERN) engineer, Web3/Solidity security background, prior light exposure to Rust via Solana/Anchor. Goal: become a production-grade Rust engineer capable of using Rust as core backend/systems architecture.
> **Rust edition target:** 2024 Edition · **Toolchain:** stable (currently ~1.95.x as of this writing — always run `rustc --version` and treat that as ground truth, not this number)

---

## 🔒 GOVERNANCE RULES (read before touching any file)

1. **No silent edits.** Neither `ROADMAP.md` nor `LEARNING.md` is ever modified by the AI without explicitly asking the learner first and getting a yes. This applies to checking off a topic, reordering, rewording, adding, or deleting anything.
2. **Every change is logged.** Any edit to `ROADMAP.md` or `LEARNING.md` — once approved — gets a corresponding collapsible entry in `LOGS.md` (see that file's format). No exceptions, no batching silently.
3. **`LEARNING.md` is the source of truth for progress.** `ROADMAP.md` is the static curriculum (it can evolve, but slowly and only with consent). `LEARNING.md` is the living journal of what's actually been learned, built, and understood.
4. **The roadmap cross-checks the learning log.** Before starting a new topic, the AI should glance at `LEARNING.md` to see what's marked done, what's marked shaky/needs-review, and adapt pacing — but it still asks before changing the roadmap itself.
5. **Status markers used consistently across both files:**
   - `[ ]` Not started
   - `[~]` In progress
   - `[x]` Completed & understood
   - `[!]` Completed but shaky / needs revisit
6. **One topic, one conversation thread of depth.** Don't dump 10 topics at once. Teach one concept fully — explanation, real-world "why", code example, optimized vs naive version, then a small exercise — before moving on, unless the learner explicitly asks to move faster.

---

## 📐 HOW THIS ROADMAP IS STRUCTURED

Each topic below has four tags where relevant:
- **Concept** — what it is
- **Reality check** — where this actually shows up in production code / why it exists
- **Anti-pattern → Pattern** — the naive way vs. the idiomatic/optimized way
- **Project hook** — which project (see Week-by-week projects) will force you to use it

---

## WEEK 1 — Foundations That Are Actually Hard (Ownership, Types, Error Handling)

> Goal: stop translating from JS/TS in your head. Start thinking in ownership.

### Day 1 — Environment, Toolchain, and Cargo Mental Model
- [ ] Install via `rustup`, understand `rustup`, `rustc`, `cargo` as three separate tools
- [ ] `cargo new`, `cargo run`, `cargo build --release`, `cargo check` (and why `cargo check` is your 90%-of-the-time command — it skips codegen)
- [ ] `Cargo.toml` vs `Cargo.lock` — what gets committed and why (lockfile = reproducibility, like `package-lock.json`)
- [ ] Workspaces (`[workspace]`) — multi-crate projects, analogous to a monorepo with shared `node_modules` but with real dependency isolation
- [ ] Editions: 2015/2018/2021/2024 — we build everything on **2024**
- [ ] `rustfmt` and `clippy` — set these up now, not later. Clippy is not optional; it's your senior engineer's code review running locally
- [ ] **Reality check**: every real Rust repo has a `rust-toolchain.toml` pinning the version, and CI runs `cargo fmt --check && cargo clippy -- -D warnings` before anything else
- [ ] Project hook: scaffold `hello-rust` + a `justfile`/`Makefile` with `fmt`, `lint`, `test` targets (you'll reuse this skeleton all month)

### Day 2 — Variables, Types, and Control Flow (fast pass — you know this shape from TS)
- [ ] `let` vs `let mut` — immutability by default (contrast: `const` in JS is shallow; Rust's is deep unless `Cell`/`RefCell`)
- [ ] Scalar types: `i8..i128`, `u8..u128`, `isize`/`usize`, `f32`/`f64`, `bool`, `char` (4 bytes, full Unicode scalar — not like JS's UTF-16 code units)
- [ ] Integer overflow behavior: panics in debug, wraps in release (and why you should never rely on that — use `checked_add`, `wrapping_add`, `saturating_add` explicitly)
- [ ] Compound types: tuples, arrays `[T; N]` (fixed size, stack), slices `&[T]` (the real workhorse)
- [ ] `if`/`match`/`loop`/`while`/`for` as expressions, not statements — everything returns a value
- [ ] Shadowing vs mutation — when to use which
- [ ] **Anti-pattern → Pattern**: using `unwrap()` everywhere in a learning script (fine for Day 2) → returning `Result` properly (Day 4 onward)
- [ ] Project hook: temperature converter / simple calculator CLI using `match` on an enum of operations

### Day 3 — Ownership, Move Semantics, Borrowing (THE topic — budget 2x time here)
- [ ] Stack vs heap, and why Rust cares about this at compile time (no GC, no runtime tracing)
- [ ] Move semantics: assignment moves by default for non-`Copy` types — `String` moves, `i32` copies
- [ ] `Copy` vs `Clone` — what makes a type eligible for `Copy` (all-Copy fields, no heap pointers)
- [ ] Borrowing: `&T` (shared/immutable) vs `&mut T` (exclusive/mutable) — the core invariant: **many readers OR one writer, never both**
- [ ] The borrow checker as a compile-time data-race preventer — this is literally what replaces your mental model of "don't mutate shared state" that you've been doing manually in JS/Solidity
- [ ] Dangling references are impossible — compare to a JS closure capturing a stale ref, or a Solidity storage pointer to deleted data
- [ ] **Reality check**: this is why Rust has no segfaults/use-after-free/double-free *without* `unsafe`, and why companies like AWS, Discord, and Cloudflare rewrote hot paths in Rust — memory bugs were costing them outages and CVEs
- [ ] **Anti-pattern → Pattern**: cloning everything to "make the borrow checker happy" (works, but is O(n) extra allocs) → designing function signatures that borrow correctly the first time
- [ ] Project hook: a small "inventory" struct system where you deliberately try to break borrow rules, read the compiler errors, and fix them — this is the single best exercise in the entire roadmap

### Day 4 — Structs, Enums, Pattern Matching, and `Option`
- [ ] Struct types: named-field, tuple structs, unit structs
- [ ] `impl` blocks, associated functions (`Self::new`) vs methods (`&self`/`&mut self`/`self`)
- [ ] Enums as **tagged unions** (not like TS enums — much closer to a Rust enum being a real algebraic data type / like a Solidity-style discriminated struct but compiler-enforced)
- [ ] `Option<T>` replacing `null`/`undefined` — and why this eliminates an entire CVE category (null pointer dereference / billion-dollar mistake)
- [ ] `match` exhaustiveness — compiler forces you to handle every variant; no more forgetting an `else if`
- [ ] `if let`, `while let`, `let...else`
- [ ] Pattern matching on nested structures, guards (`match x { n if n > 5 => ... }`), `@` bindings
- [ ] **Reality check**: in a Solana program (which you've touched), account state is basically structs + enums for instruction dispatch — this is the same discipline, just without the runtime
- [ ] Project hook: build a `TrafficLight`/state-machine enum, then a small **task tracker** (CRUD on a `Vec<Task>` in memory) using structs + enums for status

### Day 5 — `Result`, Error Handling, and the `?` Operator
- [ ] `Result<T, E>` as the explicit alternative to exceptions/try-catch
- [ ] `?` operator — propagation, and how it desugars (calls `From::from` on the error type)
- [ ] `panic!` vs `Result` — when each is appropriate (panic = programmer bug / unrecoverable; Result = expected failure mode)
- [ ] Custom error enums with `thiserror` (for libraries — defines a precise error API)
- [ ] `anyhow` (for applications — convenient `Result<T>` + `.context()` chains) — and **when to use which** (this is a real, common interview question)
- [ ] Converting between error types, `From`/`Into` for errors
- [ ] `unwrap()`, `expect()`, `unwrap_or`, `unwrap_or_else`, `unwrap_or_default` — and why `unwrap()` in production code outside of tests/prototypes is a smell
- [ ] **Anti-pattern → Pattern**: stringly-typed errors (`Err("something broke".to_string())`) → structured error enums callers can `match` on
- [ ] Project hook: rebuild Day 4's task tracker with **real error handling** — file-not-found, invalid input, etc. all become typed errors, not panics

### Day 6 — Collections Deep Dive
- [ ] `Vec<T>` — growth strategy (capacity doubling), `with_capacity` to avoid reallocations (this is your first real "optimized vs naive" lesson)
- [ ] `String` vs `&str` — owned heap-allocated UTF-8 vs borrowed string slice; **why this distinction is the #1 source of "fighting the compiler" for JS devs**
- [ ] `HashMap<K, V>` vs `BTreeMap<K, V>` — hashing cost vs ordered iteration, when you need which
- [ ] `HashSet`, `BTreeSet`
- [ ] `VecDeque` — when you need a queue/ring buffer (BFS, sliding window)
- [ ] Iterators over collections: `.iter()` vs `.iter_mut()` vs `.into_iter()` — borrow vs mutate vs consume
- [ ] **Reality check**: choosing `HashMap` over `BTreeMap` by default is the right call 95% of the time — ordered iteration is a real cost, not a free nicety
- [ ] Project hook: word-frequency counter on a large text file — measure `HashMap` vs naive `Vec<(String, u32)>` linear scan performance difference yourself

### Day 7 — Iterators, Closures, and Functional Patterns (Rust's biggest ergonomic weapon)
- [ ] Closures: `Fn`, `FnMut`, `FnOnce` — what each means for capture-by-reference vs capture-by-value vs consuming captures
- [ ] The `Iterator` trait — laziness: nothing runs until you call a consuming method (`.collect()`, `.sum()`, `.for_each()`, etc.)
- [ ] Chaining: `.map()`, `.filter()`, `.fold()`, `.zip()`, `.enumerate()`, `.take()`, `.skip()`, `.chain()`, `.flat_map()`
- [ ] Zero-cost abstraction claim — **prove it yourself**: compare a hand-rolled loop vs an iterator chain in `--release` mode and look at near-identical assembly/benchmark timing
- [ ] `collect::<Vec<_>>()`, turbofish syntax explained
- [ ] **Anti-pattern → Pattern**: writing index-based `for i in 0..v.len() { v[i] }` loops (bounds-checked, easy to get wrong) → iterator chains (often faster, definitely safer, definitely more idiomatic)
- [ ] Project hook: refactor every loop from Days 2–6's projects into iterator chains; benchmark with `criterion` (mini intro) to see the zero-cost claim hold up

**🏁 Week 1 Capstone Project: CLI Task Manager v2**
- Persists tasks to a JSON file (`serde` + `serde_json`)
- Full error handling with custom error enum
- Uses `clap` for argument parsing (subcommands: add, list, complete, delete)
- Idiomatic iterator-based filtering/sorting
- `cargo clippy` clean, `cargo fmt` applied, basic unit tests

---

## WEEK 2 — Type System Mastery (Traits, Generics, Lifetimes) + Testing

> Goal: this week is what separates "can write Rust" from "writes Rust like it's meant to be written."

### Day 8 — Traits I: Shared Behavior
- [ ] Defining traits, default methods, implementing for multiple types
- [ ] Trait bounds on generic functions: `fn foo<T: Display>(x: T)`
- [ ] `where` clauses for readability when bounds get complex
- [ ] Deriving common traits: `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Default`, `PartialOrd`, `Ord`
- [ ] Operator overloading via `std::ops` traits (`Add`, `Sub`, `Index`, etc.)
- [ ] **Reality check**: traits are how Rust does polymorphism *without* a class hierarchy — closer to TypeScript interfaces or Go interfaces than Java/C# inheritance, but compiler-checked everywhere
- [ ] Project hook: build a small `Shape` trait (Circle, Rectangle, Triangle) with `area()`, `perimeter()` — classic but you'll extend it Day 9

### Day 9 — Traits II: Static vs Dynamic Dispatch
- [ ] Generics + trait bounds = **monomorphization** (compiler generates a specialized copy per concrete type — zero runtime cost, but larger binary)
- [ ] `dyn Trait` = **trait objects** (vtable-based dynamic dispatch — runtime cost of a pointer indirection, but one binary, heterogeneous collections)
- [ ] `impl Trait` in argument and return position — when it's sugar for generics vs when it's necessary (returning unnameable closure/iterator types)
- [ ] `Box<dyn Trait>` for heterogeneous collections (`Vec<Box<dyn Shape>>`)
- [ ] Object safety rules — why some traits can't become `dyn Trait`
- [ ] **Anti-pattern → Pattern**: reaching for `Box<dyn Trait>` everywhere "to be safe" (loses compiler's ability to inline/optimize, adds heap allocation) → using generics + bounds when the concrete type is known at compile time, `dyn` only when you genuinely need runtime polymorphism (e.g., plugin systems, heterogeneous lists)
- [ ] Project hook: extend `Shape` into a `Vec<Box<dyn Shape>>` "shape collection" that computes total area — then write the *same* thing with an enum instead and discuss tradeoffs (this enum-vs-trait-object tradeoff is a real senior-level design question)

### Day 10 — Generics Deep Dive + Associated Types
- [ ] Generic structs, enums, methods — `struct Wrapper<T> { value: T }`
- [ ] Multiple type parameters, default type parameters
- [ ] Associated types (`type Item;` in `Iterator`) vs generic type parameters — when a trait should use which (one output type per impl vs many possible)
- [ ] `PhantomData<T>` — marking unused type parameters (you'll see this constantly in library code)
- [ ] Const generics: `[T; N]` and `struct Matrix<const N: usize>` — compile-time-sized arrays without heap allocation
- [ ] **Reality check**: `Iterator::Item` is the textbook example of "why associated type, not generic param" — a type can only iterate one way, so one output type per implementation makes sense
- [ ] Project hook: implement a custom `Iterator` for a `Fibonacci` struct and for a simple `Stack<T>` — internalize associated types by writing one

### Day 11 — Lifetimes (the part everyone fears — demystified properly)
- [ ] What a lifetime annotation actually is: **a description for the compiler, not a control mechanism** — you're not making references live longer, you're proving to the compiler how long they already live
- [ ] Lifetime elision rules — why 90% of functions need zero explicit lifetime annotations
- [ ] Explicit lifetimes: `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str` — walk through *exactly* why the compiler needs this
- [ ] Lifetimes in structs holding references — `struct Excerpt<'a> { part: &'a str }`
- [ ] `'static` lifetime — what it really means (lives for the whole program; string literals, not "global" in the loose sense)
- [ ] Lifetime bounds on generics: `T: 'a`
- [ ] **Reality check**: lifetimes show up constantly in parsers, zero-copy deserialization (`serde` with `&'a str` fields), and any API that returns borrowed data instead of cloning — this is how you write genuinely fast Rust instead of clone-happy Rust
- [ ] **Anti-pattern → Pattern**: sprinkling `.clone()` to dodge a lifetime error you don't understand → reading the actual borrow-checker message, understanding which reference outlives which scope, and fixing the real structural issue
- [ ] Project hook: write a zero-copy CSV-line parser that returns `Vec<&str>` slices into the original line instead of allocating new `String`s — measure allocations saved

### Day 12 — Smart Pointers & Interior Mutability
- [ ] `Box<T>` — heap allocation, recursive types (`enum List { Cons(i32, Box<List>), Nil }`)
- [ ] `Rc<T>` — reference-counted shared ownership (single-threaded) — and why this is *not* a GC, it's deterministic refcounting
- [ ] `RefCell<T>` — runtime-checked borrowing when the compiler can't prove safety statically (interior mutability pattern)
- [ ] `Rc<RefCell<T>>` combo — the classic "shared mutable state, single-threaded" pattern, and its real costs (runtime panics on borrow violations, not compile-time safety)
- [ ] `Weak<T>` — breaking reference cycles (parent/child tree structures)
- [ ] Deref coercion — why `Box<T>`, `Rc<T>` etc. let you call `T`'s methods directly
- [ ] **Reality check**: `Rc<RefCell<T>>` is everywhere in GUI/tree/graph code, but in *concurrent* code you reach for `Arc<Mutex<T>>` instead (Week 3) — knowing which "shared state wrapper" applies to which concurrency model is a real skill gap most people have
- [ ] Project hook: build a simple doubly-linked-list-free **tree** structure (e.g., a file-system simulation) using `Rc<RefCell<Node>>`, including a parent pointer via `Weak`

### Day 13 — Testing, Documentation, and Project Organization
- [ ] Unit tests (`#[cfg(test)] mod tests`), `#[test]`, `assert!`/`assert_eq!`/`assert_ne!`
- [ ] Integration tests (`tests/` directory) — testing your crate's public API like an external consumer
- [ ] Doc tests — code examples in `///` comments that are actually compiled and run as tests (this is unique and excellent — your docs can't go stale and lie)
- [ ] `#[should_panic]`, `Result`-returning tests
- [ ] Test organization for larger projects, `mod` and `pub(crate)` visibility rules
- [ ] Module system deep dive: `mod`, `pub`, `pub(crate)`, `pub(super)`, file-based modules (`mod.rs` vs `foo.rs` + `foo/` directory — modern style)
- [ ] **Reality check**: doc tests are why crates.io documentation pages can show "this example actually compiles" — a real trust signal when picking a dependency
- [ ] Project hook: take Week 1's task manager, split it into proper modules (`models`, `storage`, `cli`, `errors`), write unit + integration tests, write doc comments with working examples

### Day 14 — Closures Advanced, Function Pointers, and Builder Patterns
- [ ] `Box<dyn Fn(...)  -> ...>` for storing heterogeneous closures
- [ ] Function pointers (`fn` type) vs closures (`Fn` traits) — when a plain `fn` pointer suffices
- [ ] The builder pattern in Rust (no method overloading, no default args — builder is the idiomatic answer)
- [ ] The newtype pattern — wrapping a primitive in a struct for type safety (`struct UserId(u64)` instead of passing raw `u64` everywhere — sound familiar from strict TS code)
- [ ] `From`/`Into`/`TryFrom`/`TryInto` for ergonomic conversions between your own types
- [ ] **Reality check**: nearly every well-designed Rust HTTP client/config struct (e.g., `reqwest::ClientBuilder`) uses the builder pattern because Rust has no optional/default function parameters
- [ ] Project hook: refactor task manager's `Task::new(...)` into a `TaskBuilder`; add newtypes for `TaskId`

**🏁 Week 2 Capstone Project: Generic In-Memory Key-Value Store Library**
- Generic over key/value types with trait bounds
- Public trait-based API (so it's swappable: `HashMap`-backed vs `BTreeMap`-backed implementation)
- Proper error enum with `thiserror`
- Full unit + integration + doc tests
- Published as its own small crate structure (even if not actually published to crates.io) — this is your "I can write a real library" proof

---

## WEEK 3 — Concurrency, Async, and Building Real Services

> Goal: go from "I understand ownership" to "I can build a production async service."

### Day 15 — Concurrency Fundamentals: Threads
- [ ] `std::thread::spawn`, `JoinHandle`, `.join()`
- [ ] `move` closures for thread ownership transfer
- [ ] `Arc<T>` — the multi-threaded version of `Rc<T>` (atomic refcounting)
- [ ] `Mutex<T>` and `RwLock<T>` — exclusive vs read-heavy shared access; poisoning on panic and what that means
- [ ] `Arc<Mutex<T>>` as the standard "shared mutable state across threads" pattern
- [ ] Why Rust's type system makes data races a **compile-time error** (`Send`/`Sync` marker traits) — this is the single biggest "wait, that's actually incredible" moment for most learners
- [ ] **Reality check**: this is the guarantee that makes companies trust Rust for systems where a C++ data race would cause a silent, unreproducible production bug
- [ ] Project hook: parallel word-counter across multiple files using a thread pool, sharing results via `Arc<Mutex<HashMap<...>>>`, then compare against a channel-based version (Day 16)

### Day 16 — Channels and Message Passing
- [ ] `std::sync::mpsc` — multiple-producer single-consumer channels
- [ ] `Sender`/`Receiver`, `.send()`, `.recv()`, iterating a receiver
- [ ] "Share memory by communicating" philosophy vs "communicate by sharing memory" (`Arc<Mutex<T>>`) — knowing when each fits is a real architecture decision
- [ ] Bounded vs unbounded channels — backpressure considerations
- [ ] **Reality check**: channel-based pipelines are how you build producer/consumer/worker-pool architectures without manual locking — directly transferable to job queues, log processors, etc.
- [ ] Project hook: build a multi-stage pipeline (read files → parse → aggregate) using channels between stages, compare ergonomics/performance against Day 15's mutex version

### Day 17 — Async Rust I: The Mental Model
- [ ] Why async exists: I/O-bound concurrency without OS-thread-per-connection cost
- [ ] `async fn`, `.await`, `Future` trait — futures are **lazy state machines**, not promises that start running immediately (critical difference from JS promises!)
- [ ] Why you need a runtime (Tokio) — Rust's std lib ships no executor, by design
- [ ] `#[tokio::main]`, `tokio::spawn`, tasks vs OS threads (green threads / cooperative scheduling)
- [ ] `.await` points as the only place a task can be preempted — implications for holding a `Mutex` across an `.await` (classic bug source)
- [ ] **Reality check**: this "futures don't run until polled/awaited" model is exactly the kind of thing that trips up JS developers, because in JS, once you call an async function, it *starts* executing synchronously up to the first await — in Rust, nothing happens until something polls the future
- [ ] Project hook: convert Day 16's pipeline to async, fetch multiple URLs concurrently with `reqwest` + `tokio::join!`/`futures::future::join_all`

### Day 18 — Async Rust II: Practical Patterns
- [ ] `tokio::select!` — racing multiple futures, cancellation patterns
- [ ] Timeouts (`tokio::time::timeout`), intervals, sleep
- [ ] `async-trait` crate vs native async fn in traits (note: native support has matured — check current state, since this was a long-standing pain point that's been evolving release to release)
- [ ] Structured concurrency basics: ensuring spawned tasks don't outlive their intended scope, cancellation safety
- [ ] Common pitfalls: blocking the async runtime with sync code (`std::thread::sleep` instead of `tokio::time::sleep`, CPU-heavy work without `spawn_blocking`)
- [ ] `tokio::task::spawn_blocking` for CPU-bound or blocking I/O work inside an async context
- [ ] **Anti-pattern → Pattern**: doing heavy synchronous computation directly inside an `async fn` (starves the runtime, stalls every other task on that worker thread) → `spawn_blocking` or a dedicated thread pool
- [ ] Project hook: build a concurrent web scraper/health-checker that pings N URLs with a timeout and concurrency limit (`tokio::sync::Semaphore`)

### Day 19 — Building a Real Web Service with Axum
- [ ] Axum's design: built on `tokio` + `hyper` + `tower` — middleware is `tower::Service`, not a bespoke system
- [ ] Routing, handlers, extractors (`Path`, `Query`, `Json`, `State`)
- [ ] Shared application state (`Arc<AppState>`) — connecting back to Day 15's concurrency primitives in a real context
- [ ] Request/response JSON with `serde`
- [ ] Custom error type implementing `IntoResponse` — proper error-to-HTTP-status mapping (ties Day 5's error handling directly into a real API)
- [ ] Middleware via `tower-http`: tracing, CORS, compression, timeouts
- [ ] **Reality check**: this Axum + Tower stack pattern is the de facto standard for production Rust web services as of 2026 — what you build here transfers directly to real job-ready backend work
- [ ] Project hook: start the **Week 3 capstone** — a REST API

### Day 20 — Persistence: Databases with `sqlx`
- [ ] `sqlx` — compile-time checked SQL queries against Postgres/SQLite (a genuinely Rust-unique feature: query syntax errors caught at `cargo build` time)
- [ ] Connection pooling (`PgPool`)
- [ ] Migrations
- [ ] Mapping rows to structs (`FromRow` / `query_as`)
- [ ] Transactions, error handling on the DB layer (tying back into your `thiserror` custom error type from Day 19)
- [ ] **Reality check**: compile-time query checking is the kind of thing that makes experienced backend engineers from any language go "wait, why doesn't every language do this"
- [ ] Project hook: wire persistence into the Week 3 capstone API (replace in-memory state with Postgres or SQLite)

### Day 21 — Observability, Logging, Config
- [ ] `tracing` + `tracing-subscriber` — structured, async-aware logging/spans (replaces naive `println!` debugging entirely)
- [ ] Log levels, `env-filter` for runtime-configurable verbosity
- [ ] Configuration management: env vars, `.env` files, layered config (`config` crate or hand-rolled with `serde` + `envy`)
- [ ] Graceful shutdown (`tokio::signal`, draining in-flight requests)
- [ ] **Reality check**: `tracing` spans are how you correlate a single request across async task boundaries — `println!` debugging falls apart the moment you have concurrent requests interleaving output
- [ ] Project hook: instrument the capstone API with tracing spans per request, structured JSON logs, graceful shutdown on SIGINT

**🏁 Week 3 Capstone Project: Production-Shaped REST API**
- Axum + Tower + sqlx (Postgres or SQLite)
- Full CRUD resource (e.g., a "notes" or "links" service) with auth-ready structure (even a simple API-key middleware)
- Structured errors → correct HTTP status codes
- `tracing` instrumentation, env-based config, graceful shutdown
- Dockerized (`Dockerfile` with multi-stage build — small image, this is itself a real optimization lesson: static vs dynamic linking, `distroless`/`scratch` base images)
- README documenting how to run it — treat this like a real take-home assignment you'd submit for a job

---

## WEEK 4 — Advanced Rust: `unsafe`, Performance, Macros, and Capstone

> Goal: the topics that separate "intermediate" from "exceptional" — what makes a senior Rust engineer dangerous-good.

### Day 22 — `unsafe` Rust, Properly Understood
- [ ] What `unsafe` actually unlocks: raw pointer deref, calling `unsafe fn`/FFI, mutable statics, union field access, implementing unsafe traits — **and nothing else**. It does NOT disable the borrow checker generally.
- [ ] Raw pointers `*const T`/`*mut T` vs references — no aliasing guarantees, no automatic validity
- [ ] Writing a safe abstraction over unsafe internals (the actual discipline: `unsafe` blocks should be small, justified with a `// SAFETY:` comment explaining *why* the invariant holds)
- [ ] Undefined behavior categories Rust protects you from normally: use-after-free, data races, invalid memory access, etc — and how `unsafe` reopens those doors if misused
- [ ] FFI basics: `extern "C"`, calling into a C library, `#[repr(C)]`
- [ ] **Reality check**: `unsafe` is not "turn off safety," it's "I am personally proving this invariant instead of the compiler" — production crates like `bytes`, `tokio`, and `serde` all contain small, heavily-commented `unsafe` blocks at their performance-critical core, surrounded by a 100%-safe public API
- [ ] **Anti-pattern → Pattern**: reaching for `unsafe` to "make the borrow checker stop complaining" → treating every borrow-checker rejection as a design signal first, `unsafe` only as a last resort with a written safety proof
- [ ] Project hook: implement a tiny safe wrapper around a raw-pointer-based fixed-size ring buffer; write the `// SAFETY:` justification comments yourself

### Day 23 — Macros: `macro_rules!` and an Intro to Derive Macros
- [ ] Declarative macros (`macro_rules!`) — pattern matching on token trees, repetition (`$(...),*`)
- [ ] When a macro is the right tool vs when a generic function/trait would do (macros operate on syntax, not types — use them when you need to generate code shape, not just behavior)
- [ ] Procedural macros overview: derive macros (`#[derive(MyTrait)]`), attribute macros, function-like macros — at least conceptually, even if you don't write a full proc-macro crate this month
- [ ] How `serde`'s `#[derive(Serialize, Deserialize)]` works under the hood (conceptually) — demystify the "magic"
- [ ] **Reality check**: most app-level Rust developers never write a proc-macro from scratch, but every senior engineer can read one and understand what code it expands to (`cargo expand` is the tool for this)
- [ ] Project hook: write a small `macro_rules!` that generates boilerplate (e.g., a `hashmap!{}` literal macro, or a logging macro with file/line info)

### Day 24 — Performance: Profiling and Optimization Mindset
- [ ] Benchmarking properly with `criterion` (statistical rigor — not just `Instant::now()` subtraction, which lies due to noise/warmup/branch prediction)
- [ ] `cargo flamegraph` / `perf` basics for finding real hotspots — **measure before optimizing, always**
- [ ] Allocation awareness: every `.clone()`, `.to_string()`, `Vec::push` without capacity is a potential hidden cost — `cargo-flamegraph`/`dhat`/`heaptrack` style thinking
- [ ] `#[inline]` hints, when they matter (rarely — trust the compiler first)
- [ ] Release profile tuning: `opt-level`, `lto`, `codegen-units`, `panic = "abort"` tradeoffs in `Cargo.toml`
- [ ] SIMD awareness (high-level — know it exists via `std::simd` or `packed_simd`-style crates, know when autovectorization already handles it)
- [ ] **Reality check**: the actual senior-engineer skill here isn't "know every micro-optimization," it's "profile first, fix the actual bottleneck, and know which 5% of code is hot" — this is true in every language, but Rust gives you the tools to act on it with zero-cost abstractions
- [ ] Project hook: take the slowest part of any prior project, profile it, optimize it, and document the before/after numbers — this becomes a portfolio-worthy writeup

### Day 25 — Advanced Trait Patterns & API Design
- [ ] Sealed traits (preventing external implementations of a trait — a real API-design technique)
- [ ] Marker traits, blanket implementations (`impl<T: Display> MyTrait for T`)
- [ ] Trait objects with generics combined carefully (object-safety edge cases)
- [ ] The "typestate" pattern — encoding valid state transitions into the type system so invalid states are literally uncompilable (e.g., a `Connection<Disconnected>` vs `Connection<Connected>`)
- [ ] API design principles: `#[non_exhaustive]`, semantic versioning discipline, when to expose a struct's fields vs force constructor use
- [ ] **Reality check**: the typestate pattern is what "exceptional" Rust engineers reach for in real systems — e.g., a TCP connection type that makes it impossible to call `.send()` before `.connect()` *at compile time*
- [ ] Project hook: redesign one of your earlier projects (e.g., the task manager or KV store) using the typestate pattern for at least one workflow

### Day 26 — Workspaces, Crate Publishing, and Dependency Hygiene
- [ ] Multi-crate workspace layout for a real project (`core`, `cli`, `api` as separate crates sharing one workspace)
- [ ] Feature flags (`[features]`) — conditional compilation, optional dependencies
- [ ] `cargo audit`, `cargo vet`/`cargo deny` — supply-chain security awareness (genuinely important given recent crates.io supply-chain incidents)
- [ ] Semantic versioning and `Cargo.toml` version constraints (`^`, `~`, exact)
- [ ] (Conceptual) publishing to crates.io — README, `LICENSE`, `docs.rs` generation, what a good crate's metadata looks like
- [ ] **Reality check**: this is the "how a real engineering org structures a Rust codebase" day — directly relevant to working at any company using Rust at scale
- [ ] Project hook: restructure your Week 2 KV-store library + Week 3 API into a proper workspace sharing common types

### Day 27–29 — FINAL CAPSTONE PROJECT (3 days, integrative)
Choose (or combine) based on interest — discuss with the AI tutor before locking in:
- [ ] **Option A**: A multi-threaded/async job-queue system (producer/consumer, persistence, retry logic, observability) — heavy concurrency + async focus
- [ ] **Option B**: A full REST+WebSocket API service (e.g., a real-time chat or notification backend) with auth, persistence, tests, Docker, CI — heavy "production service" focus
- [ ] **Option C**: A CLI dev-tool (e.g., a linter, a log-parser, a code-generator) with plugin-style trait architecture — heavy "systems tool" focus, good bridge toward future Solana/CLI tooling work given your background
- [ ] Must include: proper error handling, tests (unit+integration), tracing/logging, README, at least one deliberate performance optimization you can explain with before/after numbers, and a short "design decisions" doc explaining *why* you chose the patterns you did
- [ ] **Reality check**: this capstone is your portfolio piece — structure it like something you'd link from your GitHub profile next to your Solidity audit work

### Day 30 — Review, Gaps, and What's Next
- [ ] Full review pass across `LEARNING.md` — flag any `[!]` (shaky) topics for one more focused session
- [ ] Mock technical discussion: AI tutor asks you to explain ownership, lifetimes, `Send`/`Sync`, and async to "a curious senior engineer" out loud/in writing — teaching it back is the real test of mastery
- [ ] Plan the next month: candidates include `tonic`/gRPC, WASM (`wasm-bindgen`), embedded (`embassy`), or going deeper into Solana/Anchor now that core Rust is solid (directly relevant given your Web3 background) — **do not start this without discussing it together first**

---

## 🧰 TOOLING & ECOSYSTEM REFERENCE (current as of June 2026 — verify versions yourself before pinning)

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

> ⚠️ Always verify exact current major versions of these crates yourself (e.g. via `cargo add <crate>` which fetches the latest, or crates.io) before pinning in a real project — this table intentionally omits version numbers since they will drift after this roadmap is written.

---

## ✅ Approval Log for This File
*(Don't edit the roadmap above without logging the change in `LOGS.md` — see that file's format.)*
- 2026-06-25 — Initial roadmap created and approved by learner.
