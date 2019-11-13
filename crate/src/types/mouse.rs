use wasm_bindgen::prelude::*;

use crate::types::Rect;

/// A type representing the state of Mouse events
#[wasm_bindgen]
#[derive(Debug)]
pub struct Mouse {
    rect: Rect,
    down: bool,
    up: bool,
}

impl Mouse {
    /// Construct a new Mouse
    pub fn new() -> Self {
        Self {
            rect: Rect::new(0.0, 0.0, 1.0, 1.0),
            down: false,
            up: false,
        }
    }

    /// Returns the mouse x position
    pub fn x(&self) -> f64 {
        self.rect.x()
    }
    /// Returns the mouse y position
    pub fn y(&self) -> f64 {
        self.rect.y()
    }
    /// Returns true on a mousedown event
    pub fn down(&self) -> bool {
        self.down
    }
    /// Returns true on a mouseup event
    pub fn up(&self) -> bool {
        self.up
    }

    /// Update the state of the mouse
    pub fn update(&mut self, x: f64, y: f64, mouse_down: bool, mouse_up: bool) {
        self.rect.set_pos(x, y);
        self.down = mouse_down;
        self.up = mouse_up;
    }

    /// Returns true if the mouse (x, y) position is inside of the referenced Rect
    pub fn inside(&self, rect: &Rect) -> bool {
        self.x() > rect.x()
            && self.y() > rect.y()
            && self.x() < rect.x() + rect.w()
            && self.y() < rect.y() + rect.h()
    }
}

impl Default for Mouse {
    fn default() -> Self {
        Self::new()
    }
}
