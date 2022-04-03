# A-Puzzle-A-Day Solver

A solver of the following calendar puzzles:
* [DragonFjord](https://www.dragonfjord.com/)'s [A-Puzzle-A-Day](https://www.dragonfjord.com/product/a-puzzle-a-day/).
* [JarringWords](http://www.jarringwords.com/)'s [calendar puzzle](https://www.etsy.com/jp/listing/1032608229/).

The web version of this tool is available [here](http://keiichiw.github.io/a-puzzle-a-day-solver/).

The solver's algorithm is implemented in Rust and its code is compiled to WebAssembly to run on browsers.

## For Developers

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
