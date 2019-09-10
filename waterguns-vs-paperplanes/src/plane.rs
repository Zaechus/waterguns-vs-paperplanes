use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PaperPlane {
    x: i32,
}

#[wasm_bindgen]
impl PaperPlane {
    pub fn new(x: i32) -> Self {
        Self { x }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn fly(&mut self) {
        self.x += 1;
    }
}
