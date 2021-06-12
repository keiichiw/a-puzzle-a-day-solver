// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('../pkg/a_puzzle_a_day_solver');

function render_board(s: string) {
    let board = document.getElementById("board");
    board.innerText = s;
}


rust.then(m => {
    let x = m.solution(0, 0, 1, 0);
    render_board(x);
    console.log("Hello!");
})
    .catch(console.error);
