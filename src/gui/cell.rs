use std::ops::Deref;

use druid::{
    widget::Label, BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget,
};
use tracing::debug;

#[derive(Data, Clone, Copy, Debug, Default)]
pub struct Value(pub Option<usize>);

impl Deref for Value {
    type Target = Option<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Cell {
    pub label: Label<Value>,
}

impl Cell {
    pub fn new() -> Self {
        let label = Label::dynamic(|c: &Value, _env| {
            if let Some(val) = c.0 {
                format!("{}", val)
            } else {
                String::new()
            }
        })
        .with_text_color(Color::BLACK);
        Self { label }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget<Value> for Cell {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Value, env: &Env) {
        self.label.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Value, env: &Env) {
        self.label.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Value, data: &Value, env: &Env) {
        self.label.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Value,
        env: &Env,
    ) -> Size {
        self.label.set_text_size(bc.max().min_side() * 0.5);
        self.label.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Value, env: &Env) {
        self.label.paint(ctx, data, env);
    }
}
