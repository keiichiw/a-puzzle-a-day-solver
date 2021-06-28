# A-Puzzle-A-Day Solver

A solver of [DragonFjord](https://www.dragonfjord.com/)'s [A-Puzzle-A-Day](https://www.dragonfjord.com/product/a-puzzle-a-day/).
The web version of this tool is available [here](http://keiichiw.github.io/a-puzzle-a-day-solver/).

The solver's algorithm is implemented in Rust and its code is compiled to WebAssembly to run on browsers.

## For Developers

### CLI

```
$ cargo run -p a-puzzle-a-day-cli -- --month Jan --day 1
```

### Web App

```
$ npm install # only once
$ npm run serve
```
