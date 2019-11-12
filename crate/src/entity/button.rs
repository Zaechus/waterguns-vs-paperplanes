use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::types::{ButtonType, Rect};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Button {
    rect: Rect,
    variant: ButtonType,
    content: String,
}

impl Button {
    /// Constructs a custom Button
    pub fn new(rect: Rect, variant: ButtonType, content: &str) -> Self {
        Self {
            rect,
            variant,
            content: String::from(content),
        }
    }

    /// Constructs a new Upgrade Button
    pub fn new_upgrade(rect: Rect) -> Self {
        Button::new(
            Rect::new(rect.x(), rect.y(), rect.w(), rect.h()),
            ButtonType::Upgrade,
            "Upgrade",
        )
    }

    /// Constructs a new Delete Button
    pub fn new_delete(rect: Rect) -> Self {
        Button::new(
            Rect::new(rect.x(), rect.y(), rect.w(), rect.h()),
            ButtonType::Delete,
            "Delete",
        )
    }

    /// Draws the button
    pub fn draw(
        &self,
        ctx: &CanvasRenderingContext2d,
        sprites: &HashMap<String, HtmlImageElement>,
    ) -> Result<(), JsValue> {
        if let Some(img) = sprites.get(&self.content) {
            ctx.draw_image_with_html_image_element_and_dw_and_dh(
                img,
                self.rect.x(),
                self.rect.y(),
                self.rect.w(),
                self.rect.h(),
            )?;
            Ok(())
        } else {
            ctx.begin_path();
            ctx.set_fill_style(&JsValue::from_str("#222222"));
            ctx.rect(
                self.rect.x(),
                self.rect.y(),
                self.rect.w(),
                self.rect.h() * 0.5,
            );
            ctx.fill();
            ctx.set_fill_style(&JsValue::from_str("#00ff00"));
            ctx.set_font(&format!("{}px monospace", self.rect.w() * 0.2));
            ctx.fill_text(
                &self.content,
                self.rect.x() + self.rect.w() * 0.07,
                self.rect.y() + self.rect.h() * 0.3,
            )?;
            ctx.close_path();
            Ok(())
        }
    }

    pub fn x(&self) -> f64 {
        self.rect.x()
    }
    pub fn y(&self) -> f64 {
        self.rect.y()
    }
    pub fn w(&self) -> f64 {
        self.rect.w()
    }
    pub fn h(&self) -> f64 {
        self.rect.h()
    }
    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn button_type(&self) -> ButtonType {
        self.variant
    }
}
