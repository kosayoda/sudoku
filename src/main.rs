use std::fs::File;
use std::io::{BufRead, BufReader};

use color_eyre::Result;
use owo_colors::OwoColorize;

use ukodus::board::Board;
use ukodus::solver::{search, Solver};

fn main() -> Result<()> {
    color_eyre::install()?;

    let file = File::open("hard_sudokus.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    for line in lines {
        // let (_board, _solution) = line.split_once(',').unwrap();
        // println!("Board: {_board}");
        // println!("Solution: {_solution}");

        let board: Board = line.as_str().try_into()?;
        let solver: Solver = (&board).into();
        let solved = search(solver).unwrap();
        // let solution: Board = _solution.try_into()?;
        // println!("{}\n{}\n", "--- Solved ---".bold().green(), solved);
        // println!("{}\n{}\n", "-- Solution --".bold().red(), solution);
    }

    Ok(())
}

//
// fn main() -> Result<()> {
//     color_eyre::install()?;
//
//     let board: Board =
//         "000075400000000008080190000300001060000000034000068170204000603900000020530200000"
//             .try_into()?;
//
//     let solution: Board =
//         "693875412145632798782194356357421869816957234429368175274519683968743521531286947"
//             .try_into()?;
//
//     let solver: Solver = (&board).into();
//     println!("\n{}\n{}\n", "--- Board ---".bold().yellow(), board);
//     println!("{}\n{}\n", "--- Solver ---".bold().yellow(), solver);
//
//     if let Some(solved) = search(solver) {
//         println!("{}\n{}\n", "--- Solved ---".bold().green(), solved);
//     }
//
//     println!("{}\n{}\n", "-- Solution --".bold().red(), solution);
//     Ok(())
// }
