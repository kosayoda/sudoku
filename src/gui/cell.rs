use std::fmt::Display;

use druid::{
    widget::Label, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Size, UpdateCtx, Widget,
};
use tracing::debug;

use crate::config::ThemeConfig;

// pub struct Unfixed {
//     value: Option<usize>,
//     candidates:
// }

#[derive(Data, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellValue {
    Fixed(u16),
    Unfixed(Option<u16>),
}

impl CellValue {
    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }
}

impl Default for CellValue {
    fn default() -> Self {
        Self::Unfixed(None)
    }
}

impl From<CellValue> for Option<u16> {
    fn from(cell: CellValue) -> Self {
        match cell {
            CellValue::Fixed(val) => Some(val),
            CellValue::Unfixed(val) => val,
        }
    }
}

impl Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellValue::Fixed(val) => {
                write!(f, "{val}")
            }
            CellValue::Unfixed(Some(val)) => {
                write!(f, "{val}")
            }
            CellValue::Unfixed(None) => {
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
        let mut size = bc.max().min_side();

        // Round the size further to be an odd number
        if size % 2.0 == 0.0 {
            size -= 1.0;
        }
        let ss = Size::new(size, size);
        let constraints = BoxConstraints::new(ss, ss);

        self.label.set_text_size(size * 0.5);
        self.label.layout(ctx, &constraints, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &CellValue, env: &Env) {
        self.label.paint(ctx, data, env);
    }
}
