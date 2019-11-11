use wasm_bindgen::prelude::*;

/// A sub-type of entities used for a logical way to apply construction of new entities
#[wasm_bindgen]
pub struct Square {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

impl Square {
    /// Constuct a new Square from x, y, and size
    pub fn new(x: f64, y: f64, size: f64) -> Self {
        Self { x, y, size }
    }

    pub fn center_x(&self) -> f64 {
        self.x + self.size * 0.5
    }
    pub fn center_y(&self) -> f64 {
        self.y + self.size * 0.5
    }
}
