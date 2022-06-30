use druid::Data;

pub type Value = usize;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct Cell {
    pub value: Option<Value>,
}
