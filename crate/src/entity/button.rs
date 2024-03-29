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
    selected: bool,
}

impl Button {
    /// Constructs a custom Button
    pub fn new(rect: Rect, variant: ButtonType, content: &str) -> Self {
        Self {
            rect,
            variant,
            content: String::from(content),
            selected: false,
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

    /// Draws the selection highlight for the Button
    fn draw_selection(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str("#00ff00"));
        ctx.rect(
            self.x(),
            self.y() - self.h() * 0.05,
            self.w(),
            self.h() + self.h() * 0.1,
        );
        ctx.stroke();
        ctx.close_path();
    }

    /// Draw a button with image content
    fn draw_image_button(
        &self,
        ctx: &CanvasRenderingContext2d,
        img: &HtmlImageElement,
    ) -> Result<(), JsValue> {
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str("#222222"));
        ctx.rect(
            self.rect.x(),
            self.rect.y() - self.h() * 0.05,
            self.rect.w(),
            self.rect.h() + self.h() * 0.1,
        );
        ctx.fill();
        ctx.draw_image_with_html_image_element_and_dw_and_dh(
            img,
            self.rect.x(),
            self.rect.y(),
            self.rect.w(),
            self.rect.h(),
        )?;
        ctx.close_path();
        Ok(())
    }

    /// Draw a button with text content
    fn draw_text_button(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str("#222222"));
        ctx.rect(self.rect.x(), self.rect.y(), self.rect.w(), self.rect.h());
        ctx.fill();
        ctx.set_fill_style(&JsValue::from_str("#00ff00"));
        ctx.set_font(&format!("{}px monospace", self.rect.w() * 0.2));
        ctx.fill_text(
            &self.content,
            self.rect.x() + self.rect.w() * 0.07,
            self.rect.y() + self.rect.h() * 0.6,
        )?;
        ctx.close_path();
        Ok(())
    }

    /// Draws the button
    pub fn draw(
        &self,
        ctx: &CanvasRenderingContext2d,
        sprites: &HashMap<String, HtmlImageElement>,
    ) -> Result<(), JsValue> {
        if let Some(img) = sprites.get(&self.content) {
            self.draw_image_button(ctx, &img)
        } else {
            self.draw_text_button(ctx)
        }?;

        if self.selected {
            self.draw_selection(ctx)
        }
        Ok(())
    }

    /// Returns the x position of the Button
    pub fn x(&self) -> f64 {
        self.rect.x()
    }
    /// Returns the y position of the Button
    pub fn y(&self) -> f64 {
        self.rect.y()
    }
    /// Returns the width of the Button
    pub fn w(&self) -> f64 {
        self.rect.w()
    }
    /// Returns the height of the Button
    pub fn h(&self) -> f64 {
        self.rect.h()
    }
    /// Returns a reference to the Rect field of the Button
    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    /// Returns the type of the Button
    pub fn button_type(&self) -> ButtonType {
        self.variant
    }
    /// Returns whether or not the Button is selected
    pub fn selected(&self) -> bool {
        self.selected
    }
    /// Set the selected property of the Button to true
    pub fn select(&mut self) {
        self.selected = true;
    }
    /// Set the selected property of the Button to false
    pub fn deselect(&mut self) {
        self.selected = false;
    }
}
