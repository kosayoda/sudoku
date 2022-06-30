use color_eyre::{eyre::eyre, Report, Result};
use druid::{Data, Lens};
use tracing::debug;

use crate::gui::CellValue;

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
