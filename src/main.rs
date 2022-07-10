use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read},
    path::Path,
};

use bitstream_io::{BigEndian, BitRead, BitReader};
use color_eyre::{eyre::anyhow, Result};
use owo_colors::OwoColorize;
use rand::Rng;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    color_eyre::install()?;

    let log_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(log_subscriber)?;

    let puzzle_path = Path::new("data/compressed/puzzles0_kaggle.puzzle");
    let _reader = BufReader::new(File::open(&puzzle_path)?);
    let mut reader = BitReader::endian(_reader, BigEndian);

    let num_puzzles = reader.read::<u32>(32)?;
    println!("Puzzles found: {}", num_puzzles.cyan());

    let mut rng = rand::thread_rng();

    let puzzle_no = rng.gen_range(1..=num_puzzles);
    println!("Getting a random puzzle: {}", puzzle_no.green());

    // Skip puzzle
    for _ in 1..puzzle_no {
        let skip_length = reader.read::<u16>(16)?;
        reader.skip(skip_length.into())?;
    }

    let puzzle_length = reader.read::<u16>(16)?;
    println!("Puzzle length: {} bits", puzzle_length.yellow());

    let mut representation = String::with_capacity(81);
    for _ in 0..81 {
        let filled = reader.read::<u32>(1)?;
        if filled == 0 {
            representation.push('.');
            continue;
        }

        assert_eq!(filled, 1);
        let num = reader.read::<u32>(4)?;
        let digit = char::from_digit(num, 10).ok_or_else(|| anyhow!("Invalid digit: {num}"))?;
        representation.push(digit);
    }
    println!("Puzzle: {}", representation.blue());

    Ok(())
}
