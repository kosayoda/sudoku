use druid::{Color, Key};
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub gui: GuiConfig,
    pub theme: ThemeConfig,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GuiConfig {
    pub grid: GridConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GridConfig {
    /// The border width of the each cell in pixels
    pub cell_border_width: f64,
    /// The relative width of block spacers to cell width
    pub block_spacer_width: f64,
    #[serde(skip)]
    #[serde(default = "default_cell_border_width_key")]
    pub cell_border_width_key: Key<f64>,
    #[serde(skip)]
    #[serde(default = "default_block_spacer_width_key")]
    pub block_spacer_width_key: Key<f64>,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            cell_border_width: 1.0,
            cell_border_width_key: default_cell_border_width_key(),
            block_spacer_width: 0.05,
            block_spacer_width_key: default_block_spacer_width_key(),
        }
    }
}

fn default_cell_border_width_key() -> Key<f64> {
    Key::new("ukodus.grid.cell-border-width")
}

fn default_block_spacer_width_key() -> Key<f64> {
    Key::new("ukodus.grid.block-spacer-width")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThemeConfig {
    /// The background color of the application
    #[serde(with = "ser_color")]
    pub bg: Color,
    /// The background color of the sudoku grid
    #[serde(with = "ser_color")]
    pub grid_bg: Color,
    /// The text color of cells
    #[serde(with = "ser_color")]
    pub cell_fg: Color,
    /// The background color of cells
    #[serde(with = "ser_color")]
    pub cell_bg: Color,
    /// The background color of the focused cell
    #[serde(with = "ser_color")]
    pub cell_bg_focused: Color,
    /// The background color of the fixed cells
    #[serde(with = "ser_color")]
    pub cell_bg_fixed: Color,
    /// The border color of cells
    #[serde(with = "ser_color")]
    pub cell_border: Color,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            bg: Color::BLACK,
            grid_bg: Color::from_hex_str("#26272b").unwrap(),
            cell_fg: Color::from_hex_str("#26272b").unwrap(),
            cell_bg: Color::from_hex_str("#d9e9ff").unwrap(),
            cell_bg_focused: Color::from_hex_str("#96b8e8").unwrap(),
            cell_bg_fixed: Color::from_hex_str("#d0d2d5").unwrap(),
            cell_border: Color::from_hex_str("#d0d2d5").unwrap(),
        }
    }
}

mod ser_color {
    use druid::Color;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Color::from_hex_str(&s).map_err(serde::de::Error::custom)
    }

    pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (r, g, b, a) = color.as_rgba8();
        let s = format!("0x{:x}{:x}{:x}{:x}", r, g, b, a);
        serializer.serialize_str(&s)
    }
}
