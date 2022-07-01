pub mod board;
pub mod gui;
pub mod solver;
pub mod config;

use std::fmt::{Display, Write};

#[derive(Clone, Copy)]
pub enum UnicodeBox {
    Top,
    Bottom,
    Vertical,
    Horizontal,
}

impl Display for UnicodeBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodeBox::Top => {
                f.write_char('\u{256D}')?;
                f.write_str(&"\u{2500}".repeat(11))?;
                f.write_char('\u{256E}')?;
            }
            UnicodeBox::Bottom => {
                f.write_char('\u{2570}')?;
                f.write_str(&"\u{2500}".repeat(11))?;
                f.write_char('\u{256F}')?;
            }
            UnicodeBox::Vertical => {
                f.write_char('\u{2502}')?;
            }
            UnicodeBox::Horizontal => {
                f.write_char('\u{2502}')?;
                f.write_str(&"\u{2500}".repeat(3))?;
                f.write_char('\u{253c}')?;
                f.write_str(&"\u{2500}".repeat(3))?;
                f.write_char('\u{253c}')?;
                f.write_str(&"\u{2500}".repeat(3))?;
                f.write_char('\u{2502}')?;
            }
        };
        Ok(())
    }
}
