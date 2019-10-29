use std::collections::HashMap;

use wasm_bindgen::{prelude::*, JsCast};

use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

use crate::{
    entity::{PaperPlane, Tower},
    types::{Mouse, Square},
    utils::set_panic_hook,
};

const PLANE_SIZE: f64 = 50.0;
const TOWER_SIZE: f64 = 75.0;

#[wasm_bindgen]
pub struct Game {
    ui_text_size: f64,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    mouse: Mouse,
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

        let watergun_base =
            HtmlImageElement::new_with_width_and_height(50, 50).expect("WaterGunBase image");
        watergun_base.set_src("static/WaterGunBase.png");
        sprites.insert(String::from("WaterGunBase"), watergun_base);

        let watergun_top =
            HtmlImageElement::new_with_width_and_height(50, 50).expect("WaterGunTop image");
        watergun_top.set_src("static/WaterGunTop.png");
        sprites.insert(String::from("WaterGunTop"), watergun_top);

        let watergun_blast =
            HtmlImageElement::new_with_width_and_height(50, 50).expect("WaterGunBlast image");
        watergun_blast.set_src("static/WaterGunBlast.png");
        sprites.insert(String::from("WaterGunBlast"), watergun_blast);

        let acidtower_top =
            HtmlImageElement::new_with_width_and_height(50, 50).expect("AcidTowerTop image");
        acidtower_top.set_src("static/AcidTowerTop.png");
        sprites.insert(String::from("AcidTowerTop"), acidtower_top);

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
        let mut towers = Vec::with_capacity(2);

        towers.push(Tower::new_water_gun(Square::new(
            500.0,
            canvas.height() as f64 / 2.0,
            TOWER_SIZE,
        )));
        towers.push(Tower::new_acid_tower(Square::new(
            1500.0,
            canvas.height() as f64 / 2.0,
            TOWER_SIZE,
        )));

        Self {
            ui_text_size: canvas.width() as f64 * 0.015,
            canvas,
            ctx,
            mouse: Mouse::new(),
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
            .set_font(&format!("{}px sans-serif", self.ui_text_size));
        self.ctx
            .fill_text(&format!("HP: {}", self.hp), 10.0, 40.0)
            .expect("display hp");
        self.ctx
            .fill_text(&format!("Cash: {}", self.cash), 10.0, 80.0)
            .expect("display cash");
        self.ctx
            .fill_text(
                &format!(
                    "X, Y: {}, {}, {}, {}",
                    self.mouse.x, self.mouse.y, self.mouse.down, self.mouse.up
                ),
                10.0,
                120.0,
            )
            .expect("display mouseXY");
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

        // if self.mouse.up && self.cash >= 20 {
        //     self.towers.push(Tower::new(
        //         Square::new(
        //             self.mouse.x - TOWER_SIZE / 2.0,
        //             self.mouse.y - TOWER_SIZE / 2.0,
        //             TOWER_SIZE,
        //         ),
        //         15,
        //         250.0,
        //     ));
        //     self.cash -= 20;
        // }

        self.render_text();

        self.render_towers();

        self.remove_planes();
        self.render_planes();
        Ok(())
    }
}
