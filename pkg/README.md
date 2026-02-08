# A-Puzzle-A-Day Solver

A solver of [DragonFjord](https://www.dragonfjord.com/)'s [A-Puzzle-A-Day](https://www.dragonfjord.com/product/a-puzzle-a-day/) and similar puzzle(s) like [this one with tetrominoes](public/tetromino_pieces.jpg).

The web version of this tool is available [here](http://keiichiw.github.io/a-puzzle-a-day-solver/).

The solver's algorithm is implemented in Rust and its code is compiled to WebAssembly to run on browsers.
Algorithm details are documented in [`SOLVER_ALGORITHM.md`](SOLVER_ALGORITHM.md).

Note that this is my personal hobby project and has nothing to do with the authors of the puzzles or my employer.

## URL Prefill

You can prefill the web UI with query parameters.

* `date=YYYY-MM-DD` (preferred)
* `month` / `day` (1-based)
* `type` (one of `dragonfjord`, `jarringwords`, `tetromino`, `weekday`, or `0-3`)
* `weekday` (`sun`..`sat` or `0-6`, only used for `weekday` type)

Examples:

* `/?date=2026-02-03&type=dragonfjord`
* `/?month=12&day=25&type=weekday&weekday=fri`

## For Developers

## Requirements

* [Rust](https://www.rust-lang.org/)
* [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/)
* [Node.js](https://nodejs.org/)

### CLI

```
$ cargo run -p a-puzzle-a-day-cli --bin a-puzzle-a-day-cli -- --month Jan --day 1
```

Use `--help` flag for advanced usages.

### Web App

#### Build

```
$ npm install # only once
$ npm run build # Build
```

#### Run Dev Server

```
$ npm run serve
```
