use std::collections::HashMap;

use wasm_bindgen::{prelude::*, JsCast};

use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

use crate::{
    entity::{Button, PaperPlane, Tower},
    types::{Mouse, Square},
    utils::set_panic_hook,
};

const PLANE_SIZE: f64 = 50.0;
const TOWER_SIZE: f64 = 75.0;

enum Selected {
    WaterGun,
    AcidTower,
    SodaMaker,
    None,
}

#[wasm_bindgen]
pub struct Game {
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
    selected: Selected,
}

#[wasm_bindgen]
impl Game {
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

        let img = HtmlImageElement::new_with_width_and_height(50, 50).expect("plane image");
        img.set_src("static/plane.png");
        sprites.insert(String::from("plane"), img);

        let img = HtmlImageElement::new_with_width_and_height(50, 50).expect("WaterGunBase image");
        img.set_src("static/WaterGunBase.png");
        sprites.insert(String::from("WaterGunBase"), img);

        let img = HtmlImageElement::new_with_width_and_height(50, 50).expect("WaterGunTop image");
        img.set_src("static/WaterGunTop.png");
        sprites.insert(String::from("WaterGunTop"), img);

        let img = HtmlImageElement::new_with_width_and_height(50, 50).expect("WaterGunBlast image");
        img.set_src("static/WaterGunBlast.png");
        sprites.insert(String::from("WaterGunBlast"), img);

        let img = HtmlImageElement::new_with_width_and_height(50, 50).expect("AcidTowerTop image");
        img.set_src("static/AcidTowerTop.png");
        sprites.insert(String::from("AcidTowerTop"), img);

        let img = HtmlImageElement::new_with_width_and_height(50, 50).expect("AcidTowerTop2 image");
        img.set_src("static/AcidTowerTop2.png");
        sprites.insert(String::from("AcidTowerTop2"), img);

        let img = HtmlImageElement::new_with_width_and_height(50, 50).expect("SodaMakerTop image");
        img.set_src("static/SodaMakerTop.png");
        sprites.insert(String::from("SodaMakerTop"), img);

        let img =
            HtmlImageElement::new_with_width_and_height(50, 50).expect("SparklingWaterTop image");
        img.set_src("static/SparklingWaterTop.png");
        sprites.insert(String::from("SparklingWaterTop"), img);

        let img = HtmlImageElement::new_with_width_and_height(50, 50).expect("RootBeerTop image");
        img.set_src("static/RootBeerTop.png");
        sprites.insert(String::from("RootBeerTop"), img);

        let mut planes = Vec::with_capacity(100);
        for i in 0..100 {
            planes.push(PaperPlane::new(
                Square::new(
                    -i as f64 * 75.0 + 50.0,
                    canvas.height() as f64 / ((i % 2 * 2) as f64 + 1.5) + i as f64,
                    PLANE_SIZE,
                ),
                50,
            ));
        }

        let towers = vec![
            Tower::new_water_gun(Square::new(500.0, canvas.height() as f64 / 2.0, TOWER_SIZE)),
            Tower::new_acid_tower(Square::new(
                1500.0,
                canvas.height() as f64 / 2.0,
                TOWER_SIZE,
            )),
        ];

        let buttons = vec![
            Button::new(
                Square::new(canvas.width() as f64 - 5.0 - TOWER_SIZE, 0.0, TOWER_SIZE),
                "WaterGunTop",
            ),
            Button::new(
                Square::new(
                    canvas.width() as f64 - 5.0 - TOWER_SIZE * 2.0,
                    0.0,
                    TOWER_SIZE,
                ),
                "AcidTowerTop",
            ),
            Button::new(
                Square::new(
                    canvas.width() as f64 - 5.0 - TOWER_SIZE * 3.0,
                    0.0,
                    TOWER_SIZE,
                ),
                "SodaMakerTop",
            ),
        ];

        Self {
            ui_text_size: canvas.width() as f64 * 0.015,
            canvas,
            ctx,
            mouse: Mouse::new(),
            sprites,
            planes,
            towers,
            buttons,
            hp: 100,
            cash: 0,
            selected: Selected::None,
        }
    }

    fn events(&mut self) {
        if self.mouse.up {
            let selection = if self.mouse.x > self.canvas.width() as f64 - TOWER_SIZE
                && self.mouse.x < self.canvas.width() as f64
                && self.mouse.y < TOWER_SIZE
            {
                Selected::WaterGun
            } else {
                Selected::None
            };
            match self.selected {
                Selected::WaterGun => {
                    if self.cash >= 20 && self.mouse.y > TOWER_SIZE * 2.0 {
                        self.towers.push(Tower::new_water_gun(Square::new(
                            self.mouse.x - TOWER_SIZE / 2.0,
                            self.mouse.y - TOWER_SIZE / 2.0,
                            TOWER_SIZE,
                        )));
                        self.cash -= 20;
                    }
                }
                Selected::AcidTower => {
                    if self.cash >= 20 && self.mouse.y > TOWER_SIZE * 2.0 {
                        self.towers.push(Tower::new_acid_tower(Square::new(
                            self.mouse.x - TOWER_SIZE / 2.0,
                            self.mouse.y - TOWER_SIZE / 2.0,
                            TOWER_SIZE,
                        )));
                        self.cash -= 20;
                    }
                }
                Selected::SodaMaker => {
                    if self.cash >= 20 && self.mouse.y > TOWER_SIZE * 2.0 {
                        self.towers.push(Tower::new_soda_maker(Square::new(
                            self.mouse.x - TOWER_SIZE / 2.0,
                            self.mouse.y - TOWER_SIZE / 2.0,
                            TOWER_SIZE,
                        )));
                        self.cash -= 20;
                    }
                }
                _ => (),
            }
            self.selected = selection;
        }
    }

    fn render_text(&self) {
        self.ctx.begin_path();
        self.ctx.set_fill_style(&JsValue::from_str("#000000"));
        self.ctx
            .set_font(&format!("{}px sans-serif", self.ui_text_size));
        self.ctx
            .fill_text(&format!("HP: {}", self.hp), 10.0, 40.0)
            .expect("display hp");
        self.ctx
            .fill_text(&format!("Cash: {}", self.cash), 10.0, 80.0)
            .expect("display cash");
        self.ctx.close_path();
    }

    fn render_towers(&mut self) {
        for tower in self.towers.iter_mut() {
            tower.events(&self.mouse);
            tower.draw(&self.ctx, &self.sprites).expect("tower draw");
            for plane in self.planes.iter_mut() {
                tower.damage(plane);
            }
        }
    }

    fn remove_planes(&mut self) {
        let canvas_width = self.canvas.width();

        let mut i = 0;
        while i != self.planes.len() {
            if self.planes[i].hp() <= 0 {
                self.planes.remove(i);
                self.cash += 10;
            } else if self.planes[i].x() >= canvas_width.into() {
                self.planes.remove(i);
                self.hp -= 1;
            } else {
                i += 1;
            }
        }
    }

    fn render_planes(&mut self) {
        for plane in self.planes.iter_mut() {
            plane
                .draw(&self.ctx, self.sprites.get("plane").unwrap())
                .expect("plane draw");
            plane.fly();
        }
    }

    fn render_top_bar(&self) -> Result<(), JsValue> {
        self.ctx.begin_path();
        self.ctx.set_fill_style(&JsValue::from_str("#555555"));
        self.ctx
            .rect(0.0, 0.0, self.canvas.width() as f64, TOWER_SIZE + 10.0);
        self.ctx.fill();
        self.ctx.close_path();

        for button in self.buttons.iter() {
            button.draw(&self.ctx, &self.sprites)?;
        }

        if let Selected::WaterGun = self.selected {
            self.ctx.begin_path();
            self.ctx.set_stroke_style(&JsValue::from_str("#000000"));
            self.ctx.rect(
                self.canvas.width() as f64 - TOWER_SIZE - 5.0,
                0.0,
                TOWER_SIZE,
                TOWER_SIZE + 9.0,
            );
            self.ctx.stroke();
            self.ctx.close_path();
        }

        self.render_text();
        Ok(())
    }

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

        self.remove_planes();
        self.render_planes();

        self.render_top_bar().expect("render top bar");
        Ok(())
    }
}
