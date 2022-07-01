use color_eyre::{eyre::eyre, Report, Result};
use druid::{Data, Lens};

use crate::{gui::CellValue, solver::Solver};

#[derive(Clone, Copy, Data, Default, Lens)]
pub struct Board {
    pub cells: [[CellValue; 9]; 9],
}

impl TryFrom<&str> for Board {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self> {
        let mut cells = [[CellValue::default(); 9]; 9];

        let length = value.chars().count();
        if length != 81 {
            return Err(eyre!("Invalid board representation length: {length}"));
        }

        for (index, cell) in value.chars().enumerate() {
            let x = index.rem_euclid(9);
            let y = index.div_euclid(9);

            if let Some(Ok(pos)) = cell.to_digit(10).map(usize::try_from) {
                if pos > 0 {
                    cells[x][y] = CellValue::Fixed(pos);
                }
            } else {
                return Err(eyre!("Invalid cell '{cell}' at index: {index}"));
            }
        }

        Ok(Board { cells })
    }
}

impl From<&Board> for Solver {
    fn from(value: &Board) -> Self {
        let board = [[0x1ff; 9]; 9];

        let possible_blocks = [[[0x1ff; 9]; 3]; 3];
        let possible_rows = [[0x1ff; 9]; 9];
        let possible_cols = [[0x1ff; 9]; 9];

        let mut solver = Solver {
            board,
            possible_blocks,
            possible_rows,
            possible_cols,
            next_square_candidate: None,
        };

        for (y, row) in (*value).cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match *cell {
                    CellValue::Fixed(val) | CellValue::User(Some(val)) => {
                        solver
                            .set_digit(x, y, val - 1)
                            .expect("Board setup should set digit without failure.");
                    }
                    _ => (),
                }
            }
        }
        solver
    }
}
