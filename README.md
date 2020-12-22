# Rusted Lolcat

![Build](https://github.com/devmode-io/rusted-lolcat/workflows/Build/badge.svg)

Rust implementation of https://github.com/busyloop/lolcat.

![Screen shot of Lolcat](/images/screen-shot.png?raw=true "Screen shot of Lolcat")

## Why??

I wanted to get my feet wet with Rust and found this fun project to replicate in Rust. Because of this reason I haven't used any CLI library. Instead this project uses a simple `while..let` loop and `match` to parse command line arguments.

## Development

### Running tests

```bash
cargo test
```

### Running local examples

```bash
cargo run -- test-input-1.txt test-input-2.txt
```
or

```bash
cat /dev/urandom | hexdump -C | target/release/rusted-lolcat
```
