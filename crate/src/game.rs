use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

use crate::entity::*;
use crate::types::Square;
use crate::utils::set_panic_hook;

const PLANE_SIZE: f64 = 50.0;
const TOWER_SIZE: f64 = 75.0;

#[wasm_bindgen]
pub struct Game {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    mouse_x: f64,
    mouse_y: f64,
    mouse_down: bool,
    mouse_up: bool,
    sprites: HashMap<String, HtmlImageElement>,
    planes: Vec<PaperPlane>,
    towers: Vec<Tower>,
    hp: i32,
    cash: i32,
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

        let plane_image = HtmlImageElement::new_with_width_and_height(50, 50).expect("plane image");
        plane_image.set_src("static/plane.png");
        sprites.insert(String::from("plane"), plane_image);

        let watergun_image =
            HtmlImageElement::new_with_width_and_height(50, 50).expect("WaterGunBase image");
        watergun_image.set_src("static/WaterGunBase.png");
        sprites.insert(String::from("WaterGunBase"), watergun_image);

        let watergun_image =
            HtmlImageElement::new_with_width_and_height(50, 50).expect("WaterGunTop image");
        watergun_image.set_src("static/WaterGunTop.png");
        sprites.insert(String::from("WaterGunTop"), watergun_image);

        let watergun_shooting_image =
            HtmlImageElement::new_with_width_and_height(50, 50).expect("WaterGunBlast image");
        watergun_shooting_image.set_src("static/WaterGunBlast.png");
        sprites.insert(String::from("WaterGunBlast"), watergun_shooting_image);

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
        // for i in 0..50 {
        //     planes.push(PaperPlane::new(
        //         Square::new(
        //             -i as f64 * 125.0 + 100.0,
        //             canvas.height() as f64 / 3.5 + i as f64,
        //             PLANE_SIZE,
        //         ),
        //         50,
        //     ));
        // }
        let mut towers = Vec::with_capacity(2);
        for i in 0..2 {
            towers.push(Tower::new(
                Square::new(
                    500.0 + i as f64 * 1000.0,
                    canvas.height() as f64 / 2.0,
                    TOWER_SIZE,
                ),
                15,
                250.0,
            ));
        }
        Self {
            canvas,
            ctx,
            mouse_down: false,
            mouse_up: false,
            mouse_x: 0.0,
            mouse_y: 0.0,
            sprites,
            planes,
            towers,
            hp: 100,
            cash: 0,
        }
    }

    fn render_text(&self) {
        self.ctx.begin_path();
        self.ctx.set_fill_style(&JsValue::from_str("#000000"));
        self.ctx
            .fill_text(&format!("HP: {}", self.hp), 10.0, 40.0)
            .expect("display hp");
        self.ctx
            .fill_text(&format!("Cash: {}", self.cash), 10.0, 80.0)
            .expect("display cash");
        self.ctx
            .fill_text(
                &format!("X, Y: {}, {}", self.mouse_x, self.mouse_y),
                10.0,
                120.0,
            )
            .expect("display mouseXY");
        self.ctx.close_path();
    }

    fn render_towers(&mut self) {
        for tower in self.towers.iter_mut() {
            tower
                .draw(
                    &self.ctx,
                    self.sprites.get("WaterGunBase").unwrap(),
                    self.sprites.get("WaterGunTop").unwrap(),
                    self.sprites.get("WaterGunBlast").unwrap(),
                )
                .expect("tower draw");
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

    fn update_mouse(&mut self, x: f64, y: f64, mouse_down: bool, mouse_up: bool) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.mouse_down = mouse_down;
        self.mouse_up = mouse_up;
    }

    pub fn draw(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        mouse_down: bool,
        mouse_up: bool,
    ) -> Result<(), JsValue> {
        self.update_mouse(mouse_x, mouse_y, mouse_down, mouse_up);

        self.ctx.clear_rect(
            0.0,
            0.0,
            self.canvas.width().into(),
            self.canvas.height().into(),
        );

        if self.mouse_up && self.cash > 20 {
            self.towers.push(Tower::new(
                Square::new(
                    self.mouse_x - TOWER_SIZE / 2.0,
                    self.mouse_y - TOWER_SIZE / 2.0,
                    TOWER_SIZE,
                ),
                15,
                250.0,
            ));
            self.cash -= 20;
        }

        self.render_text();

        self.render_towers();

        self.remove_planes();
        self.render_planes();
        Ok(())
    }
}
