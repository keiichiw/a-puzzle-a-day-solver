// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('../public/pkg/index');

const BOARD_ID: string = "board";
const HINT_ID: string = "hint";
const MONTH_FORM_ID: string = "month-form";
const DAY_FORM_ID: string = "day-form";

function buttonOnClick() {
    const m_form =<HTMLSelectElement>document.getElementById(MONTH_FORM_ID);
    const month = m_form.selectedIndex + 1;
    const d_form =<HTMLSelectElement>document.getElementById(DAY_FORM_ID);
    const day = d_form.selectedIndex + 1;

    resetBoard();

    callSolver(month, day).then(result => {
        console.log(result);
        renderBoard(result);
    })
}

function resetBoard() {
    let board = document.getElementById(BOARD_ID);
    board.innerText = "Searching...";

    let hint = document.getElementById(HINT_ID);
    hint.innerText = "";
}

function renderBoard(s: string) {
    let board = document.getElementById(BOARD_ID);
    board.innerText = s;
}

async function callSolver(month: number, day: number): Promise<string> {
    if (!(1 <= month && month <= 12 && 1 <= day && day <= 31)) {

        throw new Error("Error: invalid date: " + month + ", " + day);
    }

    // If there is a solution without flipping, return it.
    let r = await rust.then(m => {
        return m.find_solution(month, day, false /* allow_flip */);
    });
    if (r != "") {
        return r;
    }

    let hint = document.getElementById(HINT_ID);
    hint.innerText = "(No solution without flipping pieces.)";

    return await rust.then(m => {
        return m.find_solution(month, day, true);
    });
}

function addOptions() {
    const months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    const today = new Date();

    const m_form =<HTMLSelectElement>document.getElementById(MONTH_FORM_ID);
    months.forEach(m => {
        const opt = document.createElement("option");
        opt.text = m;
        m_form.add(opt);
    });
    m_form.selectedIndex = today.getMonth();

    const d_form =<HTMLSelectElement>document.getElementById(DAY_FORM_ID);
    for (let i = 1; i <= 31; i++) {
        const opt = document.createElement("option");
        opt.text = i.toString();
        d_form.add(opt);
    }
    d_form.selectedIndex = today.getDate() - 1;
}



function initialize() {
    document.getElementById("find-button").onclick=buttonOnClick;

    addOptions();
}

initialize();
