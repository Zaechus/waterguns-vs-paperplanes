use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::types::Square;

#[wasm_bindgen]
pub struct Button {
    x: f64,
    y: f64,
    size: f64,
    img: String,
}

impl Button {
    pub fn new(square: Square, img: &str) -> Self {
        Self {
            x: square.x,
            y: square.y,
            size: square.size,
            img: String::from(img),
        }
    }

    pub fn draw(
        &self,
        ctx: &CanvasRenderingContext2d,
        sprites: &HashMap<String, HtmlImageElement>,
    ) -> Result<(), JsValue> {
        ctx.draw_image_with_html_image_element_and_dw_and_dh(
            sprites.get(&self.img).unwrap(),
            self.x,
            self.y,
            self.size,
            self.size,
        )
    }
}
