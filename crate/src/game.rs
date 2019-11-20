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

    bg_ctx: CanvasRenderingContext2d,
    fg_ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    mouse: Mouse,

    sprites: HashMap<String, HtmlImageElement>,
    planes: Vec<PaperPlane>,
    towers: Vec<Tower>,
    buttons: Vec<Button>,
    path: PlanePath,

    round: u32,
    round_start_time: f64,
    hp: i32,
    cash: i32,
}

#[wasm_bindgen]
impl Game {
    /// Setup a new game
    pub fn new() -> Self {
        set_panic_hook();
        let document = window().unwrap().document().unwrap();

        let window_width = window()
            .unwrap()
            .inner_width()
            .expect("inner width")
            .as_f64()
            .unwrap() as u32;
        let window_height = window()
            .unwrap()
            .inner_height()
            .expect("inner height")
            .as_f64()
            .unwrap() as u32;

        let bg_canvas = document
            .create_element("canvas")
            .expect("create canvas")
            .dyn_into::<HtmlCanvasElement>()
            .expect("dyn_into canvas element");
        bg_canvas.set_width(window_width);
        bg_canvas.set_height(window_height);
        bg_canvas
            .style()
            .set_property("position", "absolute")
            .unwrap();
        bg_canvas.style().set_property("z-index", "0").unwrap();

        let fg_canvas = document
            .create_element("canvas")
            .expect("create canvas")
            .dyn_into::<HtmlCanvasElement>()
            .expect("dyn_into canvas element");
        fg_canvas.set_width(window_width);
        fg_canvas.set_height(window_height);
        fg_canvas
            .style()
            .set_property("position", "absolute")
            .unwrap();
        fg_canvas.style().set_property("z-index", "1").unwrap();

        document
            .body()
            .unwrap()
            .append_child(&bg_canvas)
            .expect("doc append bg_canvas");
        document
            .body()
            .unwrap()
            .append_child(&fg_canvas)
            .expect("doc append fg_canvas");

        let bg_ctx = bg_canvas
            .get_context("2d")
            .expect("get 2d ctx")
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("dyn_into 2d ctx");
        let fg_ctx = fg_canvas
            .get_context("2d")
            .expect("get 2d ctx")
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("dyn_into 2d ctx");

        fg_ctx.set_font("36px monospace");

        let width = fg_canvas.width();
        let height = fg_canvas.height();

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

        let tower_size = height as f64 * 0.08;
        let buttons = vec![
            Button::new(
                Rect::new(
                    width as f64 - 5.0 - tower_size,
                    tower_size * 0.05,
                    tower_size,
                    tower_size,
                ),
                ButtonType::WaterGun,
                "WaterGunTop",
            ),
            Button::new(
                Rect::new(
                    width as f64 - 10.0 - tower_size * 2.0,
                    tower_size * 0.05,
                    tower_size,
                    tower_size,
                ),
                ButtonType::AcidTower,
                "AcidTowerTop",
            ),
            Button::new(
                Rect::new(
                    width as f64 - 15.0 - tower_size * 3.0,
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
                Turn::new((width as f64 * 0.22, height as f64 * 0.26), Direction::Down),
                Turn::new(
                    (width as f64 * 0.22, height as f64 * 0.72),
                    Direction::Right,
                ),
                Turn::new((width as f64 * 0.43, height as f64 * 0.72), Direction::Up),
                Turn::new(
                    (width as f64 * 0.43, height as f64 * 0.25),
                    Direction::Right,
                ),
                Turn::new((width as f64 * 0.62, height as f64 * 0.25), Direction::Down),
                Turn::new(
                    (width as f64 * 0.62, height as f64 * 0.72),
                    Direction::Right,
                ),
                Turn::new((width as f64 * 0.85, height as f64 * 0.72), Direction::Up),
                Turn::new(
                    (width as f64 * 0.85, height as f64 * 0.26),
                    Direction::Right,
                ),
            ]),
            plane_size: height as f64 * 0.05,
            tower_size,
            ui_text_size: height as f64 * 0.03,
            width,
            height,
            bg_ctx,
            fg_ctx,
            mouse: Mouse::new(),
            sprites,
            planes: Vec::with_capacity(50),
            towers: Vec::with_capacity(10),
            buttons,
            round: 1,
            round_start_time: Date::now(),
            hp: 100,
            cash: 100,
        }
    }
    fn spawn_basics(&mut self, n: i32, spacing: f64) {
        for x in 1..=n {
            self.planes.push(PaperPlane::new_basic(Rect::new(
                self.plane_size * -x as f64 * spacing,
                self.height as f64 * 0.26,
                self.plane_size,
                self.plane_size,
            )));
        }
    }

    fn spawn_bullets(&mut self, n: i32, spacing: f64) {
        for x in 1..=n {
            self.planes.push(PaperPlane::new_bullet(Rect::new(
                self.plane_size * -x as f64 * spacing,
                self.height as f64 * 0.26,
                self.plane_size,
                self.plane_size,
            )));
        }
    }

    fn spawn_gliders(&mut self, n: i32, spacing: f64) {
        for x in 1..=n {
            self.planes.push(PaperPlane::new_glider(Rect::new(
                self.plane_size * -x as f64 * spacing,
                self.height as f64 * 0.26,
                self.plane_size,
                self.plane_size,
            )));
        }
    }

    fn spawn_blimps(&mut self, n: i32, spacing: f64) {
        for x in 1..=n {
            self.planes.push(PaperPlane::new_blimp(Rect::new(
                self.plane_size * -x as f64 * spacing,
                self.height as f64 * 0.26,
                self.plane_size,
                self.plane_size,
            )));
        }
    }

    fn make_planes(&mut self) {
        let elapsed = Date::now() - self.round_start_time;

        match self.round {
            1 if elapsed >= 1000.0 => {
                self.round_start_time = Date::now();
                self.spawn_bullets(25, 2.0);
                self.round += 1;
            }
            2 if elapsed >= 10000.0 => {
                self.round_start_time = Date::now();
                self.spawn_basics(25, 2.0);
                self.round += 1;
            }
            3 if elapsed >= 10000.0 => {
                self.round_start_time = Date::now();
                self.spawn_bullets(25, 2.0);
                self.spawn_gliders(25, 2.0);
                self.round += 1;
            }
            4 if elapsed >= 10000.0 => {
                self.round_start_time = Date::now();
                self.spawn_blimps(25, 2.0);
                self.round += 1;
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
            tower.draw(&self.fg_ctx, &self.sprites).expect("tower draw");

            for plane in self.planes.iter_mut() {
                tower.damage(plane);
            }
        }
    }

    /// Render all planes
    fn render_planes(&mut self) {
        for plane in self.planes.iter_mut() {
            plane.draw(&self.fg_ctx, &self.sprites).expect("Plane draw");
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
            } else if self.planes[i].x() >= self.width.into() {
                self.hp -= self.planes[i].damage() as i32;
                self.planes.remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// Render text found in the top bar
    fn render_text(&self) {
        self.fg_ctx.begin_path();
        self.fg_ctx.set_fill_style(&JsValue::from_str("#111111"));
        self.fg_ctx
            .set_font(&format!("{}px sans-serif", self.ui_text_size));
        self.fg_ctx
            .fill_text(&format!("❤️ : {}", self.hp), 10.0, self.ui_text_size + 5.0)
            .expect("display hp");
        self.fg_ctx
            .fill_text(&format!("$  : {}", self.cash), 10.0, self.tower_size - 5.0)
            .expect("display cash");
        self.fg_ctx.close_path();
    }

    /// Render the top bar
    fn render_top_bar(&self) -> Result<(), JsValue> {
        self.fg_ctx.begin_path();
        self.fg_ctx.set_fill_style(&JsValue::from_str("#555555"));
        self.fg_ctx
            .rect(0.0, 0.0, self.width as f64, self.tower_size + 10.0);
        self.fg_ctx.fill();
        self.fg_ctx.close_path();

        for button in self.buttons.iter() {
            button.draw(&self.fg_ctx, &self.sprites)?;
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

        self.fg_ctx
            .clear_rect(0.0, 0.0, self.width.into(), self.height.into());

        self.fg_ctx
            .draw_image_with_html_image_element_and_dw_and_dh(
                self.sprites.get("Map").unwrap(),
                0.0,
                0.0,
                self.width as f64,
                self.height as f64,
            )
            .expect("drawing Map");

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
