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
| Week 1 | CLI Tools & Core Rust | `[ ]` Not started |
| Week 2 | Libraries, Generics & Type System Mastery | `[ ]` Not started |
| Week 3 | Concurrency, Async & Production Web Services | `[ ]` Not started |
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

---

<!-- New day entries get appended below this line. Ask the AI to draft an entry at the end of each session; approve or edit before it's saved. -->

---

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
| CLI Task Manager (polished) | 1 | `[ ]` | — | Week 1 capstone |
| Generic Cache Library with TTL | 2 | `[ ]` | — | Week 2 capstone |
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
