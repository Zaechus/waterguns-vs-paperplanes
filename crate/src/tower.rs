use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Tower {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Tower {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
}
