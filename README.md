# ia41-project
Projet for the class IA41 ("Base concepts for artificial intelligence")

## Installation

To install this project, clone the repository and navigate to it:

```sh
git clone https://github.com/adri326/ia41-project
cd ia41-project
```

Then, simply run the file named `main.py`, followed by the path to a position:

```sh
python main.py boards/example.txt
```

You can optionally add the `--solve` option to let the included AI solve the position.

## Rust version

The rust version requires the `cargo` command-line tool to be compiled.

Run the following to run the AI on a given position:

```
cargo run --release -- boards/example.txt
```

You can also run the integrated benchmarks with `cargo bench`.
