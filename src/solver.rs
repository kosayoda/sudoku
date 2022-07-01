use std::fmt::Debug;

use color_eyre::{eyre::eyre, Result};

static BIT_MASK: [u16; 16] = [
    1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768,
];

macro_rules! iter_ones {
    (for $bit_index:ident in $bitset:ident => $body:block ) => {
        for $bit_index in 0..16 {
            if $bitset & BIT_MASK[$bit_index] != 0 {
                $body
            }
        }
    };
}

type DigitMask = u16;

pub fn search(board: Solver) -> Option<Solver> {
    if let Some((x, y)) = board.get_next_square_to_assign() {
        let digits = board.board[x][y];
        iter_ones!(for bit_index in digits => {
            let mut board_copy = board;
            if board_copy.set_digit(x, y, bit_index).is_ok() {
                if let Some(solution) = search(board_copy) {
                    return Some(solution);
                }
            }
        });
    } else {
        return Some(board);
    }
    None
}

// Source: https://www.sebastiansylvan.com/post/sudoku/
#[derive(Clone, Copy)]
pub struct Solver {
    pub board: [[DigitMask; 9]; 9],
    pub possible_blocks: [[[DigitMask; 9]; 3]; 3],
    pub possible_rows: [[DigitMask; 9]; 9],
    pub possible_cols: [[DigitMask; 9]; 9],
    pub next_square_candidate: Option<(usize, usize)>,
}

impl Solver {
    pub fn eliminate(
        &mut self,
        x: usize,
        y: usize,
        mut digits_to_eliminate: DigitMask,
    ) -> Result<()> {
        // Only eliminate digits that are possible in the square
        digits_to_eliminate &= self.board[x][y];

        // All digits to eliminate are eliminated
        if digits_to_eliminate == 0 {
            return Ok(());
        }

        // Eliminate the digits
        self.board[x][y] &= !digits_to_eliminate;
        let remaining_digits = self.board[x][y];
        if remaining_digits == 0 {
            return Err(eyre!("No possible digits left in ({}, {})", x, y));
        }

        let block_x = x / 3;
        let block_y = y / 3;
        let block_bit_index = (x % 3) + 3 * (y % 3);

        // Clear the reverse index
        iter_ones!(for bit_index in digits_to_eliminate => {
            // Clear columns
            self.possible_cols[x][bit_index] &= !(1 << y);
            if self.possible_cols[x][bit_index] == 0{
                return Err(eyre!("No possible digits left in col {}", x));
            }

            // Clear rows
            self.possible_rows[y][bit_index] &= !(1 << x);
            if self.possible_rows[y][bit_index] == 0{
                return Err(eyre!("No possible digits left in row {}", y));
            }

            // Clear blocks
            self.possible_blocks[block_x][block_y][bit_index] &= !(1 << block_bit_index);
            if self.possible_blocks[block_x][block_y][bit_index] == 0{
                return Err(eyre!(
                    "No possible digits left in block row {} col {}",
                    y,
                    x
                ));
            }
        });

        // If only one digit is left, eliminate that digit from all peers
        if remaining_digits.count_ones() == 1 {
            let remaining_digit_index = remaining_digits.trailing_zeros();

            // Get all positions where this digit is set in the current row, column and block,
            // and eliminate them from those squares.

            // Eliminate from row
            let mut remaining_position_mask: DigitMask =
                self.possible_rows[y][remaining_digit_index as usize];
            remaining_position_mask &= !(1 << x);
            iter_ones!(for bit_index in remaining_position_mask => {
                self.eliminate(bit_index, y, remaining_digits)?;
            });

            // Eliminate from column
            remaining_position_mask = self.possible_cols[x][remaining_digit_index as usize];
            remaining_position_mask &= !(1 << y);
            iter_ones!(for bit_index in remaining_position_mask => {
                self.eliminate(x, bit_index, remaining_digits)?;
            });

            // Eliminate from block
            remaining_position_mask =
                self.possible_blocks[block_x][block_y][remaining_digit_index as usize];
            remaining_position_mask &= !(1 << block_bit_index);
            iter_ones!(for bit_index in remaining_position_mask => {
                let x_offset = bit_index % 3;
                let y_offset = bit_index / 3;
                self.eliminate(
                    block_x * 3 + x_offset,
                    block_y * 3 + y_offset,
                    remaining_digits,
                )?;
            });
        }

        // For each digit we just eliminated, find if it now only has one remaining posible location
        // in either the row, column or block. If so, set the digit
        iter_ones!(for bit_index in digits_to_eliminate => {
            // Check row
            if self.possible_rows[y][bit_index].count_ones() == 1 {
                let digit_x = self.possible_rows[y][bit_index].trailing_zeros() as usize;
                self.set_digit(digit_x, y, bit_index)?;
            }

            // Check column
            if self.possible_cols[x][bit_index].count_ones() == 1 {
                let digit_y = self.possible_cols[x][bit_index].trailing_zeros() as usize;
                self.set_digit(x, digit_y, bit_index)?;
            }

            // Check column
            if self.possible_blocks[block_x][block_y][bit_index].count_ones() == 1 {
                let bit_with_digit = self.possible_blocks[block_x][block_y][bit_index]
                    .trailing_zeros() as usize;
                let x_offset = bit_with_digit % 3;
                let y_offset = bit_with_digit / 3;
                self.set_digit(block_x * 3 + x_offset, block_y * 3 + y_offset, bit_index)?;
            }
        });

        // While we're here, check if the square has a popcnt of 2, this means it's has the minimum
        // number of possibilities without being "done", making it an excellent candidate for the next
        // trial assignment.
        if self.board[x][y].count_ones() == 2 {
            self.next_square_candidate = Some((x, y));
        }

        Ok(())
    }

    pub fn set_digit(&mut self, x: usize, y: usize, digit: usize) -> Result<()> {
        let digit_mask = 1 << digit;
        self.board[x][y] |= digit_mask;
        self.eliminate(x, y, !digit_mask)
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
                let digit = self.board[x][y].trailing_zeros();
                for _x in 0..9 {
                    if _x != x && self.board[x][y].trailing_zeros() == digit {
                        return false;
                    }
                }

                for _y in 0..9 {
                    if _y != y && self.board[x][y].trailing_zeros() == digit {
                        return false;
                    }
                }

                let start_x = (x / 3) * 3;
                let start_y = (y / 3) * 3;
                let end_x = (x / 3 + 1) * 3;
                let end_y = (y / 3 + 1) * 3;

                for _x in (start_x..).take_while(|i| *i < end_x) {
                    for _y in (start_y..).take_while(|i| *i < end_y) {
                        if _x != x && _y != y && self.board[x][y].trailing_zeros() == digit {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

impl Debug for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter() {
            for cell in row.iter() {
                if cell.count_ones() == 1 {
                    let val = cell.trailing_zeros() + 1;
                    write!(f, "{}", val)?;
                } else {
                    write!(f, "x")?;
                }
            }
        }
        Ok(())
    }
}
