# rDeadfish

__rDeadfish__ is a free and open-source Deadfish interpreter written in Rust. It's very minimalistic and only consists of thirty something lines of code.

"Error handling is bloat." - SaynedBread

## Requirements

- Rust
- A clone of this repo

## Building

You can simply build __rDeadfish__ with cargo.

```sh
git clone https://github.com/CyntexMore/rDeadfish.git
cd rDeadfish
cargo build
```

## Usage

__rDeadfish__ can understand 5 commands:

| Command | Description                                                              |
|:-------:|--------------------------------------------------------------------------|
| i       | increments the accumulator                                               |
| d       | decrements the accumulator                                               |
| s       | squares the accumulator                                                  |
| o       | "prints" the value of the accumulator                                    |
| q       | (non-default) quits the program - it's recommended to run this by itself |

By default __rDeadfish__ opens a Deadfish shell but you can read "source code" from a `.df` file too if you want to by launching __rDeadfish__ with `./rDeadfish </path/to/file>` or with `cargo run -- </path/to/file>` if you haven't built it beforehand.
