use druid::{
    widget::{BackgroundBrush, Container, Flex, Label, Widget, WidgetExt},
    BoxConstraints, Color, Env, Event, EventCtx, KbKey, KeyEvent, LayoutCtx, LensExt, LifeCycle,
    LifeCycleCtx, PaintCtx, Size, UpdateCtx,
};
use tracing::debug;

use crate::gui::{Board, Cell, Value};

pub struct Grid {
    display: Container<Board>,
}

impl Grid {
    pub fn new() -> Self {
        const SPACER: f64 = 0.02;

        let mut column = Flex::column();
        for y in 0..9 {
            let mut row = Flex::row();

            for x in 0..9 {
                if x % 3 == 0 && x != 0 {
                    row.add_flex_spacer(SPACER);
                }

                row.add_flex_child(
                    GridCell::new().lens(Board::cells.as_ref().index(x).as_ref().index(y)),
                    1.0,
                );
            }

            if y % 3 == 0 && y != 0 {
                column.add_flex_spacer(SPACER);
            }

            column.add_flex_child(row, 1.0);
        }
        Self {
            display: column
                .center()
                .background(Color::BLACK)
                .border(Color::grey(0.5), 0.5),
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget<Board> for Grid {
    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Board,
        env: &Env,
    ) -> Size {
        let side = bc.max().min_side();
        let size = Size::new(side, side);
        let constraints = BoxConstraints::new(size, size);
        self.display.layout(ctx, &constraints, data, env)
    }

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Board, env: &Env) {
        self.display.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Board, env: &Env) {
        self.display.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Board, data: &Board, env: &Env) {
        self.display.update(ctx, old_data, data, env);
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Board, env: &Env) {
        self.display.paint(ctx, data, env);
    }
}

pub struct GridCell {
    cell: Container<Value>,
}

impl GridCell {
    pub fn new() -> Self {
        Self {
            cell: Cell::new().center().border(Color::grey(0.5), 0.5),
        }
    }

    fn set_background_color(&mut self, focused: bool) {
        const WHITE: Color = Color::rgb8(241, 247, 255);
        const BLUE: Color = Color::rgb8(97, 158, 239);
        // const GREEN: Color = Color::rgb8(149, 190, 147);
        // const RED: Color = Color::rgb8(239, 90, 112);

        let background: BackgroundBrush<_> = if focused { BLUE.into() } else { WHITE.into() };
        self.cell.set_background(background);
    }
}

impl Default for GridCell {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget<Value> for GridCell {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Value, env: &Env) {
        match event {
            Event::KeyDown(KeyEvent { key, .. }) => match key {
                KbKey::Character(c) => {
                    debug!("Character pressed: {c}");
                    let press: Option<usize> = c
                        .chars()
                        .last()
                        .and_then(|c| c.to_digit(10))
                        .and_then(|n| usize::try_from(n).ok())
                        .filter(|&n| (1..=9).contains(&n));

                    if let Some(num) = press {
                        debug!("Cell set: {num}, was {data:?}");
                        data.0 = Some(num);
                    }
                }
                KbKey::Backspace | KbKey::Delete => {
                    debug!("Cell cleared, was {data:?}");
                    data.0 = None;
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

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Value, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                ctx.register_for_focus();
                self.set_background_color(false);
            }
            LifeCycle::FocusChanged(focused) => {
                self.set_background_color(*focused);
                ctx.request_paint();
            }
            _ => {}
        }

        self.cell.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Value, data: &Value, env: &Env) {
        self.set_background_color(ctx.has_focus());
        ctx.request_paint();
        // self.update_label();
        self.cell.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Value,
        env: &Env,
    ) -> Size {
        self.cell.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Value, env: &Env) {
        self.cell.paint(ctx, data, env);
    }
}
