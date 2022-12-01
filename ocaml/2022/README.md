# Advent of Code 2022 :camel:

## Build

This project uses [`dune`](https://dune.readthedocs.io/en/stable/index.html) as the OCaml build system.

```
cd aoc
dune build
```

## Execute

To run the solution for a particular day, first download the input file and save it in the `data` directory under: `data/<day>/input.txt`. Then, run the code for the given day in the format `dune exec d<number> <filepath>` with the appropriate input. For example:

```
dune exec d1 "data/01/input.txt"
```
