use druid::{Color, Key};

pub struct Keys;

macro_rules! define_keys {
    ($($key_var:ident: $key_type: ident = $key_name:literal),*) => {
        impl Keys {
        $(pub const $key_var: Key<$key_type> = Key::new($key_name);)*
        }
    }
}
define_keys! {
    CELL_BORDER_WIDTH: f64 = "dev.config.gui.cell-border-width",
    THEME_BG: Color = "dev.config.theme.bg",
    THEME_GRID_BG: Color = "dev.config.theme.grid-bg",
    THEME_CELL_FG: Color = "dev.config.theme.cell-fg",
    THEME_CELL_BG: Color = "dev.config.theme.cell-bg",
    THEME_CELL_BG_FOCUSED: Color = "dev.config.theme.cell-bg-focused",
    THEME_CELL_BG_FIXED: Color = "dev.config.theme.cell-bg-fixed",
    THEME_CELL_BORDER: Color = "dev.config.theme.cell-border"
}
