use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use color_eyre::Result;
use owo_colors::OwoColorize;

use ukodus::board::Board;
use ukodus::solver::{search, Solver};

fn main() -> Result<()> {
    color_eyre::install()?;

    let reader = BufReader::new(File::open("all_17_clue_sudokus.txt")?);

    let mut total_time = std::time::Duration::new(0, 0);
    for line in reader.lines().flatten() {
        let board: Board = line.as_str().try_into()?;
        // println!("Board: {line}");
        // println!("{}\n{}\n", "--- Board ---".bold().green(), board);
        let solver: Solver = (&board).into();

        let before = std::time::Instant::now();
        let solution = search(solver).unwrap();
        // println!("{}\n{}\n", "-- Solution --".bold().red(), solution);

        total_time += before.elapsed();
    }
    println!("Elapsed time: {:2?}", total_time.green());

    Ok(())
}
