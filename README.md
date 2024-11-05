# Football Robot

## Prerequisites

### AVR

You must have `avrdude` and `avr-gcc` along with some other packages depending on your platform. You can find platform specific instructions in the [`avr-hal` README].

### Ravedude

Next you can install [`ravedude`] with:

```sh
cargo +stable install ravedude
```

### Nightly Rust

This project requires nightly Rust and to avoid weird bugs from mismatching versions this project contains a [`rust-toolchain.toml`](./rust-toolchain.toml) file to force the use of a specific nightly version.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## Project Layout

Each robot, [`goalie`](goalie) and [`attack`](attack), have their own Rust crate for their game logic. Anything not related to game logic like reading sensors and controlling motors goes in the shared [`lib`](lib) crate that both robot crates depend on.
