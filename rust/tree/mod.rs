#[allow(unused_imports)]
use super::board::{Board, BoardIter};
#[allow(unused_imports)]
use super::{WIDTH, HEIGHT};
use super::car::CarMove;
use std::collections::{VecDeque, HashSet, HashMap};
use std::rc::Rc;

pub mod hash;
pub mod list;
use list::{RcLinkedList, RcNode};

#[inline]
pub fn target(board: &Board) -> bool {
    board.cars[0].x == WIDTH - 2
}

pub fn bfs(initial: Board) -> Option<Vec<CarMove>> {
    let mut queue: VecDeque<(Board, RcNode<CarMove>)> = VecDeque::new();
    let mut visited: HashSet<u64, hash::IdentityHashBuilder> = HashSet::with_hasher(hash::IdentityHashBuilder::new());
    visited.insert(initial.get_board_hash());
    queue.push_back((initial, None));

    while let Some((board, moves)) = queue.pop_front() {
        for mv in board.iter() {
            let new_board = board.apply(&mv);
            let new_moves = Rc::new(RcLinkedList::new(mv, &moves));

            if target(&new_board) {
                return Some(new_moves.into_iter_rc().collect());
            } else {
                let hash = new_board.get_board_hash();
                if !visited.contains(&hash) {
                    visited.insert(hash);
                    queue.push_back((new_board, Some(new_moves)));
                }
            }
        }
    }

    None
}

fn dfs_recurse(board: &Board, depth: usize, visited: &mut HashMap<u64, usize, hash::IdentityHashBuilder>) -> Option<Vec<CarMove>> {
    for mv in board.iter() {
        let new_board = board.apply(&mv);
        if target(&new_board) {
            return Some(vec![mv]);
        } else if depth > 0 {
            let hash = new_board.get_board_hash();
            if let Some(v) = visited.get(&hash) {
                if *v >= depth {
                    continue;
                } else {
                    visited.insert(hash, depth);
                }
            } else {
                visited.insert(hash, depth);
            }

            match dfs_recurse(&new_board, depth - 1, visited) {
                Some(mut arr) => {
                    arr.push(mv);
                    return Some(arr);
                },
                None => {}
            }
        }
    }

    None
}

pub fn dfs(initial: Board, max_depth: usize) -> Option<Vec<CarMove>> {
    let mut visited: HashMap<u64, usize, hash::IdentityHashBuilder> = HashMap::with_hasher(hash::IdentityHashBuilder::new());

    match dfs_recurse(&initial, max_depth, &mut visited) {
        Some(mut arr) => {
            arr.reverse();
            Some(arr)
        }
        None => None
    }
}

pub fn iddfs(initial: Board, max_depth: Option<usize>) -> Option<Vec<CarMove>> {
    let max_depth = max_depth.unwrap_or(usize::MAX);
    let mut visited: HashMap<u64, usize, hash::IdentityHashBuilder> = HashMap::with_hasher(hash::IdentityHashBuilder::new());

    for depth in 0..max_depth {
        if let Some(mut res) = dfs_recurse(&initial, depth, &mut visited) {
            // res.reverse();
            return Some(res);
        }
    }

    None
}
