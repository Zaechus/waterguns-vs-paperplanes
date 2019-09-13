use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct PaperPlane {
    img: HtmlCanvasElement,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    hp: i32,
}

#[wasm_bindgen]
impl PaperPlane {
    pub fn new(img: HtmlCanvasElement, x: f64, y: f64, w: f64, h: f64, hp: i32) -> Self {
        Self {
            img,
            x,
            y,
            w,
            h,
            hp,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn fly(&mut self) {
        self.x += 1.0;
        self.y += 1.0;
    }

    pub fn draw(&self, ctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
        ctx.draw_image_with_html_canvas_element_and_dw_and_dh(
            &self.img, self.x, self.y, self.w, self.h,
        )
    }
}
