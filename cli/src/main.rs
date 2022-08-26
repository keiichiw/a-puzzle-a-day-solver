use std::env;

use a_puzzle_a_day_lib::*;
use anyhow::{bail, Context, Result};
use getopts::Options;

const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.reqopt(
        "m",
        "month",
        "month",
        &format!("[{}]", MONTH_NAMES.to_vec().join("|")),
    );
    opts.reqopt("d", "day", "day", "[1-31]");
    opts.optflag("f", "allow-flip", "allow flipping pieces");
    opts.optflag(
        "o",
        "one-solution",
        "stop searching once one solution is found",
    );
    opts.optopt(
        "t",
        "type",
        "puzzle type. 'd' for 'DragonFjord', 'j' for 'JarringWords', or 't' for 'Tetromino' (default='d')",
        "[d|j|t]",
    );
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", opts.usage(&program));
            bail!(f.to_string())
        }
    };
    if matches.opt_present("h") {
        println!("{}", opts.usage(&program));
        return Ok(());
    }

    let allow_flip = matches.opt_present("f");
    let one_solution = matches.opt_present("o");
    let opts = SolverOptions {
        allow_flip,
        one_solution,
    };

    let month_str: String = matches.opt_get("month").unwrap().unwrap();
    let month_pos = match MONTH_NAMES.iter().position(|m| *m == month_str) {
        None => {
            bail!("unexpected month name: {}", month_str);
        }
        Some(p) => {
            let x = if p <= 5 { 0 } else { 1 };
            let y = p - x * 6;
            Point::new(x as i32, y as i32)
        }
    };

    let day: u32 = matches
        .opt_get::<String>("day")
        .unwrap()
        .unwrap()
        .parse()
        .context("invalid number was given as day")?;
    let day_pos = {
        let x = (day - 1) / 7 + 2;
        let y = (day - 1) % 7;
        Point::new(x as i32, y as i32)
    };

    let typ = matches
        .opt_get::<PuzzleType>("type")?
        .unwrap_or(PuzzleType::DragonFjord);

    let board = Board::new_from_day_pos(month_pos, day_pos, typ);
    let blocks = Block::get_blocks(typ);

    let sols = solve(&board, &blocks, &opts);
    if sols.is_empty() {
        println!("Solution not found");
        return Ok(());
    }
    println!("{} solutions found.", sols.len());

    for (idx, s) in sols.iter().enumerate() {
        assert!(s.board.first_empty_cell().is_none());
        print!("Solution {}:\n{}", idx, s.board);
        println!("=======");
    }

    Ok(())
}
