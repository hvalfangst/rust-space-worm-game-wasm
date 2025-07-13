# Space Worm

A simple game written in Rust, inspired by the classic Snake game. The player controls a worm that grows longer as it eats food, while avoiding collisions with itself.
Every X score one is eligible to choose a perk. A perk is a special ability that can be selected to enhance the gameplay experience. Perks can provide advantages such as increased speed, temporary invincibility, or other unique effects.

Notice that no framework is used, and the game is built from scratch using Rust's standard library and a few crates for input handling and rendering.
All assets (sprites and audio) are created by me, Hichael, using Aseprite and Ableton Live.


## Requirements
* [Rust](https://www.rust-lang.org/tools/install)

## Cargo dependencies

* [minifb](https://crates.io/crates/minifb) - input handling
* [image](https://crates.io/crates/image) - sprite rendering
* [winit](https://docs.rs/winit) - sprite rescaling 
* [rodio](https://crates.io/crates/rodio) - audio playback
* [rand](https://crates.io/crates/rand) - random number generation


## Running program: Cargo

The shell script 'up' builds and runs our application by executing the following:
```
1. cargo build
2. cargo run
```