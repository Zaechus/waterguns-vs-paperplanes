/// Various entities found within the game
mod entity;
/// The Game struct
mod game;
/// Custom types used within entities in the game
mod types;
/// Useful miscellaneous functions
mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Sets the panic hook for easy web console debugging
#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}
