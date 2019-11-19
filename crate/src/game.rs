use std::collections::HashMap;

use wasm_bindgen::{prelude::*, JsCast};

use js_sys::Date;

use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

use crate::{
    entity::{Button, PaperPlane, Tower},
    types::{ButtonType, Direction, Mouse, PlanePath, Rect, TowerStatus, Turn},
    utils::set_panic_hook,
};

const WATERGUN_COST: i32 = 20;
const ACID_COST: i32 = 30;
const SODA_COST: i32 = 50;

/// A struct that handles the workings of the game
#[wasm_bindgen]
pub struct Game {
    plane_size: f64,
    tower_size: f64,
    ui_text_size: f64,

    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    mouse: Mouse,

    sprites: HashMap<String, HtmlImageElement>,
    planes: Vec<PaperPlane>,
    towers: Vec<Tower>,
    buttons: Vec<Button>,
    path: PlanePath,

    round: u32,
    start_time: f64,
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

        let sprite_names: [&str; 22] = [
            "Map",
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

        let tower_size = canvas.height() as f64 * 0.08;
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
                    canvas.width() as f64 - 10.0 - tower_size * 2.0,
                    tower_size * 0.05,
                    tower_size,
                    tower_size,
                ),
                ButtonType::AcidTower,
                "AcidTowerTop",
            ),
            Button::new(
                Rect::new(
                    canvas.width() as f64 - 15.0 - tower_size * 3.0,
                    tower_size * 0.05,
                    tower_size,
                    tower_size,
                ),
                ButtonType::SodaMaker,
                "SodaMakerTop",
            ),
        ];

        Self {
            path: PlanePath::new(vec![
                Turn::new(
                    (canvas.width() as f64 * 0.22, canvas.height() as f64 * 0.26),
                    Direction::Down,
                ),
                Turn::new(
                    (canvas.width() as f64 * 0.22, canvas.height() as f64 * 0.72),
                    Direction::Right,
                ),
                Turn::new(
                    (canvas.width() as f64 * 0.43, canvas.height() as f64 * 0.72),
                    Direction::Up,
                ),
                Turn::new(
                    (canvas.width() as f64 * 0.43, canvas.height() as f64 * 0.25),
                    Direction::Right,
                ),
                Turn::new(
                    (canvas.width() as f64 * 0.62, canvas.height() as f64 * 0.25),
                    Direction::Down,
                ),
                Turn::new(
                    (canvas.width() as f64 * 0.62, canvas.height() as f64 * 0.72),
                    Direction::Right,
                ),
                Turn::new(
                    (canvas.width() as f64 * 0.85, canvas.height() as f64 * 0.72),
                    Direction::Up,
                ),
                Turn::new(
                    (canvas.width() as f64 * 0.85, canvas.height() as f64 * 0.26),
                    Direction::Right,
                ),
            ]),
            plane_size: canvas.height() as f64 * 0.05,
            tower_size,
            ui_text_size: canvas.height() as f64 * 0.03,
            canvas,
            ctx,
            mouse: Mouse::new(),
            sprites,
            planes: Vec::with_capacity(50),
            towers: Vec::with_capacity(10),
            buttons,
            round: 0,
            start_time: Date::now(),
            hp: 100,
            cash: 100,
        }
    }

    fn update_round(&mut self) {
        let elapsed = Date::now() - self.start_time;
        self.round = if elapsed >= 10000.0 && self.round == 1 {
            2
        } else if elapsed >= 1000.0 && self.round == 0 {
            1
        } else {
            self.round
        };
    }

    fn make_planes(&mut self) {
        match self.round {
            1 => {
                for x in 0..25 {
                    self.planes.push(PaperPlane::new_bullet(Rect::new(
                        self.plane_size * -x as f64,
                        self.canvas.height() as f64 * 0.26,
                        self.plane_size,
                        self.plane_size,
                    )));
                }
            }
            _ => (),
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
                if self.mouse.y() < self.tower_size && self.mouse.inside(button.rect()) {
                    button.select();
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
            plane.fly(&self.path);
        }
    }

    /// Remove planes if they complete the track or get destroyed
    fn remove_towers(&mut self) {
        let mut i = 0;
        while i != self.towers.len() {
            if let TowerStatus::Deleted = self.towers[i].status() {
                self.towers.remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// Remove planes if they complete the track or get destroyed
    fn remove_planes(&mut self) {
        let mut i = 0;
        while i != self.planes.len() {
            if self.planes[i].hp() <= 0 {
                self.cash += self.planes[i].bounty() as i32;
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
            .fill_text(&format!("❤️ : {}", self.hp), 10.0, self.ui_text_size + 5.0)
            .expect("display hp");
        self.ctx
            .fill_text(&format!("$  : {}", self.cash), 10.0, self.tower_size - 5.0)
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

        self.ctx.draw_image_with_html_image_element_and_dw_and_dh(
            self.sprites.get("Map").unwrap(),
            0.0,
            0.0,
            self.canvas.width().into(),
            self.canvas.height().into(),
        )?;

        self.update_round();
        self.make_planes();

        self.events();

        self.render_towers();
        self.render_planes();

        self.remove_towers();
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
