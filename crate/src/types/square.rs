use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Square {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

impl Square {
    pub fn new(x: f64, y: f64, size: f64) -> Self {
        Self { x, y, size }
    }
}
