use a_puzzle_a_day_lib::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(mx: i32, my: i32, dx: i32, dy: i32) {
    let m = Point::new(mx, my);
    let d = Point::new(dx, dy);
    let s = get_solution(m, d);
    alert(&format!("Hello, World!\n{}", s[0]));
}

#[wasm_bindgen]
pub fn solution(mx: i32, my: i32, dx: i32, dy: i32) -> String {
    let m = Point::new(mx, my);
    let d = Point::new(dx, dy);
    get_solution(m, d)[0].to_string()
}

fn get_solution(month: Point, day: Point) -> Vec<Board> {
    let board = Board::new_from_day_pos(month, day);
    let blocks = Block::get_blocks();
    let opts = SolverOptions {
        allow_flip: false,
        one_solution: false,
    };

    let sols = solve(&board, &blocks, &opts);
    sols.into_iter().map(|s| s.board).collect()
}
