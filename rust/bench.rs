use std::fs::File;
use std::io::prelude::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub mod car;
pub mod board;
pub mod tree;

use board::Board;

const WIDTH: usize = 6;
const HEIGHT: usize = 6;

fn load_board(path: &str) -> Board {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    Board::from_string(&contents).expect("Error while parsing input file!")
}

pub fn bfs_benchmark(c: &mut Criterion) {
    let board_complex = load_board("boards/complex.txt");
    let board_simple = load_board("boards/simple.txt");
    let board_trivial = load_board("boards/trivial.txt");
    let board_no_solution = load_board("boards/no_solution.txt");

    c.bench_function("bfs complex", |b| b.iter(|| tree::bfs(board_complex.clone())));
    c.bench_function("bfs simple", |b| b.iter(|| tree::bfs(board_simple.clone())));
    c.bench_function("bfs trivial", |b| b.iter(|| tree::bfs(board_trivial.clone())));
    c.bench_function("bfs no_solution", |b| b.iter(|| tree::bfs(board_no_solution.clone())));
}

pub fn dfs_benchmark(c: &mut Criterion) {
    let board_complex = load_board("boards/complex.txt");
    let board_simple = load_board("boards/simple.txt");
    let board_trivial = load_board("boards/trivial.txt");

    // c.bench_function("iddfs complex", |b| b.iter(|| tree::iddfs(board_complex.clone(), None)));
    c.bench_function("iddfs simple", |b| b.iter(|| tree::iddfs(board_simple.clone(), None)));
    c.bench_function("iddfs trivial", |b| b.iter(|| tree::iddfs(board_trivial.clone(), None)));
}

criterion_group!(benches, bfs_benchmark, dfs_benchmark);
criterion_main!(benches);
