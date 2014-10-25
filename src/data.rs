
pub enum ScaleType {
    Linear,
    Logarithmic,
}

type MinValue = f64;
type MaxValue = f64;

pub enum ScaleMode {
    Fixed(f64, f64),
    Variable,
}

pub struct Program {
    pub data:Vec<f64>,
    pub title:Option<String>,
    pub scale:ScaleMode,
}
