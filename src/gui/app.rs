use std::sync::Arc;

use color_eyre::Result;

use druid::{AppLauncher, Data, Env, Event, EventCtx, Lens, Widget, WidgetExt, WindowDesc};

use crate::config::Config;
use crate::gui::{Board, Grid};

pub fn run() -> Result<()> {
    let board_str =
        "000075400000000008080190000300001060000000034000068170204000603900000020530200000";
    let board: Board = board_str.try_into()?;
    let data = AppData::new(board);

    let app = App::new(&data).center();
    let window = WindowDesc::new(app).resizable(true);
    AppLauncher::with_window(window).launch(data)?;

    Ok(())
}

#[derive(Clone, Data, Default, Lens)]
pub struct AppData {
    pub board: Board,
    pub config: Arc<Config>,
}

impl AppData {
    fn new(board: Board) -> Self {
        Self {
            board,
            ..Default::default()
        }
    }
}

struct App {
    grid: Grid,
}

impl App {
    fn new(data: &AppData) -> Self {
        Self {
            grid: Grid::new(data.config.gui.grid.block_spacer_width),
        }
    }
}

impl Widget<AppData> for App {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        self.grid.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &AppData,
        env: &Env,
    ) {
        self.grid.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &AppData,
        data: &AppData,
        env: &Env,
    ) {
        self.grid.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &AppData,
        env: &Env,
    ) -> druid::Size {
        self.grid.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppData, env: &Env) {
        self.grid.paint(ctx, data, env);
    }
}
