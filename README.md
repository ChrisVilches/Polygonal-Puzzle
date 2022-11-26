# Polygonal Puzzle

A geometric algorithm that finds the maximum possible length of the common boundary of two polygons when they are optimally placed.

**Problem Source:** ACM-ICPC World Finals 2016

## Overview

A Rust solution for the [Polygonal Puzzle](https://open.kattis.com/problems/puzzle2) problem.

<p align="center">
  <img src="https://github.com/ChrisVilches/Polygonal-Puzzle/blob/main/images/sample2.svg?raw=true" />
</p>

<p align="center">
  <img src="https://github.com/ChrisVilches/Polygonal-Puzzle/blob/main/images/sample3.svg?raw=true" />
</p>

## Run

Run all tests manually (it uses the official test data):

```sh
cargo run --release < tests/data/input
```

Or compare output using [cpdiff](https://github.com/ChrisVilches/cpdiff):

```sh
cargo run --release < tests/data/input | cpdiff tests/data/output
```

## Format & Lint

```sh
cargo fmt
touch src/main.rs && touch src/lib.rs && cargo clippy
```

## Testing

```sh
cargo test
```

## Alternative Solution (C++)

Another solution in C++ is [here](https://github.com/ChrisVilches/Algorithms/blob/main/kattis/puzzle2.cpp).
