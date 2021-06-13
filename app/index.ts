// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('../public/pkg/index');

const BOARD_ID: string = "board";
const MONTH_FORM_ID: string = "month-form";
const DAY_FORM_ID: string = "day-form";
const FLIP_CHECK_ID: string = "allow-flip-check";

function findSolution() {
    const m_form =<HTMLSelectElement>document.getElementById(MONTH_FORM_ID);
    const month = m_form.selectedIndex + 1;
    const d_form =<HTMLSelectElement>document.getElementById(DAY_FORM_ID);
    const day = d_form.selectedIndex + 1;
    const flip_form =<HTMLInputElement>document.getElementById(FLIP_CHECK_ID);
    const allow_flip = flip_form.checked;

    resetBoard();

    callSolver(month, day, allow_flip).then(board => {
        renderBoard(board);
    }).catch(console.error);
}

function resetBoard() {
    let board = document.getElementById(BOARD_ID);
    board.innerText = "Searching...";
}

function renderBoard(s: string) {
    let board = document.getElementById(BOARD_ID);
    board.innerText = s;
}

async function callSolver(month: number, day: number, allow_flip: boolean): Promise<string> {
    if (!(1 <= month && month <= 12 && 1 <= day && day <= 31)) {
        console.error("Error: invalid date", month, day);
        return Promise.reject();
    }

    return rust.then(m => {
        const s: string = m.find_solution(month, day, allow_flip);
        return s;
    });
}

function addOptions() {
    const months = ["January", "Feburary", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    const m_form =<HTMLSelectElement>document.getElementById(MONTH_FORM_ID);
    months.forEach(m => {
        const opt = document.createElement("option");
        opt.text = m;
        m_form.add(opt);
    });

    const d_form =<HTMLSelectElement>document.getElementById(DAY_FORM_ID);
    for (let i = 1; i <= 31; i++) {
        const opt = document.createElement("option");
        opt.text = i.toString();
        d_form.add(opt);
    }
}



function initialize() {
    document.getElementById("find-button").onclick=findSolution;

    addOptions();
}

initialize();
