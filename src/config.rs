use druid::Color;
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub gui: GuiConfig,
    pub theme: ThemeConfig,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GuiConfig {
    /// The border width of the each cell in pixels
    pub cell_border_width: f64,
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            cell_border_width: 1.0,
        }
    }
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
            bg: Color::from_hex_str("#26272b").unwrap(),
            grid_bg: Color::from_hex_str("#26272b").unwrap(),
            cell_fg: Color::from_hex_str("#26272b").unwrap(),
            cell_bg: Color::from_hex_str("#f1f7ff").unwrap(),
            cell_bg_focused: Color::from_hex_str("#96b8e8").unwrap(),
            cell_bg_fixed: Color::from_hex_str("#dedfe3").unwrap(),
            cell_border: Color::from_hex_str("#4f4f59").unwrap(),
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
