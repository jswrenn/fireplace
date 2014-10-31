pub enum ScaleMode {
    Fixed(f64, f64),
    Variable,
}

pub struct Program {
    pub data:Vec<f64>,
    pub title:Option<String>,
    pub scale:ScaleMode,
}

pub type Extremes = (f64, f64);

pub struct Frame<'a> {
    pub rows: i32,
    pub cols: i32,
    pub visible_data:&'a[f64],
    pub extremes:Extremes,
}
