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
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);

    fn requestAnimationFrame(closure: &Closure<dyn FnMut()>) -> u32;
    fn cancelAnimationFrame(id: u32);
}

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();

    let game = game::Game::new("canvas", "button", 30, 2, "#333", "#DDD");
    game.attach_onclicks();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let interval_id = Rc::new(RefCell::new(None));
    let frame_id = Rc::new(RefCell::new(None));

    let interval_id_cell = interval_id.clone();
    let frame_id_cell = frame_id.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        game.tick();
        // utils::log(&format!("{}", game.paused.borrow().clone()));
        if game.paused.borrow().clone() {
            frame_id_cell
                .borrow_mut()
                .replace(requestAnimationFrame(f.borrow().as_ref().unwrap()));
            match interval_id_cell.borrow().clone() {
                Some(x) => clearInterval(x),
                None => {}
            }
        } else {
            interval_id_cell.replace(Some(setInterval(f.borrow().as_ref().unwrap(), 1000)));
            match frame_id_cell.borrow().clone() {
                Some(x) => cancelAnimationFrame(x),
                None => {}
            }
        }
    }) as Box<dyn FnMut()>));

    frame_id
        .borrow_mut()
        .replace(requestAnimationFrame(g.borrow().as_ref().unwrap()));
}
