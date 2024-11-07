<div align="center">

# Football Robot

Our entry to the [Robocup Junior football competition](https://www.robocupjunior.org.nz/competition-categories/soccer-competition).

</div>

## Summary

The purpose of this project is to have two robots play a game of football against another team of two robots. One of our robots will act as a goalie while the other one will focus on attacking.

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

## Flashing and running the code

After conencting the Arduino to your computer you can run the following commands to flash the code to the Arduino and start reading the serial port.

```sh
cargo run -p goalie # or attack
```

## Project Layout

Each robot, [`goalie`](goalie) and [`attack`](attack), have their own Rust crate for their game logic. Anything not related to game logic like reading sensors and controlling motors goes in the shared [`lib`](lib) crate that both robot crates depend on.

There are also other crates under the [tools](tools) directory that you may find useful. They can be run the same way as the goalie or attack crates, just use the name of the tool instead.
