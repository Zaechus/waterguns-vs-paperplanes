use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Mouse {
    pub x: f64,
    pub y: f64,
    pub down: bool,
    pub up: bool,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            down: false,
            up: false,
        }
    }

    pub fn update(&mut self, x: f64, y: f64, mouse_down: bool, mouse_up: bool) {
        self.x = x;
        self.y = y;
        self.down = mouse_down;
        self.up = mouse_up;
    }
}
