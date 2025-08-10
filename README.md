<!-- # rustqlite -->
<!---->
<!-- Steps I am taking to learn/build: -->
<!---->
<!-- 1) Completed the rust book and the rustlings excercises for a basic understanding of rust. -->
<!---->
<!-- 2) Discussed with ChatGPT about the high level structure and architecture of the project. -->
<!---->
<!-- 3) Decided on the Axum for web APIs, because it might be the right mix of complexity and utility to provide both learning and progress. -->
<!---->
<!-- 4) Tried to look at Axum docs and understood very little so I am taking a step back and looking at the tokio crate first. -->
<!---->
<!-- 5) Referencing Jon Gjensat's Videos to learn about tokio and axum: -->
<!--     Axum Video: https://www.youtube.com/watch?v=Wnb_n5YktO8 -->
<!---->
<!--     Tokio Video: https://www.youtube.com/watch?v=o2ob8zkeq2s -->
<!---->
<!-- 6) First will complete these videos and try to build the API to the database before starting the actual database. -->
# 🦀 Rustqlite

A journey to build a **SQLite-inspired database from scratch in Rust**, with a modern **Axum-based API layer**.  
This project is purely for **learning**, exploration, and fun.

---

## 📚 Learning Plan & Progress

1. ✅ **Completed** [The Rust Book](https://doc.rust-lang.org/book/) and `rustlings` for foundational Rust knowledge.

2. 💬 **Discussed high-level architecture** and design goals with ChatGPT (aka rubber ducking with extra RAM).

3. 🧩 **Chose Axum** for the API layer – a solid middle-ground between complexity and power.

4. 🤯 **Paused Axum** exploration after realizing I needed a better async foundation. Switching gears to understand **Tokio** first.

5. 🎥 **Watching Jon Gjengset's excellent videos** to get hands-on understanding:
   - [Tokio Video](https://www.youtube.com/watch?v=o2ob8zkeq2s)
   - [Axum Video](https://www.youtube.com/watch?v=Wnb_n5YktO8)

6. 🛠️ **Plan**: Finish these videos, build the Axum API layer first, then dive deep into **writing the database engine** from scratch.

---

## 🔭 Project Goals (WIP)
- A working REST API to query a custom DB engine.
- Implement minimal SQL-like operations (CRUD).
- Learn about memory management, disk I/O, and indexing.

---

> “Databases are just fancy file I/O... said no one who's written one.”
> — Someone probably

---



---

## ⚡ Tokio Summary — Async in Rust, Simplified

Tokio is Rust’s asynchronous runtime that supercharges concurrency and performance, especially for IO-heavy applications. Here's what makes it cool:

### 🚀 Key Concepts

* **Tokio = Async Runtime**
  It powers async code execution by managing tasks efficiently, switching context only when it benefits performance (e.g., during blocking IO).

* **Tasks, Not Threads**
  `tokio::spawn` creates *tasks*, which are lightweight top-level futures — not OS threads. This lets you run many concurrent operations without spawning actual threads.

* **Smart Context Switching**
  Tokio yields control from tasks stuck on IO to those that can actually make progress, avoiding wasted CPU time.

* **Familiar APIs**
  Many Tokio APIs intentionally resemble Rust’s standard library (like `std::thread`, `std::Mutex`, etc.), making async easier to adopt.

* **Async Utilities**
  Tokio provides async versions of essentiall tools:

  * `tokio::io` – non-blocking file and network IO
  * `tokio::sync::Mutex`, `RwLock` – async-safe synchronization
  * `tokio::stream` – async streams for data pipelines

> Think of it as an async version of the standard library... with extra caffeine. ☕⚙️

---


## 📌 Stay tuned! More commits, insights, and pain coming soon...
