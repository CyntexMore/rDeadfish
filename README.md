# rDeadfish

**rDeadfish** is a blazingly fast, free and open-source Deadfish interpreter written in Rust. It's very minimalistic and only consists of 58 lines of code.

"Error handling is bloat." - SaynedBread

## Requirements

- A local installation of Rust
- A clone of this repo

## Building

You can simply build **rDeadfish** with cargo.

```sh
git clone https://github.com/CyntexMore/rDeadfish.git
cd rDeadfish
cargo build --release
```

## Usage

**rDeadfish** can understand 5 commands:

| Command | Description                                                              |
|:-------:|--------------------------------------------------------------------------|
| i       | increments the accumulator                                               |
| d       | decrements the accumulator                                               |
| s       | squares the accumulator                                                  |
| o       | "prints" the value of the accumulator                                    |
| q       | (non-default) quits the program - it's recommended to run this by itself |

By default **rDeadfish** opens a Deadfish shell but you can read "source code" from well, any file (although it's recommended to use the `.df` file extension for clarity) by launching **rDeadfish** with `./target/release/rDeadfish </path/to/file>` or with `cargo run -- </path/to/file>` if you haven't built it beforehand.

## Thanks

Thanks to [pml68](https://github.com/pml68/) **rDeadfish** has the ability to read from "source code".
