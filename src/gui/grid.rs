use druid::{
    widget::{BackgroundBrush, Container, Flex, Widget, WidgetExt},
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx,
};
use tracing::debug;

use super::{AppData, CellPosition, GridCell, Keys};
use crate::config::Config;

const CELL_FLEX: f64 = 1.0;
const BLOCK_SPACER_FLEX: f64 = 0.05;
const TOTAL_FLEX: f64 = CELL_FLEX * 9.0 + BLOCK_SPACER_FLEX * 2.0;

pub struct Grid {
    display: Container<AppData>,
}

impl Grid {
    pub fn new(cfg: &Config) -> Self {
        let mut column = Flex::column();
        for y in 0..9 {
            if y % 3 == 0 && y != 0 {
                column.add_flex_spacer(BLOCK_SPACER_FLEX);
            }

            let mut row = Flex::row();
            for x in 0..9 {
                if x % 3 == 0 && x != 0 {
                    row.add_flex_spacer(BLOCK_SPACER_FLEX);
                }

                let position = CellPosition { x, y };
                let cell = GridCell::new(position, cfg);
                row.add_flex_child(
                    cell.with_id(position.to_widget_id()).lens(AppData::board),
                    CELL_FLEX,
                );
            }
            column.add_flex_child(row, CELL_FLEX);
        }

        Self {
            display: column
                .center()
                .background(BackgroundBrush::ColorKey(Keys::THEME_GRID_BG)),
        }
    }
}

impl Widget<AppData> for Grid {
    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppData,
        env: &Env,
    ) -> Size {
        // Round the grid size to be a multiple of the flex size
        let mut size = bc.max().min_side();
        size = (size - (size % TOTAL_FLEX)).round();

        let constraints = bc.shrink_max_width_to(size).shrink_max_height_to(size);
        debug!("Grid constraints: {constraints:?}");
        self.display.layout(ctx, &constraints, data, env)
    }

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        self.display.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppData, env: &Env) {
        self.display.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppData, data: &AppData, env: &Env) {
        self.display.update(ctx, old_data, data, env);
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, env: &Env) {
        self.display.paint(ctx, data, env);
    }
}
