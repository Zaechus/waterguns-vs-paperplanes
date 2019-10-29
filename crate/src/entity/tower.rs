use std::{collections::HashMap, f64::consts::PI};

use wasm_bindgen::prelude::*;

use js_sys::Date;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

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

    base_img: String,
    blast_img: String,
    top_img: String,

    dmg: i32,
    dmg_interval: f64,
    range: f64,
    last_dmg_time: f64,

    mouse_over: bool,
    context_open: bool,
}

impl Tower {
    pub fn new_water_gun(square: Square) -> Self {
        Self {
            x: square.x,
            y: square.y,
            size: square.size,
            center_x: square.x + square.size * 0.5,
            center_y: square.y + square.size * 0.5,
            rotation: 0.0,
            base_img: String::from("WaterGunBase"),
            blast_img: String::from("WaterGunBlast"),
            top_img: String::from("WaterGunTop"),
            dmg: 15,
            dmg_interval: 750.0,
            range: 200.0,
            last_dmg_time: 0.0,
            mouse_over: false,
            context_open: false,
        }
    }

    pub fn new_acid_tower(square: Square) -> Self {
        Self {
            x: square.x,
            y: square.y,
            size: square.size,
            center_x: square.x + square.size * 0.5,
            center_y: square.y + square.size * 0.5,
            rotation: 0.0,
            base_img: String::from("WaterGunBase"),
            blast_img: String::from("WaterGunBlast"),
            top_img: String::from("AcidTowerTop"),
            dmg: 15,
            dmg_interval: 750.0,
            range: 150.0,
            last_dmg_time: 0.0,
            mouse_over: false,
            context_open: false,
        }
    }

    pub fn new_soda_maker(square: Square) -> Self {
        Self {
            x: square.x,
            y: square.y,
            size: square.size,
            center_x: square.x + square.size * 0.5,
            center_y: square.y + square.size * 0.5,
            rotation: 0.0,
            base_img: String::from("WaterGunBase"),
            blast_img: String::from("WaterGunBlast"),
            top_img: String::from("SodaMakerTop"),
            dmg: 15,
            dmg_interval: 1000.0,
            range: 250.0,
            last_dmg_time: 0.0,
            mouse_over: false,
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
        sprites: &HashMap<String, HtmlImageElement>,
    ) -> Result<(), JsValue> {
        // draw tower base
        let base_size = self.size * 1.25;
        ctx.draw_image_with_html_image_element_and_dw_and_dh(
            sprites.get(&self.base_img).unwrap(),
            self.center_x - base_size * 0.5,
            self.center_y - base_size / 2.5,
            base_size,
            base_size,
        )?;

        // draw top image
        ctx.translate(self.center_x, self.center_y)?;
        ctx.rotate(self.rotation)?;
        if Date::now() - self.last_dmg_time < 100.0 {
            ctx.draw_image_with_html_image_element_and_dw_and_dh(
                sprites.get(&self.blast_img).unwrap(),
                -self.size * 0.5,
                -self.size * 1.4,
                self.size,
                self.size,
            )?;
        }
        ctx.draw_image_with_html_image_element_and_dw_and_dh(
            sprites.get(&self.top_img).unwrap(),
            -self.size * 0.5,
            -self.size * 0.5,
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

        // temp tower range
        if self.mouse_over {
            ctx.begin_path();
            ctx.set_stroke_style(&JsValue::from_str("#00ff00"));
            ctx.ellipse(
                self.center_x,
                self.center_y,
                self.size,
                self.size,
                PI * 0.25,
                0.0,
                PI * 2.0,
            )?;
            ctx.stroke();
            ctx.close_path();
        }

        // TODO menu
        if self.context_open {
            ctx.begin_path();
            ctx.set_fill_style(&JsValue::from_str("#222222"));
            ctx.rect(self.x, self.y - self.size * 0.6, self.size, self.size * 0.5);
            ctx.fill();
            ctx.set_fill_style(&JsValue::from_str("#00ff00"));
            ctx.set_font(&format!("{}px sans-serif", self.size * 0.2));
            ctx.fill_text(
                "Upgrade",
                self.x + self.size * 0.07,
                self.y - self.size * 0.3,
            )
            .expect("display upgrade");
            ctx.close_path();
        }

        Ok(())
    }

    pub fn events(&mut self, mouse: &Mouse) {
        if mouse.x > self.x
            && mouse.y > self.y
            && mouse.x < self.x + self.size
            && mouse.y < self.y + self.size
        {
            self.mouse_over = true;
        } else {
            self.mouse_over = false;
        }
        if mouse.up {
            if self.mouse_over {
                self.context_open = !self.context_open;
            } else if mouse.x < self.x - self.size
                || mouse.y < self.y - self.size
                || mouse.x > self.x + self.size * 2.0
                || mouse.y > self.y + self.size * 2.0
            {
                self.context_open = false;
            }
        }
    }
}
