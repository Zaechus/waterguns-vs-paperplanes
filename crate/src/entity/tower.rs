use std::f64::consts::PI;

use wasm_bindgen::prelude::*;

use js_sys::Date;

use web_sys::{console, CanvasRenderingContext2d, HtmlImageElement};

use crate::{
    entity::PaperPlane,
    types::{Mouse, Square},
};

#[wasm_bindgen]
pub struct Tower {
    x: f64,
    y: f64,
    size: f64,
    center_x: f64,
    center_y: f64,
    rotation: f64,
    dmg: i32,
    dmg_interval: f64,
    range: f64,
    last_dmg_time: f64,
    context_open: bool,
}

impl Tower {
    pub fn new(square: Square, dmg: i32, range: f64) -> Self {
        Self {
            x: square.x,
            y: square.y,
            size: square.size,
            center_x: square.x + square.size / 2.0,
            center_y: square.y + square.size / 2.0,
            rotation: 0.0,
            dmg,
            dmg_interval: 750.0,
            range,
            last_dmg_time: 0.0,
            context_open: false,
        }
    }

    pub fn center_x(&self) -> f64 {
        self.center_x
    }
    pub fn center_y(&self) -> f64 {
        self.center_y
    }

    pub fn damage(&mut self, plane: &mut PaperPlane) {
        let dx = self.center_x - plane.center_x();
        let dy = self.center_y - plane.center_y();
        let dist = (dx.powi(2) + dy.powi(2)).sqrt();

        if dist < self.range {
            if Date::now() - self.last_dmg_time > self.dmg_interval {
                self.last_dmg_time = Date::now();

                if plane.center_y() > self.center_y {
                    self.rotation = PI - ((dx / dist).acos() + PI * 1.5);
                } else {
                    self.rotation = (dx / dist).acos() + PI * 1.5;
                }

                plane.take_damage(self.dmg);
            }
        }
    }

    pub fn draw(
        &self,
        ctx: &CanvasRenderingContext2d,
        base_img: &HtmlImageElement,
        top_img: &HtmlImageElement,
        blast_img: &HtmlImageElement,
    ) -> Result<(), JsValue> {
        // draw tower base
        let base_size = self.size * 1.25;
        ctx.draw_image_with_html_image_element_and_dw_and_dh(
            base_img,
            self.center_x - base_size / 2.0,
            self.center_y - base_size / 2.5,
            base_size,
            base_size,
        )?;

        // draw top image
        ctx.translate(self.center_x, self.center_y)?;
        ctx.rotate(self.rotation)?;
        if Date::now() - self.last_dmg_time < 100.0 {
            ctx.draw_image_with_html_image_element_and_dw_and_dh(
                blast_img,
                -self.size / 2.0,
                -self.size * 1.4,
                self.size,
                self.size,
            )?;
        }
        ctx.draw_image_with_html_image_element_and_dw_and_dh(
            top_img,
            -self.size / 2.0,
            -self.size / 2.0,
            self.size,
            self.size,
        )?;
        ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)?;

        // temp tower range
        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str("#ff0000"));
        ctx.ellipse(
            self.center_x,
            self.center_y,
            self.range,
            self.range,
            PI / 4.0,
            0.0,
            2.0 * PI,
        )?;
        ctx.stroke();
        ctx.close_path();

        if self.context_open {
            ctx.begin_path();
            ctx.set_stroke_style(&JsValue::from_str("#ff00ff"));
            ctx.rect(self.x, self.y, self.size, self.size);
            ctx.stroke();
            ctx.close_path();
        }

        Ok(())
    }

    pub fn events(&mut self, mouse: &Mouse) {
        if mouse.up {
            if mouse.x > self.x
                && mouse.y > self.y
                && mouse.x < self.x + self.size
                && mouse.y < self.y + self.size
            {
                console::log_1(&JsValue::from("CLICKED IN"));
                self.context_open = !self.context_open;
            }
        }
    }
}
