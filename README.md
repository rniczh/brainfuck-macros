# Brainfuck macros in Rust

A non-optimization brainfuck macros for execute brainfuck language ~

<img src="https://i.imgur.com/A5jxigC.png" width="60%"/>


## Installation

```shell

git clone https://github.com/rniczh/brainfuck-macros.git

```

## Usage


In `Cargo.toml`

```rust

[dependencies]
brainfuck-macros = { path = "path/to/brainfuck-macros" }

```

In `main.rs`

```rust
#![feature(proc_macro_hygiene)]
use brainfuck_macros::brainfuck;

fn main() {
    brainfuck!(++++++++++[>+++++++>++++++++++>+++>+<<<<-]
               >++.>+.+++++++..+++.>++.<<+++++++++++++++.
               >.+++.------.--------.>+.>.);
}
```


Then

```shell
cargo +nightly run
```

---
