use a_puzzle_a_day_lib::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
/// month: 1~12, day: 1~31
pub fn find_solution(month: i32, day: i32, puzzle_type: i32, allow_flip: bool) -> String {
    let m = {
        let x = if month <= 6 { 0 } else { 1 };
        let y = (month - 1) - x * 6;
        Point::new(x as i32, y as i32)
    };
    let d = {
        let x = (day - 1) / 7 + 2;
        let y = (day - 1) % 7;
        Point::new(x as i32, y as i32)
    };
    let puzzle_type = if puzzle_type == 0 {
        PuzzleType::DragonFjord
    } else {
        PuzzleType::JarringWords
    };

    let board = Board::new_from_day_pos(m, d, puzzle_type);
    let blocks = Block::get_blocks(puzzle_type);
    let opts = SolverOptions {
        allow_flip,
        one_solution: true,
    };

    let sols = solve(&board, &blocks, &opts)
        .into_iter()
        .map(|s| s.board)
        .collect::<Vec<_>>();

    if sols.is_empty() {
        "".to_string()
    } else {
        sols[0].to_string()
    }
}
