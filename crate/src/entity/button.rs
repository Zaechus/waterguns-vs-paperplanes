use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::types::{Selected, Square};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Button {
    sq: Square,
    select: Selected,
    content: String,
}

impl Button {
    pub fn new(sq: Square, select: Selected, content: &str) -> Self {
        Self {
            sq,
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
                img,
                self.sq.x(),
                self.sq.y(),
                self.sq.size(),
                self.sq.size(),
            )?;
            Ok(())
        } else {
            ctx.begin_path();
            ctx.set_fill_style(&JsValue::from_str("#222222"));
            ctx.rect(
                self.sq.x(),
                self.sq.y(),
                self.sq.size(),
                self.sq.size() * 0.5,
            );
            ctx.fill();
            ctx.set_fill_style(&JsValue::from_str("#00ff00"));
            ctx.set_font(&format!("{}px monospace", self.sq.size() * 0.2));
            ctx.fill_text(
                &self.content,
                self.sq.x() + self.sq.size() * 0.07,
                self.sq.y() + self.sq.size() * 0.3,
            )?;
            ctx.close_path();
            Ok(())
        }
    }

    pub fn x(&self) -> f64 {
        self.sq.x()
    }
    pub fn y(&self) -> f64 {
        self.sq.y()
    }
    pub fn size(&self) -> f64 {
        self.sq.size()
    }

    pub fn select(&self) -> Selected {
        self.select
    }
}
