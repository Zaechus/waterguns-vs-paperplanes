use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::types::Square;

/// An entity spawned by the game to get to the end a map and reduce the player's HP
#[wasm_bindgen]
pub struct PaperPlane {
    x: f64,
    y: f64,
    size: f64,
    center_x: f64,
    center_y: f64,
    speed: f64,
    img: String,
    hp: i32,
    max_hp: u32,
}

impl PaperPlane {
    /// Constructs a new basic Plane
    pub fn new_basic(square: Square) -> Self {
        Self {
            x: square.x,
            y: square.y,
            size: square.size,
            center_x: square.center_x(),
            center_y: square.center_y(),
            speed: 1.7,
            img: String::from("Plane"),
            hp: 40,
            max_hp: 40,
        }
    }

    /// Constructs a new Bullet
    pub fn new_bullet(square: Square) -> Self {
        Self {
            x: square.x,
            y: square.y,
            size: square.size,
            center_x: square.center_x(),
            center_y: square.center_y(),
            speed: 2.0,
            img: String::from("Bullet"),
            hp: 25,
            max_hp: 25,
        }
    }

    /// Constructs a new Glider
    pub fn new_glider(square: Square) -> Self {
        Self {
            x: square.x,
            y: square.y,
            size: square.size,
            center_x: square.center_x(),
            center_y: square.center_y(),
            speed: 1.5,
            img: String::from("Glider"),
            hp: 50,
            max_hp: 50,
        }
    }

    /// Constructs a new Blimp
    pub fn new_blimp(square: Square) -> Self {
        Self {
            x: square.x,
            y: square.y,
            size: square.size,
            center_x: square.center_x(),
            center_y: square.center_y(),
            speed: 1.0,
            img: String::from("Blimp"),
            hp: 100,
            max_hp: 100,
        }
    }

    /// Return the x-coordinate of the Plane
    pub fn x(&self) -> f64 {
        self.x
    }
    /// Return the y-coordinate of the Plane
    pub fn y(&self) -> f64 {
        self.y
    }
    /// Return the size of the Plane
    pub fn size(&self) -> f64 {
        self.size
    }
    /// Return the x-coordinate of the center of the Plane
    pub fn center_x(&self) -> f64 {
        self.center_x
    }
    /// Return the y-coordinate of the center of the Plane
    pub fn center_y(&self) -> f64 {
        self.center_y
    }

    /// Return current HP of the Plane
    pub fn hp(&self) -> i32 {
        self.hp
    }
    /// Return the current HP of the Plane as a percentage
    pub fn hp_percent(&self) -> f64 {
        self.hp as f64 / self.max_hp as f64
    }

    /// Reduce the HP of the Plane by a damage value
    pub fn take_damage(&mut self, dmg: i32) {
        self.hp -= dmg;
    }

    /// Advance the location of the Plane by one increment
    pub fn fly(&mut self) {
        self.x += self.speed;
        self.center_x += self.speed;
    }

    /// Draw the HP indicator of the Plane
    fn draw_hp_bar(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str("#00ff00"));
        ctx.fill_rect(self.x, self.y, self.size * self.hp_percent(), 3.0);
        ctx.close_path();

        Ok(())
    }

    /// Draw the Plane on the referenced Context
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
        )?;

        self.draw_hp_bar(ctx)?;

        Ok(())
    }
}
