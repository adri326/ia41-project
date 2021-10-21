use super::board::{Board, BoardIter};
use super::{WIDTH, HEIGHT};
use super::car::CarMove;
use std::collections::{VecDeque, HashSet};

#[inline]
pub fn target(board: &Board) -> bool {
    board.cars[0].x == WIDTH - 2
}

pub fn bfs(initial: Board) -> Option<Vec<CarMove>> {
    let mut queue: VecDeque<(Board, Vec<CarMove>)> = VecDeque::new();
    let mut visited: HashSet<[bool; WIDTH * HEIGHT]> = HashSet::new();
    queue.push_back((initial, Vec::new()));

    while let Some((board, moves)) = queue.pop_front() {
        visited.insert(board.board.clone());
        // println!("{:?}", moves.iter().map(|m| (m.index, m.to.0, m.to.1)).collect::<Vec<_>>());
        for mv in board.iter() {
            let new_board = board.apply(&mv);
            let mut moves = moves.clone();
            moves.push(mv);
            if target(&new_board) {
                return Some(moves);
            } else if !visited.contains(&new_board.board) {
                queue.push_back((new_board, moves));
            }
        }
    }

    None
}
