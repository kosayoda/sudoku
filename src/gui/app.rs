use color_eyre::Result;

use druid::widget::{Button, Flex, Label, SizedBox};
use druid::{AppLauncher, LensExt, Widget, WidgetExt, WindowDesc};

use crate::gui::{Board, Grid, GridCell};

pub fn run() -> Result<()> {
    let board_str =
        "000075400000000008080190000300001060000000034000068170204000603900000020530200000";
    let board: Board = board_str.try_into()?;

    let window = WindowDesc::new(Grid::new()).resizable(true);
    AppLauncher::with_window(window).launch(board)?;
    Ok(())
}
