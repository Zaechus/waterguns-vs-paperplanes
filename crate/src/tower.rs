use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Tower {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
impl Tower {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
}
