use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::types::{Selected, Square};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Button {
    x: f64,
    y: f64,
    size: f64,
    select: Selected,
    content: String,
}

impl Button {
    pub fn new(square: Square, select: Selected, content: &str) -> Self {
        Self {
            x: square.x,
            y: square.y,
            size: square.size,
            select,
            content: String::from(content),
        }
    }

    pub fn draw(
        &self,
        ctx: &CanvasRenderingContext2d,
        sprites: &HashMap<String, HtmlImageElement>,
    ) -> Result<(), JsValue> {
        if let Some(img) = sprites.get(&self.content) {
            ctx.draw_image_with_html_image_element_and_dw_and_dh(
                img, self.x, self.y, self.size, self.size,
            )?;
            Ok(())
        } else {
            ctx.begin_path();
            ctx.set_fill_style(&JsValue::from_str("#222222"));
            ctx.rect(self.x, self.y, self.size, self.size * 0.5);
            ctx.fill();
            ctx.set_fill_style(&JsValue::from_str("#00ff00"));
            ctx.set_font(&format!("{}px monospace", self.size * 0.2));
            ctx.fill_text(
                &self.content,
                self.x + self.size * 0.07,
                self.y + self.size * 0.3,
            )?;
            ctx.close_path();
            Ok(())
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

    pub fn select(&self) -> Selected {
        self.select
    }
}
