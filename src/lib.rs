mod canvas;
mod game;
mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // js functions here
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

#[wasm_bindgen]
pub fn start(
    canvas_id: &str,
    button_id: &str,
    fps_input_id: &str,
    randomize_id: &str,
    clear_id: &str,
    cell_size: u32,
    padding: u32,
) -> Result<(), JsValue> {
    utils::set_panic_hook();
    let game = game::Game::new(
        canvas_id,
        button_id,
        fps_input_id,
        randomize_id,
        clear_id,
        cell_size,
        padding,
    );
    game.attach_listeners();

    let fps_cell = game.fps.clone();
    let mut previous = js_sys::Date::now();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        request_animation_frame(f.borrow().as_ref().unwrap());

        let fps = fps_cell.borrow();
        let now = js_sys::Date::now();
        if now < previous + (1000.0 / *fps) {
            return ();
        }
        previous = now;

        game.tick();
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
