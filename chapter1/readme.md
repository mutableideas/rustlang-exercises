# Chatper 1 Summary

**File Names** - if using more than one word for a file name use underscores.

**Macros** - Macros have a ! e.g. `println!("Hello World!");`.  This will be discussed later.

## Compilation and Running

`Rust` is a ahead of time compiled language.

``` bash
rustc main.rs

// Windows Output
.\main.exe
```

### Cargo

**Cargo** - A package manager like npm to manage rust projects.  Handles building, downloading dependent libraries, and building those libraries.

**Cargo.toml** - Simliar to the `package.json` file for npm.

``` toml
[package]
name = "hello_cargo"
version = "0.0.1"
authors = ["Paul Mead <paul@ngserve.io>"]
edition = "2018"

# Line Comment done by hash

[dependencies]
```

Building, Run, Check

``` bash
#Buliding
cargo build

#For Release Build
cargo build --release

#Running
cargo run

#Check - Check quickly if code compiles
cargo check
```
