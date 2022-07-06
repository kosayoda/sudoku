use druid::{
    widget::{BackgroundBrush, Container, Flex, Widget, WidgetExt},
    BoxConstraints, Env, Event, EventCtx, KbKey, KeyEvent, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Size, UpdateCtx, WidgetId,
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

        // Round the grid size further to be an odd number
        if size % 2.0 == 0.0 {
            size -= 1.0;
        }
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

#[derive(Clone, Copy, Debug)]
pub struct CellPosition {
    x: u16,
    y: u16,
}

impl CellPosition {
    fn to_widget_id(self) -> WidgetId {
        WidgetId::reserved(self.to_absolute())
    }
    fn to_absolute(self) -> u16 {
        self.y * 9 + self.x
    }
}

pub struct GridCell {
    cell: Container<CellValue>,
    position: CellPosition,
}

impl GridCell {
    pub fn new(position: CellPosition, cfg: &Config) -> Self {
        Self {
            cell: Cell::new(&cfg.theme)
                .center()
                .border(Keys::THEME_CELL_BORDER, Keys::CELL_BORDER_WIDTH),
            position,
        }
    }

    fn set_background_color(&mut self, value: &CellValue, focused: bool, env: &Env) {
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

    fn get_cell(&self, board: &Board) -> CellValue {
        self.get_cell_at_pos(board, &self.position)
    }

    fn get_cell_mut<'a>(&self, board: &'a mut Board) -> &'a mut CellValue {
        self.get_cell_at_pos_mut(board, &self.position)
    }

    fn get_cell_at_pos(&self, board: &Board, pos: &CellPosition) -> CellValue {
        board.cells[usize::from(pos.x)][usize::from(pos.y)]
    }

    fn get_cell_at_pos_mut<'a>(
        &self,
        board: &'a mut Board,
        pos: &CellPosition,
    ) -> &'a mut CellValue {
        &mut board.cells[usize::from(pos.x)][usize::from(pos.y)]
    }

    fn shift_focus(&self, key: &druid::KbKey, ctx: &mut EventCtx, data: &mut Board) {
        let x_pos: usize = self.position.x.into();
        let y_pos: usize = self.position.y.into();

        let x: u16 = match key {
            KbKey::ArrowRight => (x_pos + 1..9)
                .find(|_x| !data.cells[*_x][y_pos].is_fixed())
                .unwrap_or(x_pos),
            KbKey::ArrowLeft => (0..x_pos)
                .rev()
                .find(|_x| !data.cells[*_x][y_pos].is_fixed())
                .unwrap_or(x_pos),
            _ => x_pos,
        }
        .try_into()
        .unwrap();
        let y: u16 = match key {
            KbKey::ArrowUp => (0..y_pos)
                .rev()
                .find(|_y| !data.cells[x_pos][*_y].is_fixed())
                .unwrap_or(y_pos),
            KbKey::ArrowDown => (y_pos + 1..9)
                .find(|_y| !data.cells[x_pos][*_y].is_fixed())
                .unwrap_or(y_pos),
            _ => y_pos,
        }
        .try_into()
        .unwrap();
        ctx.set_focus(WidgetId::reserved(y * 9 + x));
    }
}

impl Widget<Board> for GridCell {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Board, env: &Env) {
        let mut value = *self.get_cell_mut(data);
        if value.is_fixed() {
            return self.cell.event(ctx, event, &mut value, env);
        }

        match event {
            Event::KeyDown(KeyEvent { key, .. }) => match key {
                KbKey::Character(c) => {
                    debug!("Character pressed: {c}");
                    let valid_value: Option<u16> = c
                        .chars()
                        .last()
                        .and_then(|c| c.to_digit(10))
                        .and_then(|n| u16::try_from(n).ok())
                        .filter(|&n| (1..=9).contains(&n));

                    if let Some(num) = valid_value {
                        debug!("Cell set: {num}, was {}", self.get_cell(data));
                        *self.get_cell_mut(data) = CellValue::Unfixed(Some(num));
                    }
                }
                KbKey::Backspace | KbKey::Delete => {
                    debug!("Cell cleared, was {}", self.get_cell(data));
                    *self.get_cell_mut(data) = CellValue::Unfixed(None);
                }
                // Move focus between cells
                KbKey::ArrowLeft | KbKey::ArrowRight | KbKey::ArrowUp | KbKey::ArrowDown => {
                    if ctx.is_focused() {
                        self.shift_focus(key, ctx, data);
                    }
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

        self.cell.event(ctx, event, &mut value, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Board, env: &Env) {
        let value = self.get_cell(data);
        match event {
            LifeCycle::WidgetAdded => {
                ctx.register_for_focus();
                self.set_background_color(&value, false, env);
            }
            LifeCycle::FocusChanged(focused) => {
                debug!("Focus is now on: {:?}", ctx.widget_id());
                self.set_background_color(&value, *focused, env);
                ctx.request_paint();
            }
            _ => {}
        }

        self.cell.lifecycle(ctx, event, &value, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Board, data: &Board, env: &Env) {
        let old_value = self.get_cell(old_data);
        let value = self.get_cell(data);

        self.set_background_color(&value, ctx.has_focus(), env);
        self.cell.update(ctx, &old_value, &value, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Board,
        env: &Env,
    ) -> Size {
        // Ensure that the GridCell remains a square by taking the average of the two side lengths;
        let mut size = bc.max().min_side();
        debug!("{:?}: {}", self.position, size);

        // Round to the nearest multiple of 2 to ensure different squares are the same size
        size = (size - size % 3.0).round();

        // Round the size further to be an odd number
        if size % 2.0 == 0.0 {
            size -= 1.0;
        }

        let constraints = bc.shrink_max_width_to(size).shrink_max_height_to(size);
        debug!("{:?}", constraints);

        let value = self.get_cell(data);
        self.cell.layout(ctx, &constraints, &value, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Board, env: &Env) {
        let value = self.get_cell(data);
        self.cell.paint(ctx, &value, env);
    }
}
