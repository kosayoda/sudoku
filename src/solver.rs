use std::fmt::Display;

use bitvec::prelude::*;
use color_eyre::{eyre::eyre, Report, Result};
use once_cell::sync::OnceCell;
use owo_colors::OwoColorize;

use crate::board::Board;
use crate::UnicodeBox;

static ONES: OnceCell<BitArr!(for 9)> = OnceCell::new();

pub fn search(board: Solver) -> Option<Solver> {
    if let Some((x, y)) = board.get_next_square_to_assign() {
        let digits = board.board[x][y];
        for bit_index in digits[..9].iter_ones() {
            let mut board_copy = board;
            if board_copy.set_digit(x, y, bit_index).is_ok() {
                if let Some(solution) = search(board_copy) {
                    return Some(solution);
                }
            }
        }
    } else {
        return Some(board);
    }
    None
}

type Digits = BitArr!(for 9);

// Source: https://www.sebastiansylvan.com/post/sudoku/
#[derive(Clone, Copy)]
pub struct Solver {
    board: [[Digits; 9]; 9],
    possible_blocks: [[[Digits; 9]; 3]; 3],
    possible_rows: [[Digits; 9]; 9],
    possible_cols: [[Digits; 9]; 9],
    next_square_candidate: Option<(usize, usize)>,
}

impl Solver {
    pub fn eliminate(&mut self, x: usize, y: usize, mut digits_to_eliminate: Digits) -> Result<()> {
        // Only eliminate digits that are possible in the square
        digits_to_eliminate &= self.board[x][y];

        // All digits to eliminate are eliminated
        if digits_to_eliminate.not_any() {
            return Ok(());
        }

        // Eliminate the digits
        self.board[x][y] &= !digits_to_eliminate & ONES.get().unwrap();
        let remaining_digits = self.board[x][y];
        if remaining_digits.not_any() {
            return Err(eyre!("No possible digits left in ({}, {})", x, y));
        }

        let block_x = x / 3;
        let block_y = y / 3;
        let block_bit_index = (x % 3) + 3 * (y % 3);

        // Clear the reverse index
        let mut mask: Digits = BitArray::ZERO;
        for bit_index in digits_to_eliminate[..9].iter_ones() {
            mask[..9].fill(true);
            mask.set(y, false);
            self.possible_cols[x][bit_index] &= mask;
            if self.possible_cols[x][bit_index].not_any() {
                return Err(eyre!("No possible digits left in col {}", x));
            }

            mask[..9].fill(true);
            mask.set(x, false);
            self.possible_rows[y][bit_index] &= mask;
            if self.possible_rows[y][bit_index].not_any() {
                return Err(eyre!("No possible digits left in row {}", y));
            }

            mask[..9].fill(true);
            mask.set(block_bit_index, false);
            self.possible_blocks[block_x][block_y][bit_index] &= mask;
            if self.possible_blocks[block_x][block_y][bit_index].not_any() {
                return Err(eyre!(
                    "No possible digits left in block row {} col {}",
                    y,
                    x
                ));
            }
        }

        // If only one digit is left, eliminate that digit from all peers
        if remaining_digits.count_ones() == 1 {
            let remaining_digit_index = remaining_digits.first_one().unwrap();

            // Get all positions where this digit is set in the current row, column and block,
            // and eliminate them from those squares.

            // Eliminate from row
            let mut remaining_position_mask = self.possible_rows[y][remaining_digit_index];
            mask[..9].fill(true);
            mask.set(x, false); // Don't eliminate from current square
            remaining_position_mask &= mask;
            for bit_index in remaining_position_mask[..9].iter_ones() {
                self.eliminate(bit_index, y, remaining_digits)?;
            }

            // Eliminate from column
            remaining_position_mask = self.possible_cols[x][remaining_digit_index];
            mask[..9].fill(true);
            mask.set(y, false); // Don't eliminate from current square
            remaining_position_mask &= mask;
            for bit_index in remaining_position_mask[..9].iter_ones() {
                self.eliminate(x, bit_index, remaining_digits)?;
            }

            // Eliminate from block
            remaining_position_mask = self.possible_blocks[block_x][block_y][remaining_digit_index];
            mask[..9].fill(true);
            mask.set(block_bit_index, false); // Don't eliminate from current square
            remaining_position_mask &= mask;
            for bit_index in remaining_position_mask[..9].iter_ones() {
                let x_offset = bit_index % 3;
                let y_offset = bit_index / 3;
                self.eliminate(
                    block_x * 3 + x_offset,
                    block_y * 3 + y_offset,
                    remaining_digits,
                )?;
            }
        }

        // For each digit we just eliminated, find if it now only has one remaining posible location
        // in either the row, column or block. If so, set the digit
        for bit_index in digits_to_eliminate[..9].iter_ones() {
            // Check row
            if self.possible_rows[y][bit_index].count_ones() == 1 {
                let digit_x = self.possible_rows[y][bit_index].first_one().unwrap();
                self.set_digit(digit_x, y, bit_index)?;
            }

            // Check column
            if self.possible_cols[x][bit_index].count_ones() == 1 {
                let digit_y = self.possible_cols[x][bit_index].first_one().unwrap();
                self.set_digit(x, digit_y, bit_index)?;
            }

            // Check column
            if self.possible_blocks[block_x][block_y][bit_index].count_ones() == 1 {
                let bit_with_digit = self.possible_blocks[block_x][block_y][bit_index]
                    .first_one()
                    .unwrap();
                let x_offset = bit_with_digit % 3;
                let y_offset = bit_with_digit / 3;
                self.set_digit(block_x * 3 + x_offset, block_y * 3 + y_offset, bit_index)?;
            }
        }

        // While we're here, check if the square has a popcnt of 2, this means it's has the minimum
        // number of possibilities without being "done", making it an excellent candidate for the next
        // trial assignment.
        if self.board[x][y].count_ones() == 2 {
            self.next_square_candidate = Some((x, y));
        }

        Ok(())
    }

    pub fn set_digit(&mut self, x: usize, y: usize, digit: usize) -> Result<()> {
        let mut digit_mask: Digits = BitArray::ZERO;
        digit_mask.set(digit, true);

        assert!((self.board[x][y] & digit_mask).count_ones() > 0);

        self.board[x][y].set(digit, true);

        self.eliminate(x, y, !digit_mask & ONES.get().unwrap())
    }

    pub fn get_next_square_to_assign(self) -> Option<(usize, usize)> {
        if let Some((x, y)) = self.next_square_candidate {
            if self.board[x][y].count_ones() == 2 {
                return Some((x, y));
            }
        }

        let mut smallest_count = 10;
        let mut smallest: Option<(usize, usize)> = None;

        for i in 0..9 {
            for j in 0..9 {
                let count = self.board[i][j].count_ones();
                if count == 2 {
                    return Some((i, j));
                }

                if count > 1 && count < smallest_count {
                    smallest_count = count;
                    smallest = Some((i, j));
                }
            }
        }

        smallest
    }

    pub fn solved(&self) -> bool {
        for y in 0..9 {
            for x in 0..9 {
                if self.board[x][y].count_ones() != 1 {
                    return false;
                }

                // Make sure this digit isn't in any of the peers
                let digit = self.board[x][y].first_one().unwrap();
                for _x in 0..9 {
                    if _x != x && self.board[x][y].first_one().unwrap_or(10) == digit {
                        return false;
                    }
                }

                for _y in 0..9 {
                    if _y != y && self.board[x][y].first_one().unwrap_or(10) == digit {
                        return false;
                    }
                }

                let start_x = (x / 3) * 3;
                let start_y = (y / 3) * 3;
                let end_x = (x / 3 + 1) * 3;
                let end_y = (y / 3 + 1) * 3;

                for _x in (start_x..).take_while(|i| *i < end_x) {
                    for _y in (start_y..).take_while(|i| *i < end_y) {
                        if _x != x && _y != y && self.board[x][y].first_one().unwrap_or(10) == digit
                        {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

impl From<&Board> for Solver {
    fn from(value: &Board) -> Self {
        let board = [[bitarr![1; 9]; 9]; 9];

        let possible_blocks = [[[bitarr![1; 9]; 9]; 3]; 3];
        let possible_rows = [[bitarr![1; 9]; 9]; 9];
        let possible_cols = [[bitarr![1; 9]; 9]; 9];

        let mut solver = Solver {
            board,
            possible_blocks,
            possible_rows,
            possible_cols,
            next_square_candidate: None,
        };

        ONES.get_or_init(|| bitarr![1, 1, 1, 1, 1, 1, 1, 1, 1]);

        for (y, row) in (*value).squares().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell > 0 {
                    let adjusted_cell = *cell - 1;
                    solver
                        .set_digit(x, y, adjusted_cell)
                        .expect("Board setup should set digit without failure.");
                }
            }
        }
        solver
    }
}

impl Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", UnicodeBox::Top)?;

        for y in 1..=9 {
            if y % 4 == 0 {
                writeln!(f, "{}", UnicodeBox::Horizontal)?;
            }

            for x in 0..9 {
                if x % 3 == 0 {
                    write!(f, "{}", UnicodeBox::Vertical)?;
                }

                let cell = self.board[x][y - 1];
                if cell.count_ones() == 1 {
                    let val = cell.first_one().unwrap() + 1;
                    write!(f, "{}", val.yellow())?;
                    continue;
                } else {
                    let remaining = cell[..8].load_be::<u32>();

                    let cell_repr = char::from_u32(0x2800 + remaining).unwrap();
                    if *cell.get(8).unwrap() {
                        write!(f, "{}", cell_repr.green())?;
                    } else {
                        write!(f, "{}", cell_repr.blue())?;
                    }
                }
            }
            writeln!(f, "{}", UnicodeBox::Vertical)?;
        }

        write!(f, "{}", UnicodeBox::Bottom)
    }
}
