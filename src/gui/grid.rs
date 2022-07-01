use druid::{
    widget::{BackgroundBrush, Container, Flex, Widget, WidgetExt},
    BoxConstraints, Env, Event, EventCtx, KbKey, KeyEvent, LayoutCtx, LensExt, LifeCycle,
    LifeCycleCtx, PaintCtx, Size, UpdateCtx, WidgetId,
};
use tracing::debug;

use crate::{
    config::Config,
    gui::{AppData, Board, Cell, CellValue, Keys},
};

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
            let mut row = Flex::row();

            if y % 3 == 0 && y != 0 {
                column.add_flex_spacer(BLOCK_SPACER_FLEX);
            }

            for x in 0..9 {
                if x % 3 == 0 && x != 0 {
                    row.add_flex_spacer(BLOCK_SPACER_FLEX);
                }

                row.add_flex_child(
                    GridCell::new(x, y, cfg).lens(
                        AppData::board.then(Board::cells.as_ref().index(x).as_ref().index(y)),
                    ),
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
        let mut side = bc.max().min_side();
        side = side - (side % TOTAL_FLEX);
        let size = Size::new(side, side);
        let constraints = BoxConstraints::new(size, size);
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

pub struct GridCell {
    cell: Container<CellValue>,
}

impl GridCell {
    pub fn new(x: usize, y: usize, cfg: &Config) -> Self {
        let id: u16 = (y * 9 + x)
            .try_into()
            .expect("Cannot assign u16 id to a grid cell!");
        let id = WidgetId::reserved(id);
        Self {
            cell: Cell::new(&cfg.theme)
                .with_id(id)
                .center()
                .border(Keys::THEME_CELL_BORDER, Keys::CELL_BORDER_WIDTH),
        }
    }

    fn set_background_color(&mut self, value: CellValue, focused: bool, env: &Env) {
        let color = match (value.is_fixed(), focused) {
            (true, true) => {
                unreachable!("Unexpected: Grid cell is fixed but focused at the same time!")
            }
            (true, false) => env.get(Keys::THEME_CELL_BG_FIXED),
            (false, true) => env.get(Keys::THEME_CELL_BG_FOCUSED),
            (false, false) => env.get(Keys::THEME_CELL_BG),
        };

        self.cell.set_background(BackgroundBrush::from(color));
    }
}

impl Widget<CellValue> for GridCell {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CellValue, env: &Env) {
        if (*data).is_fixed() {
            return self.cell.event(ctx, event, data, env);
        }

        match event {
            Event::KeyDown(KeyEvent { key, .. }) => match key {
                KbKey::Character(c) => {
                    debug!("Character pressed: {c}");
                    let valid_value: Option<usize> = c
                        .chars()
                        .last()
                        .and_then(|c| c.to_digit(10))
                        .and_then(|n| usize::try_from(n).ok())
                        .filter(|&n| (1..=9).contains(&n));

                    if let Some(num) = valid_value {
                        debug!("Cell set: {num}, was {data}");
                        *data = CellValue::User(Some(num));
                    }
                }
                KbKey::Backspace | KbKey::Delete => {
                    debug!("Cell cleared, was {data}");
                    *data = CellValue::User(None);
                }
                _ => {}
            },
            Event::MouseDown(_) => {
                debug!("Cell clicked, toggling focus");
                if ctx.has_focus() {
                    ctx.resign_focus();
                } else {
                    ctx.request_focus();
                }
            }
            _ => {}
        };

        self.cell.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &CellValue,
        env: &Env,
    ) {
        match event {
            LifeCycle::WidgetAdded => {
                ctx.register_for_focus();
                self.set_background_color(*data, false, env);
            }
            LifeCycle::FocusChanged(focused) => {
                self.set_background_color(*data, *focused, env);
                ctx.request_paint();
            }
            _ => {}
        }

        self.cell.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &CellValue, data: &CellValue, env: &Env) {
        self.set_background_color(*data, ctx.has_focus(), env);
        self.cell.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &CellValue,
        env: &Env,
    ) -> Size {
        // Ensure that the GridCell remains a square by taking the smaller of the two side lengths.
        let size = bc.max().min_side().round().min(bc.max().max_side());
        let constraints = bc.shrink_max_width_to(size).shrink_max_height_to(size);
        self.cell.layout(ctx, &constraints, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &CellValue, env: &Env) {
        self.cell.paint(ctx, data, env);
    }
}
