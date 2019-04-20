# Brainfuck_macros in Rust

A non-optimization brainfuck macros for execute brainfuck language ~

<img src="https://i.imgur.com/A5jxigC.png"/>


## Installation

```shell

git clone https://github.com/rniczh/brainfuck_macros.git

```

## Usage


In `Cargo.toml`

```rust

[dependencies]
brainfuck_macros = { path = "path/to/brainfuck_macros" }

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
