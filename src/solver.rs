use std::collections::BTreeSet;

use crate::block::*;
use crate::board::*;
use crate::point::*;

pub struct Solution {
    pub board: Board,
    _blocks: Vec<(Block, Point)>,
}

fn dfs(
    blocks: &[Block],
    available_blocks: &mut BTreeSet<usize>,
    board: &mut Board,
    acc: &mut Vec<(Block, Point)>,
    solutions: &mut Vec<Solution>,
    cnt: &mut u32,
) {
    if available_blocks.is_empty() {
        solutions.push(Solution {
            board: board.clone(),
            _blocks: acc.clone(),
        });
        return;
    }

    *cnt += 1;

    let p = board.first_empty_cell().unwrap();

    let block_ids = available_blocks.clone();
    for i in block_ids {
        let mut block = blocks[i].clone();
        available_blocks.remove(&i);
        for _ in 0..4 {
            block = block.rot();
            if let Err(_) = board.put_block(&p, i, &block) {
                continue;
            }
            acc.push((block.clone(), p));
            dfs(blocks, available_blocks, board, acc, solutions, cnt);
            acc.pop();
            board
                .remove_block(&p, &block)
                .expect("remove block must succeed");
        }
        available_blocks.insert(i);
    }
}

pub fn solve(board: &Board, blocks: &[Block]) -> Vec<Solution> {
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
    let mut acc = vec![];
    let mut solutions = vec![];
    dfs(
        blocks,
        &mut available_blocks,
        &mut mut_board,
        &mut acc,
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

        assert!(solve(&board, &blocks).is_ok());
    }
}
