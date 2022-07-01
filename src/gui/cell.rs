use std::fmt::Display;

use druid::{
    widget::Label, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Size, UpdateCtx, Widget,
};

use crate::config::ThemeConfig;

#[derive(Data, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellValue {
    Fixed(usize),
    User(Option<usize>),
}

impl CellValue {
    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }
}

impl Default for CellValue {
    fn default() -> Self {
        Self::User(None)
    }
}

impl From<CellValue> for Option<usize> {
    fn from(cell: CellValue) -> Self {
        match cell {
            CellValue::Fixed(val) => Some(val),
            CellValue::User(val) => val,
        }
    }
}

impl Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellValue::Fixed(val) => {
                write!(f, "{val}")
            }
            CellValue::User(Some(val)) => {
                write!(f, "{val}")
            }
            CellValue::User(None) => {
                write!(f, "")
            }
        }
    }
}

pub struct Cell {
    pub label: Label<CellValue>,
}

impl Cell {
    pub fn new(theme_cfg: &ThemeConfig) -> Self {
        let label = Label::dynamic(|c: &CellValue, _env| format!("{}", *c))
            .with_text_color(theme_cfg.cell_fg.clone());
        Self { label }
    }
}

impl Widget<CellValue> for Cell {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CellValue, env: &Env) {
        self.label.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &CellValue,
        env: &Env,
    ) {
        self.label.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &CellValue, data: &CellValue, env: &Env) {
        self.label.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &CellValue,
        env: &Env,
    ) -> Size {
        let size = bc.max().min_side();
        self.label.set_text_size(size * 0.5);
        self.label.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &CellValue, env: &Env) {
        self.label.paint(ctx, data, env);
    }
}
