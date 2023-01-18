# A framework for making games with rust

This framework is made out of three crates in the include directory:

* sdl_helper - for hiding the implementation details of sdl2 in rust, and some helper functions for game development
* tiled - for loading Tiled maps
* geometry - for rectangle and vector maths


The src directory acts as an example template of how to use the crates

## Features

* window creation and keyboard/mouse input
* loading and drawing textures and fonts to the screen
* loading and drawing [Tiled](https://www.mapeditor.org/) maps 
* 2D camera for scaling and moving drawn textures and fonts

## Setup 

1. Clone this repository 
2. Install [Rust](https://www.rust-lang.org/tools/install) if you haven't already
3. Install SDL2, SDL2_image, SDL2_mixer, SDL2_ttf
4. go to the root directory of this project and run 
```
$ cargo run
```

### Installing SDL2 on debian-based systems
Run the following command
```
$ sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-ttf-dev
```
### Installing SDL2 on other systems

follow the instructions for your system given in the repo for [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2), but remember you also need to get sdl2_mixer, sdl2_ttf and sdl2_image.


## Dependancies

* [rust-sdl2](https://crates.io/crates/sdl2) for windowing, rendering, input, resource loading
* [quick_xml](https://crates.io/crates/quick-xml) for loading tiled maps

## Projects using this framework

* [Coupled Explorers - for LD51 - Platformer](https://github.com/NoamZeise/Coupled-Explorers-LD51)
* [Hex - for a 48hr Jam - Falling Block Puzzle](https://github.com/NoamZeise/Hex)

## TODO

* audio loading and playback
* Better resolution controls
