use std::collections::HashMap;

use wasm_bindgen::{prelude::*, JsCast};

use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

use crate::{
    entity::{Button, PaperPlane, Tower},
    types::{ButtonType, Mouse, Rect},
    utils::set_panic_hook,
};

const WATERGUN_COST: i32 = 20;
const ACID_COST: i32 = 30;
const SODA_COST: i32 = 50;

/// A struct that handles the workings of the game
#[wasm_bindgen]
pub struct Game {
    tower_size: f64,
    ui_text_size: f64,

    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    mouse: Mouse,

    sprites: HashMap<String, HtmlImageElement>,
    planes: Vec<PaperPlane>,
    towers: Vec<Tower>,
    buttons: Vec<Button>,

    hp: i32,
    cash: i32,
}

#[wasm_bindgen]
impl Game {
    /// Setup a new game
    pub fn new() -> Self {
        set_panic_hook();
        let document = window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")
            .expect("create canvas")
            .dyn_into::<HtmlCanvasElement>()
            .expect("dyn_into canvas element");
        document
            .body()
            .unwrap()
            .append_child(&canvas)
            .expect("doc append canvas");
        canvas.set_width(
            window()
                .unwrap()
                .inner_width()
                .expect("inner width")
                .as_f64()
                .unwrap() as u32,
        );
        canvas.set_height(
            window()
                .unwrap()
                .inner_height()
                .expect("inner height")
                .as_f64()
                .unwrap() as u32,
        );

        let ctx = canvas
            .get_context("2d")
            .expect("get 2d ctx")
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("dyn_into 2d ctx");

        ctx.set_font("36px monospace");

        let mut sprites = HashMap::new();

        let sprite_names: [&str; 21] = [
            "Plane",
            "Bullet",
            "Glider",
            "Blimp",
            "WaterGunBase",
            "WaterGunTop",
            "WaterGunBlast",
            "SuperSoakerTop",
            "SuperSoakerBlast",
            "ExtremeSoakerTop",
            "ExtremeSoakerBlast",
            "AcidTowerTop",
            "AcidTowerBlast",
            "RadioactiveTowerTop",
            "RadioactiveTowerBlast",
            "SodaMakerTop",
            "SodaMakerBlast",
            "SparklingWaterTop",
            "SparklingWaterBlast",
            "RootBeerTop",
            "RootBeerBlast",
        ];
        for sprite in sprite_names.iter() {
            let img = HtmlImageElement::new().unwrap();
            img.set_src(&format!("static/{}.png", sprite));
            sprites.insert(String::from(*sprite), img);
        }

        let plane_size = canvas.height() as f64 * 0.05;
        let mut planes = Vec::with_capacity(100);
        for i in 0..25 {
            planes.push(PaperPlane::new_basic(Rect::new(
                -i as f64 * 75.0 + 50.0,
                canvas.height() as f64 / ((i % 2 * 2) as f64 + 1.5) + i as f64,
                plane_size,
                plane_size,
            )));
        }
        for i in 25..50 {
            planes.push(PaperPlane::new_bullet(Rect::new(
                -i as f64 * 75.0 + 50.0,
                canvas.height() as f64 / ((i % 2 * 2) as f64 + 1.5) + i as f64,
                plane_size,
                plane_size,
            )));
        }
        for i in 50..75 {
            planes.push(PaperPlane::new_glider(Rect::new(
                -i as f64 * 75.0 + 50.0,
                canvas.height() as f64 / ((i % 2 * 2) as f64 + 1.5) + i as f64,
                plane_size,
                plane_size,
            )));
        }
        for i in 75..100 {
            planes.push(PaperPlane::new_blimp(Rect::new(
                -i as f64 * 75.0 + 50.0,
                canvas.height() as f64 / ((i % 2 * 2) as f64 + 1.5) + i as f64,
                plane_size,
                plane_size,
            )));
        }

        let tower_size = canvas.height() as f64 * 0.08;
        let towers = vec![Tower::new_water_gun(Rect::new(
            canvas.width() as f64 / 2.0,
            canvas.height() as f64 / 2.0,
            tower_size,
            tower_size,
        ))];

        let buttons = vec![
            Button::new(
                Rect::new(
                    canvas.width() as f64 - 5.0 - tower_size,
                    tower_size * 0.05,
                    tower_size,
                    tower_size,
                ),
                ButtonType::WaterGun,
                "WaterGunTop",
            ),
            Button::new(
                Rect::new(
                    canvas.width() as f64 - 5.0 - tower_size * 2.0,
                    tower_size * 0.05,
                    tower_size,
                    tower_size,
                ),
                ButtonType::AcidTower,
                "AcidTowerTop",
            ),
            Button::new(
                Rect::new(
                    canvas.width() as f64 - 5.0 - tower_size * 3.0,
                    tower_size * 0.05,
                    tower_size,
                    tower_size,
                ),
                ButtonType::SodaMaker,
                "SodaMakerTop",
            ),
        ];

        Self {
            tower_size,
            ui_text_size: canvas.width() as f64 * 0.015,
            canvas,
            ctx,
            mouse: Mouse::new(),
            sprites,
            planes,
            towers,
            buttons,
            hp: 100,
            cash: 100,
        }
    }

    /// Handle mouse events
    fn events(&mut self) {
        if self.mouse.up() {
            for button in self.buttons.iter_mut() {
                if self.mouse.y() > self.tower_size * 1.5 && button.selected() {
                    match button.button_type() {
                        ButtonType::WaterGun => {
                            if self.cash >= WATERGUN_COST {
                                self.towers.push(Tower::new_water_gun(Rect::new(
                                    self.mouse.x() - self.tower_size / 2.0,
                                    self.mouse.y() - self.tower_size / 2.0,
                                    self.tower_size,
                                    self.tower_size,
                                )));
                                self.cash -= WATERGUN_COST;
                            }
                        }
                        ButtonType::AcidTower => {
                            if self.cash >= ACID_COST {
                                self.towers.push(Tower::new_acid_tower(Rect::new(
                                    self.mouse.x() - self.tower_size / 2.0,
                                    self.mouse.y() - self.tower_size / 2.0,
                                    self.tower_size,
                                    self.tower_size,
                                )));
                                self.cash -= ACID_COST;
                            }
                        }
                        ButtonType::SodaMaker => {
                            if self.cash >= SODA_COST {
                                self.towers.push(Tower::new_soda_maker(Rect::new(
                                    self.mouse.x() - self.tower_size / 2.0,
                                    self.mouse.y() - self.tower_size / 2.0,
                                    self.tower_size,
                                    self.tower_size,
                                )));
                                self.cash -= SODA_COST;
                            }
                        }
                        _ => (),
                    }
                }
                button.deselect();
                if self.mouse.y() < self.tower_size {
                    if self.mouse.inside(button.rect()) {
                        button.select();
                    }
                }
            }
        }
    }

    /// Render towers and harm planes
    fn render_towers(&mut self) {
        for tower in self.towers.iter_mut() {
            tower.events(&self.mouse, &mut self.cash);
            tower.draw(&self.ctx, &self.sprites).expect("tower draw");

            for plane in self.planes.iter_mut() {
                tower.damage(plane);
            }
        }
    }

    /// Render all planes
    fn render_planes(&mut self) {
        for plane in self.planes.iter_mut() {
            plane.draw(&self.ctx, &self.sprites).expect("Plane draw");
            plane.fly();
        }
    }

    /// Remove planes if they complete the track or get destroyed
    fn remove_planes(&mut self) {
        let mut i = 0;
        while i != self.planes.len() {
            if self.planes[i].hp() <= 0 {
                self.cash += 10;
                self.planes.remove(i);
            } else if self.planes[i].x() >= self.canvas.width().into() {
                self.hp -= self.planes[i].damage() as i32;
                self.planes.remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// Render text found in the top bar
    fn render_text(&self) {
        self.ctx.begin_path();
        self.ctx.set_fill_style(&JsValue::from_str("#111111"));
        self.ctx
            .set_font(&format!("{}px sans-serif", self.ui_text_size));
        self.ctx
            .fill_text(&format!("HP: {}", self.hp), 10.0, 30.0)
            .expect("display hp");
        self.ctx
            .fill_text(&format!("Cash: {}", self.cash), 10.0, 70.0)
            .expect("display cash");
        self.ctx.close_path();
    }

    /// Render the top bar
    fn render_top_bar(&self) -> Result<(), JsValue> {
        self.ctx.begin_path();
        self.ctx.set_fill_style(&JsValue::from_str("#555555"));
        self.ctx
            .rect(0.0, 0.0, self.canvas.width() as f64, self.tower_size + 10.0);
        self.ctx.fill();
        self.ctx.close_path();

        for button in self.buttons.iter() {
            button.draw(&self.ctx, &self.sprites)?;
        }

        self.render_text();
        Ok(())
    }

    /// Render an increment of the Game
    pub fn draw(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        mouse_down: bool,
        mouse_up: bool,
    ) -> Result<(), JsValue> {
        self.mouse.update(mouse_x, mouse_y, mouse_down, mouse_up);

        self.ctx.clear_rect(
            0.0,
            0.0,
            self.canvas.width().into(),
            self.canvas.height().into(),
        );

        self.events();

        self.render_towers();
        self.render_planes();

        self.remove_planes();

        self.render_top_bar().expect("render top bar");
        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
