# Chapter 1 Notes

## 1.1 - Installation

## 1.2 - Hello, World
Rust files end in `.rs`. The entry point for a Rust programs is `main()`.
To create a program, use `rustc`, the Rust compiler, which results in an
executable, which can be ran like any other executable.

## 1.3 - Hello, Cargo
Cargo is the package manager of Rust. Packages are called crates. `cargo new` creates a new project, `cargo init` creates one from an existing project.
`cargo build` will compile a project, artifacts are location in `/target`, `cargo build --release` will build with optimizations, slower to do so. `cargo run` will compile, if needed, and run. `cargo check` will check if a package will build without error, without actually building it, useful for develop+check processes.
