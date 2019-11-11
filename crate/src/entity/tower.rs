use std::{collections::HashMap, f64::consts::PI};

use wasm_bindgen::prelude::*;

use js_sys::Date;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::{
    entity::{Button, PaperPlane},
    types::{Mouse, Selected, Square},
};

/// Different upgrade variants of a Water Gun
enum WaterGun {
    Basic,
    SuperSoaker,
    ExtremeSoaker,
}
/// Different upgrade variants of an Acid Tower
enum AcidTower {
    Basic,
    Radioactive,
}
/// Different upgrade variants of a Soda Maker
enum SodaMaker {
    Basic,
    SparklingWater,
    RootBeer,
}

/// Represents the current type of Tower
enum TowerType {
    WaterGun(WaterGun),
    AcidTower(AcidTower),
    SodaMaker(SodaMaker),
}

/// An entity the user spends cash to create in order to destroy Planes
#[wasm_bindgen]
pub struct Tower {
    variant: TowerType,

    x: f64,
    y: f64,
    size: f64,
    center_x: f64,
    center_y: f64,
    rotation: f64,

    base_img: String,
    blast_img: String,
    top_img: String,

    upgrade_button: Button,
    upgrade_cost: i32,

    dmg: i32,
    dmg_interval: f64,
    range: f64,
    last_dmg_time: f64,

    mouse_over: bool,
    context_open: bool,
}

impl Tower {
    /// Construct a new Water Gun
    pub fn new_water_gun(square: Square) -> Self {
        Self {
            variant: TowerType::WaterGun(WaterGun::Basic),
            x: square.x,
            y: square.y,
            size: square.size,
            center_x: square.center_x(),
            center_y: square.center_y(),
            rotation: 0.0,
            base_img: String::from("WaterGunBase"),
            blast_img: String::from("WaterGunBlast"),
            top_img: String::from("WaterGunTop"),
            upgrade_button: Button::new(
                Square::new(square.x, square.y - square.size * 0.6, square.size),
                Selected::None,
                "Upgrade",
            ),
            upgrade_cost: 10,
            dmg: 10,
            dmg_interval: 700.0,
            range: 200.0,
            last_dmg_time: 0.0,
            mouse_over: false,
            context_open: false,
        }
    }

    /// Construct a new Acid Tower
    pub fn new_acid_tower(square: Square) -> Self {
        Self {
            variant: TowerType::AcidTower(AcidTower::Basic),
            x: square.x,
            y: square.y,
            size: square.size,
            center_x: square.x + square.size * 0.5,
            center_y: square.y + square.size * 0.5,
            rotation: 0.0,
            base_img: String::from("WaterGunBase"),
            blast_img: String::from("AcidTowerBlast"),
            top_img: String::from("AcidTowerTop"),
            upgrade_button: Button::new(
                Square::new(square.x, square.y - square.size * 0.6, square.size),
                Selected::None,
                "Upgrade",
            ),
            upgrade_cost: 10,
            dmg: 1,
            dmg_interval: 100.0,
            range: 150.0,
            last_dmg_time: 0.0,
            mouse_over: false,
            context_open: false,
        }
    }

    /// Construct a new Soda Maker
    pub fn new_soda_maker(square: Square) -> Self {
        Self {
            variant: TowerType::SodaMaker(SodaMaker::Basic),
            x: square.x,
            y: square.y,
            size: square.size,
            center_x: square.x + square.size * 0.5,
            center_y: square.y + square.size * 0.5,
            rotation: 0.0,
            base_img: String::from("WaterGunBase"),
            blast_img: String::from("SodaMakerBlast"),
            top_img: String::from("SodaMakerTop"),
            upgrade_button: Button::new(
                Square::new(square.x, square.y - square.size * 0.6, square.size),
                Selected::None,
                "Upgrade",
            ),
            upgrade_cost: 10,
            dmg: 20,
            dmg_interval: 1000.0,
            range: 250.0,
            last_dmg_time: 0.0,
            mouse_over: false,
            context_open: false,
        }
    }

    /// Return the x-coordinate of the center of the tower
    pub fn center_x(&self) -> f64 {
        self.center_x
    }
    /// Return the y-coordinate of the center of the tower
    pub fn center_y(&self) -> f64 {
        self.center_y
    }

    /// Takes a reference to a Plane and applies damage if conditions are met
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

    /// Draws the circular range of the Tower
    fn draw_range(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
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
        Ok(())
    }

    /// Indicates selection of the tower when drawn
    fn draw_selection(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str("#00ff00"));
        ctx.ellipse(
            self.center_x,
            self.center_y,
            self.size * 0.8,
            self.size * 0.8,
            PI * 0.25,
            0.0,
            PI * 2.0,
        )?;
        ctx.stroke();
        ctx.close_path();
        Ok(())
    }

    /// Draws the Tower on the referenced Context
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

        // draw top sprite with potential blast sprite
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

        if self.mouse_over {
            self.draw_range(ctx)?;

            self.draw_selection(ctx)?;
        }

        if self.context_open {
            self.draw_selection(ctx)?;

            self.upgrade_button.draw(ctx, sprites)?;
        }

        Ok(())
    }

    /// Handle mouse interaction with the Tower
    pub fn events(&mut self, mouse: &Mouse, cash: &mut i32) {
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
            } else if mouse.x > self.x
                && mouse.y > self.y - self.size * 0.6
                && mouse.x < self.x + self.size
                && mouse.y < self.y
            {
                match self.variant {
                    TowerType::WaterGun(WaterGun::Basic) => self.upgrade_water2(cash),
                    TowerType::WaterGun(WaterGun::SuperSoaker) => self.upgrade_water3(cash),
                    TowerType::AcidTower(AcidTower::Basic) => self.upgrade_acid2(cash),
                    TowerType::SodaMaker(SodaMaker::Basic) => self.upgrade_soda2(cash),
                    TowerType::SodaMaker(SodaMaker::SparklingWater) => self.upgrade_soda3(cash),
                    _ => (),
                }
            }
        }
    }

    /// Upgrade the tower to a Super Soaker
    fn upgrade_water2(&mut self, cash: &mut i32) {
        if *cash >= self.upgrade_cost {
            self.top_img = String::from("SuperSoakerTop");
            self.blast_img = String::from("SuperSoakerBlast");
            self.variant = TowerType::WaterGun(WaterGun::SuperSoaker);
            self.range *= 1.2;
            self.dmg += 5;
            self.dmg_interval *= 0.5;
            *cash -= self.upgrade_cost;
            self.upgrade_cost += 10;
        }
    }
    /// Upgrade the tower to an Extreme Soaker
    fn upgrade_water3(&mut self, cash: &mut i32) {
        if *cash >= self.upgrade_cost {
            self.top_img = String::from("ExtremeSoakerTop");
            self.blast_img = String::from("ExtremeSoakerBlast");
            self.variant = TowerType::WaterGun(WaterGun::ExtremeSoaker);
            self.range *= 1.2;
            self.dmg += 5;
            self.dmg_interval *= 0.66;
            *cash -= self.upgrade_cost;
            self.upgrade_cost += 10;
        }
    }
    /// Upgrade the tower to a Radioactive Tower
    fn upgrade_acid2(&mut self, cash: &mut i32) {
        if *cash >= self.upgrade_cost {
            self.top_img = String::from("RadioactiveTowerTop");
            self.blast_img = String::from("RadioactiveTowerBlast");
            self.variant = TowerType::AcidTower(AcidTower::Radioactive);
            self.range *= 1.1;
            self.dmg *= 2;
            self.dmg_interval *= 0.5;
            *cash -= self.upgrade_cost;
            self.upgrade_cost += 10;
        }
    }
    /// Upgrade the tower to a Sparkling Water Tower
    fn upgrade_soda2(&mut self, cash: &mut i32) {
        if *cash >= self.upgrade_cost {
            self.top_img = String::from("SparklingWaterTop");
            self.blast_img = String::from("SparklingWaterBlast");
            self.variant = TowerType::SodaMaker(SodaMaker::SparklingWater);
            self.range *= 1.2;
            self.dmg += 10;
            *cash -= self.upgrade_cost;
            self.upgrade_cost += 10;
        }
    }
    /// Upgrade the tower to a Root Beer Blaster
    fn upgrade_soda3(&mut self, cash: &mut i32) {
        if *cash >= self.upgrade_cost {
            self.top_img = String::from("RootBeerTop");
            self.blast_img = String::from("RootBeerBlast");
            self.variant = TowerType::SodaMaker(SodaMaker::RootBeer);
            self.range *= 1.2;
            self.dmg += 20;
            *cash -= self.upgrade_cost;
            self.upgrade_cost += 10;
        }
    }
}
