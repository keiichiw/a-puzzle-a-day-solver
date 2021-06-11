use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BTreeSet;

use crate::block::*;
use crate::board::*;
use crate::point::*;

//#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Solution {
    pub board: Board,
    moves: Vec<(Block, Point)>,
}

impl Solution {
    fn new(board: Board, mut moves: Vec<(Block, Point)>) -> Self {
        moves.sort(); // guarantee that `moves` is sorted`
        Self { board, moves }
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.moves == other.moves
    }
}

impl Eq for Solution {}

impl Ord for Solution {
    fn cmp(&self, other: &Self) -> Ordering {
        self.moves.cmp(&other.moves)
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dfs(
    blocks: &[Block],
    allow_flip: bool,
    available_blocks: &mut BTreeSet<usize>,
    board: &mut Board,
    moves: &mut Vec<(Block, Point)>,
    solutions: &mut BTreeSet<Solution>,
    cnt: &mut u32,
) {
    if available_blocks.is_empty() {
        solutions.insert(Solution::new(board.clone(), moves.clone()));
        return;
    }

    *cnt += 1;

    let p = board.first_empty_cell().unwrap();

    let block_ids = available_blocks.clone();
    for i in block_ids {
        let blk = blocks[i].clone();
        available_blocks.remove(&i);
        for block in blk.possible_dirs(allow_flip) {
            if let Err(_) = board.put_block(&p, i, &block) {
                continue;
            }
            moves.push((block.clone(), p));
            dfs(
                blocks,
                allow_flip,
                available_blocks,
                board,
                moves,
                solutions,
                cnt,
            );
            moves.pop();
            board
                .remove_block(&p, &block)
                .expect("remove block must succeed");
        }
        available_blocks.insert(i);
    }
}

pub fn solve(board: &Board, blocks: &[Block], allow_flip: bool) -> BTreeSet<Solution> {
    let mut candidate_ps = vec![];
    for i in 0..board.height() {
        for j in 0..board.width() {
            if board.board[i][j] == State::Empty {
                candidate_ps.push(Point::new(i as i32, j as i32));
            }
        }
    }

    let mut mut_board = board.clone();
    let mut cnt = 0;

    let mut available_blocks = BTreeSet::new();
    for i in 0..blocks.len() {
        available_blocks.insert(i);
    }
    let mut moves = vec![];
    let mut solutions = BTreeSet::new();
    dfs(
        blocks,
        allow_flip,
        &mut available_blocks,
        &mut mut_board,
        &mut moves,
        &mut solutions,
        &mut cnt,
    );
    println!("Count: {}", cnt);
    solutions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_jan1() {
        // January 1st
        let board = Board::new_from_day_pos(Point::new(0, 0), Point::new(2, 0));
        let blocks = Block::get_blocks();

        assert_eq!(solve(&board, &blocks, false /* allow_flip */).len(), 2);
    }

    #[test]
    fn test_solve_dec29() {
        // January 1st
        let board = Board::new_from_day_pos(Point::new(1, 5), Point::new(6, 0));
        let blocks = Block::get_blocks();

        assert_eq!(solve(&board, &blocks, false /* allow_flip */).len(), 0);
        assert_eq!(solve(&board, &blocks, true /* allow_flip */).len(), 54);
    }
}
