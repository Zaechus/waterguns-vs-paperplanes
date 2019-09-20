use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::entities::Tower;
use crate::types::Rect;

#[wasm_bindgen]
pub struct PaperPlane {
    img: HtmlCanvasElement,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    hp: i32,
    max_hp: u32,
}

#[wasm_bindgen]
impl PaperPlane {
    pub fn new(img: HtmlCanvasElement, rect: Rect, hp: i32) -> Self {
        Self {
            img,
            x: rect.x,
            y: rect.y,
            w: rect.w,
            h: rect.h,
            hp,
            max_hp: hp as u32,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn w(&self) -> f64 {
        self.w
    }
    pub fn h(&self) -> f64 {
        self.h
    }

    pub fn right_side(&self) -> f64 {
        self.x + self.w
    }
    pub fn bottom_side(&self) -> f64 {
        self.y + self.h
    }

    pub fn hp(&self) -> i32 {
        self.hp
    }
    pub fn hp_percent(&self) -> f64 {
        self.hp as f64 / self.max_hp as f64
    }

    pub fn take_damage(&mut self, tower: &Tower, dmg: i32) {
        if self.right_side() > tower.left_range()
            && self.x < tower.right_range()
            && self.bottom_side() > tower.top_range()
            && self.y < tower.bottom_range()
        {
            self.hp -= dmg;
        }
    }

    pub fn fly(&mut self) {
        self.x += 2.0;
    }

    pub fn draw(&self, ctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
        ctx.draw_image_with_html_canvas_element_and_dw_and_dh(
            &self.img, self.x, self.y, self.w, self.h,
        )?;
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str("#00ff00"));
        ctx.fill_rect(self.x, self.y, self.w * self.hp_percent(), 3.0);
        ctx.close_path();
        Ok(())
    }
}
