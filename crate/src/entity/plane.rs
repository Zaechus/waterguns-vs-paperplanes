use std::{collections::HashMap, f64::consts::PI};

use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::types::{Direction, PlanePath, Rect};

/// An entity spawned by the game to get to the end a map and reduce the player's HP
#[wasm_bindgen]
pub struct PaperPlane {
    rect: Rect,
    rotation: f64,
    speed: f64,
    dx: f64,
    dy: f64,
    img: String,
    hp: i32,
    max_hp: u32,
    damage: u32,
    bounty: u32,
}

impl PaperPlane {
    /// Constructs a new basic Plane
    pub fn new_basic(rect: Rect) -> Self {
        Self {
            rect,
            rotation: 0.0,
            speed: 1.7,
            dx: 1.7,
            dy: 0.0,
            img: String::from("Plane"),
            hp: 40,
            max_hp: 40,
            damage: 1,
            bounty: 5,
        }
    }

    /// Constructs a new Bullet
    pub fn new_bullet(rect: Rect) -> Self {
        Self {
            rect,
            rotation: 0.0,
            speed: 3.0,
            dx: 3.0,
            dy: 0.0,
            img: String::from("Bullet"),
            hp: 25,
            max_hp: 25,
            damage: 2,
            bounty: 5,
        }
    }

    /// Constructs a new Glider
    pub fn new_glider(rect: Rect) -> Self {
        Self {
            rect,
            rotation: 0.0,
            speed: 1.5,
            dx: 1.5,
            dy: 0.0,
            img: String::from("Glider"),
            hp: 50,
            max_hp: 50,
            damage: 2,
            bounty: 10,
        }
    }

    /// Constructs a new Blimp
    pub fn new_blimp(rect: Rect) -> Self {
        Self {
            rect,
            rotation: 0.0,
            speed: 1.0,
            dx: 1.0,
            dy: 0.0,
            img: String::from("Blimp"),
            hp: 100,
            max_hp: 100,
            damage: 3,
            bounty: 10,
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

    /// Returns the amount of damage the Plane does upon course completion
    pub fn damage(&self) -> u32 {
        self.damage
    }
    /// Returns the amount of cash the Plane yields when destroyed
    pub fn bounty(&self) -> u32 {
        self.bounty
    }

    /// Reduce the HP of the Plane by a damage value
    pub fn take_damage(&mut self, dmg: i32) {
        self.hp -= dmg;
    }

    /// Advance the location of the Plane by one increment
    pub fn fly(&mut self, path: &PlanePath) {
        for turn in path.turns().iter() {
            if turn.touching(&self.rect) {
                match turn.direction() {
                    Direction::Up => {
                        self.dx = 0.0;
                        self.dy = -self.speed;
                        self.rotation = PI * 1.5;
                    }
                    Direction::Down => {
                        self.dx = 0.0;
                        self.dy = self.speed;
                        self.rotation = PI * 0.5;
                    }
                    Direction::Left => {
                        self.dx = -self.speed;
                        self.rotation = PI;
                        self.dy = 0.0;
                    }
                    Direction::Right => {
                        self.dx = self.speed;
                        self.rotation = 0.0;
                        self.dy = 0.0;
                    }
                }
            }
        }
        self.rect
            .set_pos(self.rect.x() + self.dx, self.rect.y() + self.dy);
    }

    /// Draw the HP indicator of the Plane
    fn draw_hp_bar(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str("#00ff00"));
        ctx.fill_rect(
            -self.rect.w() * 0.5,
            -self.rect.h() * 0.5,
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
        ctx.translate(self.rect.center_x(), self.rect.center_y())?;
        ctx.rotate(self.rotation)?;
        ctx.draw_image_with_html_image_element_and_dw_and_dh(
            sprites.get(&self.img).unwrap(),
            -self.rect.w() * 0.5,
            -self.rect.h() * 0.5,
            self.rect.w(),
            self.rect.h(),
        )?;
        self.draw_hp_bar(ctx)?;
        ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)?;

        Ok(())
    }
}
