use wasm_bindgen::prelude::*;

/// A sub-type of entities used for a logical way to apply construction of new entities
#[wasm_bindgen]
#[derive(Debug)]
pub struct Square {
    x: f64,
    y: f64,
    size: f64,
    center_x: f64,
    center_y: f64,
}

impl Square {
    /// Constuct a new Square from x, y, and size
    pub fn new(x: f64, y: f64, size: f64) -> Self {
        Self {
            x,
            y,
            size,
            center_x: x + size * 0.5,
            center_y: y + size * 0.5,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn size(&self) -> f64 {
        self.size
    }
    pub fn center_x(&self) -> f64 {
        self.center_x
    }
    pub fn center_y(&self) -> f64 {
        self.center_y
    }

    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.x = x;
        self.center_x = x + self.size * 0.5;
        self.y = y;
        self.center_y = y + self.size * 0.5;
    }
}
