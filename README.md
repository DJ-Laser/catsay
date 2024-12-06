# Catsay
### A simple and fast cowsay-like tool that prints messages said by the cutest little guys ever: cats!

[![asciicast](https://asciinema.org/a/hZfsVTRbyuqcL6odbyYvImjm4.svg)](https://asciinema.org/a/hZfsVTRbyuqcL6odbyYvImjm4)

## Features:
- Easy to use CLI made with [Clap](https://crates.io/crates/clap)
- Full word wrap support (and newlines in stdin mode)
- 6 cats that can be picked rendomly, or chosen through a cli flag
- Stdin support for piping input from commands or scipts

## Web Demo
### [Try catsay on the web now!](https://dj-laser.github.io/catsay/)

The demo features a virtual terminal where you can use the catsay and clear commands

Tech Stack:
- [Vite](https://github.com/vitejs/vite) for development and bundling
- [Xterm.js](https://github.com/xtermjs/xterm.js) to run the virtual terminal environment
- [xterm-js-rs](https://crates.io/crates/xterm-js-rs): rust bindings for Xterm.js
- [wasm-pack](https://github.com/rustwasm/wasm-pack): for bundling rust code into webassembly

## Install via cargo
### `cargo install djlaser_catsay --features="cli"`

Catsay is mainly developed for the linux command line. Windows builds will most likely work, but they are unsupported
