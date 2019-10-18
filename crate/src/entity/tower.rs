use std::f64::consts::PI;

use wasm_bindgen::prelude::*;

use js_sys::Date;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::entity::PaperPlane;
use crate::types::Rect;

#[wasm_bindgen]
pub struct Tower {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    center_x: f64,
    center_y: f64,
    dmg: i32,
    dmg_interval: f64,
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
            center_x: rect.x + rect.w / 2.0,
            center_y: rect.y + rect.h / 2.0,
            dmg,
            dmg_interval: 750.0,
            range,
            last_dmg_time: 0.0,
        }
    }

    pub fn damage(&mut self, plane: &mut PaperPlane) {
        let dx = self.center_x - (plane.x() + plane.w() / 2.0);
        let dy = self.center_y - (plane.y() + plane.h() / 2.0);
        let dist = (dx.powi(2) + dy.powi(2)).sqrt();

        if dist < self.range {
            if Date::now() - self.last_dmg_time > self.dmg_interval {
                self.last_dmg_time = Date::now();

                plane.take_damage(self.dmg);
            }
        }
    }

    pub fn draw(
        &self,
        ctx: &CanvasRenderingContext2d,
        base_img: &HtmlImageElement,
        top_img: &HtmlImageElement,
        firing_img: &HtmlImageElement,
    ) -> Result<(), JsValue> {
        ctx.draw_image_with_html_image_element_and_dw_and_dh(
            base_img, self.x, self.y, self.w, self.h,
        )?;

        let size = self.w * 1.75;
        let offset = size / 4.8;
        let x = self.x - offset;
        let y = self.y - offset;
        if Date::now() - self.last_dmg_time < 100.0 {
            ctx.draw_image_with_html_image_element_and_dw_and_dh(firing_img, x, y, size, size)?;
        } else {
            ctx.draw_image_with_html_image_element_and_dw_and_dh(top_img, x, y, size, size)?;
        }
        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str("#ff0000"));
        ctx.rect(self.x, self.y, self.w, self.h);
        ctx.stroke();
        ctx.close_path();
        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str("#ff0000"));
        ctx.ellipse(
            self.x + self.w / 2.0,
            self.y + self.h / 2.0,
            self.range,
            self.range,
            PI / 4.0,
            0.0,
            2.0 * PI,
        )?;
        ctx.stroke();
        ctx.close_path();
        Ok(())
    }
}
