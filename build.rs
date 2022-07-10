use std::{
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use bitstream_io::{BigEndian, BitWrite, BitWriter};
use color_eyre::{eyre::anyhow, Result};

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=data/plaintext");

    let input_dir = Path::new("data/plaintext");
    let output_dir = Path::new("data/compressed");
    let mut compress_log = BufWriter::new(File::create(output_dir.join("log.txt"))?);

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let input_file = entry.path();
        // Skip non .puzzle files
        if input_file
            .extension()
            .map(|ext| !ext.eq_ignore_ascii_case("puzzle"))
            .unwrap_or(true)
        {
            continue;
        }

        let reader = BufReader::new(File::open(&input_file)?);

        let mut total_puzzles = 0_u32;
        let mut total_puzzle_length = 0_u32;
        let mut max_puzzle_length = u16::MIN;
        let mut min_puzzle_length = u16::MAX;

        let mut _all_puzzles: Vec<u8> = Vec::new();
        let mut all_puzzles = BitWriter::endian(&mut _all_puzzles, BigEndian);

        for line in reader.lines().map(|l| l.unwrap()) {
            // Skip comments
            if line.starts_with('#') {
                continue;
            }

            // Data format requires 81 + 4n bits where n is the number of clues.
            // n = 17 takes 149 bits, n = 45 takes 261 bits.
            // Allocating 35 * 8 = 280 to hopefully avoid reallocation.
            let mut _current_puzzle: Vec<u8> = Vec::with_capacity(35);
            let mut current_puzzle = BitWriter::endian(&mut _current_puzzle, BigEndian);

            // Puzzle length in bits
            let mut puzzle_length: u16 = 0;

            // Process puzzle
            for c in line.chars() {
                // Empty cell
                if c == '.' {
                    current_puzzle.write_bit(false)?;
                    puzzle_length += 1;
                    continue;
                }

                // Filled cell
                let digit = c
                    .to_digit(10)
                    .ok_or_else(|| anyhow!("Character {c} is not a valid digit!"))?;
                current_puzzle.write_bit(true)?;
                current_puzzle.write(4, digit)?;
                puzzle_length += 5;
            }

            // Add the number of bits the puzzle takes, then the puzzle's bit
            // representation
            all_puzzles.write(16, puzzle_length)?;
            let (bits_remaining, value) = current_puzzle.into_unwritten();
            all_puzzles.write_bytes(&_current_puzzle)?;
            all_puzzles.write(bits_remaining, value)?;

            total_puzzle_length += u32::from(puzzle_length);
            min_puzzle_length = min_puzzle_length.min(puzzle_length);
            max_puzzle_length = max_puzzle_length.max(puzzle_length);
            total_puzzles += 1;
        }

        // Pad `all_puzzles` to ensure the last puzzle is flushed.
        all_puzzles.write(7, 0b0)?;

        let filename = input_file
            .file_name()
            .ok_or_else(|| anyhow!("Unable to retrieve filename for file: {input_file:?}"))?;
        let mut writer = BufWriter::new(File::create(output_dir.join(filename))?);
        writer.write_all(&total_puzzles.to_be_bytes())?;
        writer.write_all(&_all_puzzles)?;

        compress_log.write_fmt(format_args!(
            "Puzzle Compression Statistics for: {:?}",
            filename
        ))?;
        compress_log.write_fmt(format_args!("Puzzles processed: {}", total_puzzles))?;
        compress_log.write_fmt(format_args!("Max length: {} bits", max_puzzle_length))?;
        compress_log.write_fmt(format_args!("Min length: {} bits", min_puzzle_length))?;
        compress_log.write_fmt(format_args!(
            "Average length: {:.2} bits\n\n",
            f64::from(total_puzzle_length) / f64::from(total_puzzles)
        ))?;
    }

    Ok(())
}
