use a_puzzle_a_day_lib::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(m: i32, d: i32) {
    let s = find_solution(m, d, false);
    alert(&format!("Hello, World!\n{}", s));
}

#[wasm_bindgen]
/// month: 1~12, day: 1~31
pub fn find_solution(month: i32, day: i32, allow_flip: bool) -> String {
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

    let board = Board::new_from_day_pos(m, d);
    let blocks = Block::get_blocks();
    let opts = SolverOptions {
        allow_flip,
        one_solution: true,
    };

    let sols = solve(&board, &blocks, &opts)
        .into_iter()
        .map(|s| s.board)
        .collect::<Vec<_>>();

    if sols.is_empty() {
        "Not found".to_string()
    } else {
        sols[0].to_string()
    }
}
