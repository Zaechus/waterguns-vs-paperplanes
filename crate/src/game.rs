use std::collections::HashMap;

use wasm_bindgen::{prelude::*, JsCast};

use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

use crate::{
    entity::{Button, PaperPlane, Tower},
    types::{ButtonType, HitPoints, Mouse, PlanePath, Rect, TowerStatus},
    utils::set_panic_hook,
};

const WATERGUN_COST: i32 = 10;
const ACID_COST: i32 = 30;
const SODA_COST: i32 = 50;

/// A struct that handles the workings of the game
#[wasm_bindgen]
pub struct Game {
    plane_size: f64,
    tower_size: f64,
    ui_text_size: f64,

    width: f64,
    height: f64,
    mouse: Mouse,
    bg_canvas: HtmlCanvasElement,
    fg_canvas: HtmlCanvasElement,
    fg_ctx: CanvasRenderingContext2d,

    sprites: HashMap<String, HtmlImageElement>,
    planes: Vec<PaperPlane>,
    towers: Vec<Tower>,
    buttons: Vec<Button>,
    path: PlanePath,

    round: u32,
    round_start_tic: u32,
    tic: u32,
    hp: HitPoints,
    cash: i32,
}

#[wasm_bindgen]
impl Game {
    /// Setup a new game
    pub fn new() -> Self {
        set_panic_hook();
        let document = window().unwrap().document().unwrap();
        document
            .body()
            .unwrap()
            .style()
            .set_property("margin", "0")
            .unwrap();
        let width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        let width = if width >= 1366.0 { 1366.0 } else { width };
        let height = window().unwrap().inner_height().unwrap().as_f64().unwrap();
        let height = if width >= 768.0 { 768.0 } else { height };

        let tower_size = height as f64 * 0.08;

        let canvas_css = "
            padding: 0;
            margin: auto;
            display: block;
            position: absolute;
            top: 0;
            bottom: 0;
            left: 0;
            right: 0;
        ";

        let bg_canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        bg_canvas.set_width(width.floor() as u32);
        bg_canvas.set_height((tower_size * 1.2).floor() as u32);
        bg_canvas
            .style()
            .set_css_text(&format!("z-index: 1;\n{}", canvas_css));
        document.body().unwrap().append_child(&bg_canvas).unwrap();
        let bg_ctx = bg_canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        bg_ctx.begin_path();
        bg_ctx.set_fill_style(&JsValue::from_str("#555555"));
        bg_ctx.rect(
            0.0,
            0.0,
            bg_canvas.width() as f64,
            bg_canvas.height() as f64,
        );
        bg_ctx.fill();
        bg_ctx.close_path();

        let fg_canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        fg_canvas.set_width(width.floor() as u32);
        fg_canvas.set_height(height.floor() as u32);
        fg_canvas.style().set_css_text(&format!(
            "z-index: 2;
            background: #4c7942;
            {}",
            canvas_css
        ));
        document.body().unwrap().append_child(&fg_canvas).unwrap();
        let fg_ctx = fg_canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        fg_ctx.set_font("36px monospace");

        // Populate the sprite map
        let mut sprites = HashMap::new();
        let sprite_names = [
            "Map",
            "Plane",
            "Bullet",
            "BulletRedux",
            "Glider",
            "GliderRedux",
            "WaterBomb",
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

        // Create Tower Buttons
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
            path: PlanePath::new_main_path(width as f64, height as f64),
            plane_size: height as f64 * 0.05,
            tower_size,
            ui_text_size: height as f64 * 0.03,
            width,
            height,
            bg_canvas,
            fg_canvas,
            fg_ctx,
            mouse: Mouse::new(),
            sprites,
            planes: Vec::with_capacity(50),
            towers: Vec::with_capacity(10),
            buttons,
            round: 1,
            round_start_tic: 0,
            tic: 1,
            hp: HitPoints::new(100),
            cash: WATERGUN_COST,
        }
    }

    fn plane_start(&self, x: i32, spacing: f64) -> Rect {
        Rect::new(
            self.plane_size * -x as f64 * spacing,
            self.height * 0.26,
            self.plane_size,
            self.plane_size,
        )
    }
    fn spawn_basics(&mut self, n: i32, spacing: f64) {
        for x in 1..=n {
            self.planes
                .push(PaperPlane::new_basic(self.plane_start(x, spacing)));
        }
    }
    fn spawn_bullets(&mut self, n: i32, spacing: f64) {
        for x in 1..=n {
            self.planes
                .push(PaperPlane::new_bullet(self.plane_start(x, spacing)));
        }
    }
    fn spawn_bullet_reduxes(&mut self, n: i32, spacing: f64) {
        for x in 1..=n {
            self.planes
                .push(PaperPlane::new_bullet_redux(self.plane_start(x, spacing)));
        }
    }
    fn spawn_gliders(&mut self, n: i32, spacing: f64) {
        for x in 1..=n {
            self.planes
                .push(PaperPlane::new_glider(self.plane_start(x, spacing)));
        }
    }
    fn spawn_glider_reduxes(&mut self, n: i32, spacing: f64) {
        for x in 1..=n {
            self.planes
                .push(PaperPlane::new_glider_redux(self.plane_start(x, spacing)));
        }
    }
    fn spawn_waterbombs(&mut self, n: i32, spacing: f64) {
        for x in 1..=n {
            self.planes
                .push(PaperPlane::new_waterbomb(self.plane_start(x, spacing)));
        }
    }
    fn spawn_blimps(&mut self, n: i32, spacing: f64) {
        for x in 1..=n {
            self.planes
                .push(PaperPlane::new_blimp(self.plane_start(x, spacing)));
        }
    }

    fn make_planes(&mut self) {
        let elapsed = self.tic - self.round_start_tic;

        match self.round {
            1 if elapsed >= 100 => {
                self.round_start_tic = 0;
                self.tic = 0;
                self.spawn_basics(20, 3.0);
                self.round += 1;
            }
            2 if elapsed >= 2000 => {
                self.round_start_tic = 0;
                self.tic = 0;
                self.spawn_basics(25, 2.5);
                self.round += 1;
            }
            3 if elapsed >= 3000 => {
                self.round_start_tic = 0;
                self.tic = 0;
                self.spawn_bullets(25, 2.0);
                self.spawn_gliders(25, 2.0);
                self.round += 1;
            }
            4 if elapsed >= 3000 => {
                self.round_start_tic = 0;
                self.tic = 0;
                self.spawn_bullet_reduxes(25, 3.0);
                self.spawn_glider_reduxes(25, 2.0);
                self.spawn_blimps(25, 3.0);
                self.spawn_waterbombs(25, 2.0);
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
    fn render_towers(&mut self) -> Result<(), JsValue> {
        for tower in self.towers.iter_mut() {
            tower.events(&self.mouse, &mut self.cash);
            tower.draw(&self.fg_ctx, &self.sprites)?;

            for plane in self.planes.iter_mut() {
                tower.damage(plane);
            }
        }
        Ok(())
    }

    /// Render all planes
    fn render_planes(&mut self) -> Result<(), JsValue> {
        for plane in self.planes.iter_mut() {
            plane.draw(&self.fg_ctx, &self.sprites)?;
            plane.fly(&self.path);
        }
        Ok(())
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
            if self.planes[i].hp().is_dead() {
                self.cash += self.planes[i].bounty() as i32;
                self.planes.remove(i);
            } else if self.planes[i].x() >= self.width.into() {
                self.hp.take_damage(self.planes[i].damage());
                self.planes.remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// Render text found in the top bar
    fn render_text(&self) -> Result<(), JsValue> {
        self.fg_ctx.begin_path();
        self.fg_ctx.set_fill_style(&JsValue::from_str("#111111"));
        self.fg_ctx
            .set_font(&format!("{}px sans-serif", self.ui_text_size));
        self.fg_ctx.fill_text(
            &format!("❤️ : {}", self.hp.curr_hp()),
            10.0,
            self.ui_text_size + 5.0,
        )?;
        self.fg_ctx
            .fill_text(&format!("$  : {}", self.cash), 10.0, self.tower_size - 5.0)?;
        self.fg_ctx.close_path();
        Ok(())
    }

    /// Render the top bar
    fn render_top_bar(&self) -> Result<(), JsValue> {
        for button in self.buttons.iter() {
            button.draw(&self.fg_ctx, &self.sprites)?;
        }
        self.render_text()?;
        Ok(())
    }

    #[wasm_bindgen(js_name = isDefeated)]
    pub fn is_defeated(&self) -> bool {
        self.hp.is_dead()
    }

    /// Render an increment of the Game
    pub fn draw(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        mouse_down: bool,
        mouse_up: bool,
    ) -> Result<(), JsValue> {
        let canvas_rect = self.fg_canvas.get_bounding_client_rect();
        self.mouse.update(
            mouse_x - canvas_rect.left(),
            mouse_y - canvas_rect.top(),
            mouse_down,
            mouse_up,
        );

        self.fg_ctx
            .clear_rect(0.0, 0.0, self.width.into(), self.height.into());

        self.fg_ctx
            .draw_image_with_html_canvas_element(&self.bg_canvas, 0.0, 0.0)?;

        self.fg_ctx
            .draw_image_with_html_image_element_and_dw_and_dh(
                self.sprites.get("Map").unwrap(),
                0.0,
                0.0,
                self.width as f64,
                self.height as f64,
            )?;

        self.make_planes();

        self.events();

        self.render_towers()?;
        self.render_planes()?;

        self.remove_towers();
        self.remove_planes();

        self.render_top_bar()?;

        self.tic += 1;
        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
