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

    c.bench_function("bfs complex", |b| b.iter(|| tree::bfs(board_complex.clone())));
    c.bench_function("bfs simple", |b| b.iter(|| tree::bfs(board_simple.clone())));
    c.bench_function("bfs trivial", |b| b.iter(|| tree::bfs(board_trivial.clone())));
}

criterion_group!(benches, bfs_benchmark);
criterion_main!(benches);
