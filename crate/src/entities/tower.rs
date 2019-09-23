use wasm_bindgen::prelude::*;

use js_sys::Date;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::entities::PaperPlane;
use crate::types::Rect;

#[wasm_bindgen]
pub struct Tower {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    dmg: i32,
    range: f64,
    last_dmg_time: f64,
}

#[wasm_bindgen]
impl Tower {
    pub fn new(rect: Rect, dmg: i32, range: f64) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            w: rect.w,
            h: rect.h,
            dmg,
            range,
            last_dmg_time: 0.0,
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

    pub fn left_range(&self) -> f64 {
        self.x - self.range
    }
    pub fn right_range(&self) -> f64 {
        self.x + self.w + self.range
    }
    pub fn top_range(&self) -> f64 {
        self.y - self.range
    }
    pub fn bottom_range(&self) -> f64 {
        self.y + self.h + self.range
    }

    pub fn damage(&mut self, plane: &mut PaperPlane) {
        if plane.right_side() > self.left_range()
            && plane.x() < self.right_range()
            && plane.bottom_side() > self.top_range()
            && plane.y() < self.bottom_range()
        {
            if (Date::now() - self.last_dmg_time).abs() > 750.0 {
                self.last_dmg_time = Date::now();

                plane.take_damage(self.dmg);
            }
        }
    }

    pub fn draw(
        &self,
        ctx: &CanvasRenderingContext2d,
        img: &HtmlImageElement,
    ) -> Result<(), JsValue> {
        ctx.draw_image_with_html_image_element_and_dw_and_dh(img, self.x, self.y, self.w, self.h)?;
        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str("#ff0000"));
        ctx.stroke_rect(
            self.x - self.range,
            self.y - self.range,
            self.w + self.range * 2.0,
            self.h + self.range * 2.0,
        );
        ctx.close_path();
        Ok(())
    }
}
