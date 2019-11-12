use wasm_bindgen::prelude::*;

use crate::types::Square;

/// A type representing the state of Mouse events
#[wasm_bindgen]
#[derive(Debug)]
pub struct Mouse {
    sq: Square,
    down: bool,
    up: bool,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            sq: Square::new(0.0, 0.0, 1.0),
            down: false,
            up: false,
        }
    }

    pub fn x(&self) -> f64 {
        self.sq.x()
    }
    pub fn y(&self) -> f64 {
        self.sq.y()
    }
    pub fn down(&self) -> bool {
        self.down
    }
    pub fn up(&self) -> bool {
        self.up
    }
    pub fn update(&mut self, x: f64, y: f64, mouse_down: bool, mouse_up: bool) {
        self.sq.set_pos(x, y);
        self.down = mouse_down;
        self.up = mouse_up;
    }
}
