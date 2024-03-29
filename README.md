# A library for making 2D games with rust, uses SDL2.

This library is made out of three crates:

* nze\_game\_sdl - for hiding the implementation details of sdl2 in rust. Load resources (texture, font, audio), rendering, 2D camera, audio playback, drawing tilemaps
User Input, Windowing
* nze_tiled - (in lib dir) for loading Tiled maps
* nze_geometry - (in lib dir) for 2D geometry types and functions

[Documentation](https://docs.rs/nze_game_sdl/latest/nze_game_sdl/)

Check out the examples folder for how you might use the library

## Features

* window creation and keyboard/mouse/controller input
* loading and drawing textures and fonts to the screen
* loading and drawing [Tiled](https://www.mapeditor.org/) maps 
* 2D camera for scaling and moving drawn textures and fonts
* loading and playing music and sound effect

## Setup 

1. Clone this repository
2. Install [Rust](https://www.rust-lang.org/tools/install) if you haven't already
3. Install SDL2, SDL2_image, SDL2_mixer, SDL2_ttf (see sections below for details)
4. go to the root directory of this project and run 
```
$ cargo run --example main
```
to make sure everything is working.

### Installing SDL2 on debian-based systems
Run the following command
```
$ sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-ttf-dev
```
### Installing SDL2 on other systems

Follow the instructions for your system given in the repo for [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2), but remember you also need to get sdl2_mixer, sdl2_ttf and sdl2_image libraries.

## Dependancies

* [rust-sdl2](https://crates.io/crates/sdl2) for windowing, rendering, input, resource loading (also need the SDL2  C libraries installed on the system, see Setup for details)
* [quick_xml](https://crates.io/crates/quick-xml) for loading tiled maps

## Projects using this framework

* [DeliCatEssen - for DSJ2023 - Falling Puzzle](https://github.com/NoamZeise/DSJ2023)
* [Bunny Patch - for GGJ2023 - Simulation](https://github.com/NoamZeise/BunnyPatch) - outdated version of lib
* [Coupled Explorers - for LD51 - Platformer](https://github.com/NoamZeise/Coupled-Explorers-LD51) - very outdated version of lib
* [Hex - for a 48hr Jam - Falling Block Puzzle](https://github.com/NoamZeise/Hex) - very outdated version lib

## TODO

* Better resolution controls
* Fade effects for audio
* Add point/line render options
* make (tiled, font, audio) dependancies optional
