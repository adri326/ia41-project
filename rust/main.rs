#[macro_use]
extern crate static_assertions;

use std::fs::File;
use std::io::prelude::*;
use std::env;

pub mod car;
pub mod board;
pub mod tree;

use board::Board;

const WIDTH: usize = 6;
const HEIGHT: usize = 6;

fn main() -> std::io::Result<()> {
    let path = env::args().last().unwrap();

    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let board = Board::from_string(&contents).expect("Error while parsing input file!");
    println!("{}", board);
    println!("");

    // Used for profiling
    for _ in 0..100 {
        tree::iddfs(board.clone(), None);
    }

    // let solution = tree::iddfs(board.clone(), None);
    let solution = tree::bfs(board.clone());

    let mut board = board;
    if let Some(solution) = solution {
        for mv in solution.into_iter().rev() {
            board = board.apply(&mv);
            println!("{}", mv);
        }
        println!("");
        println!("{}", board);
    }

    Ok(())
}
