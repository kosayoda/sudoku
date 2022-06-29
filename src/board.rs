use std::fmt::Display;

use color_eyre::{eyre::eyre, Report, Result};
use owo_colors::OwoColorize;
use thiserror::Error;

use crate::UnicodeBox;

#[derive(Error, Debug)]
pub enum BoardError {
    #[error("Invalid board length, expected 81, got {0}")]
    InvalidLength(usize),
    #[error("Invalid board cell at index {index}, expected a number between 0-9, got '{cell}'")]
    InvalidCell { index: usize, cell: char },
}

#[derive(PartialEq, Eq)]
pub struct Board {
    squares: [[usize; 9]; 9],
}

impl Board {
    pub fn squares(&self) -> [[usize; 9]; 9] {
        self.squares
    }
}

impl TryFrom<&str> for Board {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self> {
        let mut squares = [[0x1ff_usize; 9]; 9];

        let length = value.chars().count();
        if length != 81 {
            return Err(eyre!(BoardError::InvalidLength(length)));
        }
        for (index, cell) in value.chars().enumerate() {
            let x = index.rem_euclid(9);
            let y = index.div_euclid(9);

            if let Some(pos) = cell.to_digit(10) {
                squares[y][x] = pos as usize;
            } else {
                return Err(eyre!(BoardError::InvalidCell { index, cell }));
            }
        }

        Ok(Board { squares })
    }
}

impl Display for Board {
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

                let cell = self.squares[y - 1][x];
                if cell == 0 {
                    write!(f, " ")?;
                } else {
                    write!(f, "{}", cell.green())?;
                }
            }
            writeln!(f, "{}", UnicodeBox::Vertical)?;
        }

        write!(f, "{}", UnicodeBox::Bottom)
    }
}
