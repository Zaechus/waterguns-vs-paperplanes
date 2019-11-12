use wasm_bindgen::prelude::*;

/// A sub-type of entities used for a logical way to apply construction of new entities
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    center_x: f64,
    center_y: f64,
}

impl Rect {
    /// Constuct a new Rect from x, y, and size
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            x,
            y,
            w,
            h,
            center_x: x + w * 0.5,
            center_y: y + h * 0.5,
        }
    }

    /// Returns the x coordinate of the Rect
    pub fn x(&self) -> f64 {
        self.x
    }
    /// Returns the y coordinate of the Rect
    pub fn y(&self) -> f64 {
        self.y
    }
    /// Returns the width of the Rect
    pub fn w(&self) -> f64 {
        self.w
    }
    /// Returns the height of the Rect
    pub fn h(&self) -> f64 {
        self.h
    }
    /// Returns the x coordinate of the center of the Rect
    pub fn center_x(&self) -> f64 {
        self.center_x
    }
    /// Returns the y coordinate of the center of the Rect
    pub fn center_y(&self) -> f64 {
        self.center_y
    }

    /// Updates the x, y, center x, and center y of the Rect according to the x an y paramaters
    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.x = x;
        self.center_x = x + self.w * 0.5;
        self.y = y;
        self.center_y = y + self.h * 0.5;
    }
}
