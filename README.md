# A framework for making games with rust

This framework is made out of three crates in the include directory:

* sdl_helper - for hiding the implimentation details of sdl2 in rust, and some helper functions for game development
* tiled - for loading Tiled maps
* geometry - for rectangle and vector maths


The src directory acts as an example template of how to use the crates

## Features

* window creation and keyboard/mouse input
* loading and drawing textures and fonts to the screen
* loading and drawing [Tiled](https://www.mapeditor.org/) maps 
* 2D camera for scaling and moving drawn textures and fonts

## Dependancies

* [rust-sdl2](https://crates.io/crates/sdl2) for windowing, rendering, input, resource loading
* [quick_xml](https://crates.io/crates/quick-xml) for loading tiled maps

## Projects using this framework

* [Coupled Explorers - for LD51 - Platformer](https://github.com/NoamZeise/Coupled-Explorers-LD51)
* [Hex - for a 48hr Jam - Falling Block Puzzle](https://github.com/NoamZeise/Hex)

## TODO

* audio loading and playback
* Better resolution controls
* Controller support

 
