use std::sync::Arc;

use color_eyre::Result;
use druid::{AppLauncher, Data, Env, Event, EventCtx, Lens, Widget, WidgetExt, WindowDesc};

use crate::config::Config;
use crate::gui::{Board, Grid, Keys};

macro_rules! initialize_keys {
    ($env:ident; $($key_var:ident = $key_path:expr),*) => {
        $($env.set(Keys::$key_var, $key_path);)*
    }
}

pub fn run() -> Result<()> {
    let board_str =
        "000075400000000008080190000300001060000000034000068170204000603900000020530200000";
    let board: Board = board_str.try_into()?;
    let config = Arc::new(Config::default());

    let data = AppData::new(board, config.clone());
    let app = App::new(&data).center().background(config.theme.bg.clone());
    let window = WindowDesc::new(app).resizable(true);

    AppLauncher::with_window(window)
        .configure_env(move |env: &mut Env, _| {
            initialize_keys! {
                env;
                CELL_BORDER_WIDTH = config.gui.cell_border_width,
                THEME_BG = config.theme.bg.clone(),
                THEME_GRID_BG = config.theme.grid_bg.clone(),
                THEME_CELL_FG = config.theme.cell_fg.clone(),
                THEME_CELL_BG = config.theme.cell_bg.clone(),
                THEME_CELL_BG_FOCUSED = config.theme.cell_bg_focused.clone(),
                THEME_CELL_BG_FIXED = config.theme.cell_bg_fixed.clone(),
                THEME_CELL_BORDER = config.theme.cell_border.clone()
            }
        })
        .launch(data)?;
    Ok(())
}

#[derive(Clone, Data, Default, Lens)]
pub struct AppData {
    pub board: Board,
    pub config: Arc<Config>,
}

impl AppData {
    fn new(board: Board, config: Arc<Config>) -> Self {
        Self { board, config }
    }
}

struct App {
    grid: Grid,
}

impl App {
    fn new(data: &AppData) -> Self {
        Self {
            grid: Grid::new(&data.config),
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
