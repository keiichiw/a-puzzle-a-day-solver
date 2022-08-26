# A-Puzzle-A-Day Solver

A solver of [DragonFjord](https://www.dragonfjord.com/)'s [A-Puzzle-A-Day](https://www.dragonfjord.com/product/a-puzzle-a-day/) and similar puzzle(s) like [this one with tetrominoes](public/tetromino_pieces.jpg).

The web version of this tool is available [here](http://keiichiw.github.io/a-puzzle-a-day-solver/).

The solver's algorithm is implemented in Rust and its code is compiled to WebAssembly to run on browsers.

Note that this is my personal hobby project and has nothing to do with the authors of the puzzles or my employer.

## For Developers

## Requirements

* [Rust](https://www.rust-lang.org/)
* [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/)
* [Node.js](https://nodejs.org/)

### CLI

```
$ cargo run -p a-puzzle-a-day-cli -- --month Jan --day 1
```

Use `--help` flag for advanced usages.

### Web App

```
$ npm install # only once
$ npm run serve
```
