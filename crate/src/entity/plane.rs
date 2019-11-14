use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::types::Rect;

/// An entity spawned by the game to get to the end a map and reduce the player's HP
#[wasm_bindgen]
pub struct PaperPlane {
    rect: Rect,
    speed: f64,
    img: String,
    hp: i32,
    max_hp: u32,
    damage: u32,
}

impl PaperPlane {
    /// Constructs a new basic Plane
    pub fn new_basic(rect: Rect) -> Self {
        Self {
            rect,
            speed: 1.7,
            img: String::from("Plane"),
            hp: 40,
            max_hp: 40,
            damage: 1,
        }
    }

    /// Constructs a new Bullet
    pub fn new_bullet(rect: Rect) -> Self {
        Self {
            rect,
            speed: 2.0,
            img: String::from("Bullet"),
            hp: 25,
            max_hp: 25,
            damage: 2,
        }
    }

    /// Constructs a new Glider
    pub fn new_glider(rect: Rect) -> Self {
        Self {
            rect,
            speed: 1.5,
            img: String::from("Glider"),
            hp: 50,
            max_hp: 50,
            damage: 2,
        }
    }

    /// Constructs a new Blimp
    pub fn new_blimp(rect: Rect) -> Self {
        Self {
            rect,
            speed: 1.0,
            img: String::from("Blimp"),
            hp: 100,
            max_hp: 100,
            damage: 3,
        }
    }

    /// Return the x-coordinate of the Plane
    pub fn x(&self) -> f64 {
        self.rect.x()
    }
    /// Return the y-coordinate of the Plane
    pub fn y(&self) -> f64 {
        self.rect.y()
    }
    /// Return the width of the Plane
    pub fn w(&self) -> f64 {
        self.rect.w()
    }
    /// Return the height of the Plane
    pub fn h(&self) -> f64 {
        self.rect.h()
    }
    /// Return the x-coordinate of the center of the Plane
    pub fn center_x(&self) -> f64 {
        self.rect.center_x()
    }
    /// Return the y-coordinate of the center of the Plane
    pub fn center_y(&self) -> f64 {
        self.rect.center_y()
    }

    /// Return current HP of the Plane
    pub fn hp(&self) -> i32 {
        self.hp
    }
    /// Return the current HP of the Plane as a percentage
    pub fn hp_percent(&self) -> f64 {
        self.hp as f64 / self.max_hp as f64
    }

    /// Returns the amount of damage the plane does upon course completion
    pub fn damage(&self) -> u32 {
        self.damage
    }

    /// Reduce the HP of the Plane by a damage value
    pub fn take_damage(&mut self, dmg: i32) {
        self.hp -= dmg;
    }

    /// Advance the location of the Plane by one increment
    pub fn fly(&mut self) {
        self.rect.set_pos(self.rect.x() + self.speed, self.rect.y());
    }

    /// Draw the HP indicator of the Plane
    fn draw_hp_bar(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str("#00ff00"));
        ctx.fill_rect(
            self.rect.x(),
            self.rect.y(),
            self.rect.w() * self.hp_percent(),
            3.0,
        );
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
            self.rect.x(),
            self.rect.y(),
            self.rect.w(),
            self.rect.h(),
        )?;

        self.draw_hp_bar(ctx)?;

        Ok(())
    }
}
