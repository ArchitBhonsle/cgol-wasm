mod canvas;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Declare all the JS functions here
}

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();

    let canvas = canvas::Canvas::new("canvas", 100);
    let state_vec = vec![vec![false; canvas.x_length as usize]; canvas.y_length as usize];
    canvas.draw(state_vec);
}
