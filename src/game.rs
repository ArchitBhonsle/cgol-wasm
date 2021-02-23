use crate::canvas;
use crate::utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

pub fn run(canvas_id: &str, cell_size: u32, padding: u32, alive_color: &str, dead_color: &str) {
    let config = canvas::CanvasConfig {
        cell_size: cell_size as f64,
        padding: padding as f64,
        alive_color: JsValue::from_str(alive_color),
        dead_color: JsValue::from_str(dead_color),
    };
    let canvas = canvas::Canvas::new(canvas_id, config);
    let canvas = attach_onclick(canvas);

    let state_vec = vec![vec![false; canvas.x_length as usize]; canvas.y_length as usize];
    canvas.draw(state_vec);
}

fn attach_onclick(canvas: canvas::Canvas) -> canvas::Canvas {
    let (x_just, y_just) = canvas.get_justs();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let x = event.offset_x() as f64 - x_just;
        let y = event.offset_y() as f64 - y_just;
        utils::log(&format!("{} {}", x, y));
    }) as Box<dyn FnMut(_)>);

    canvas
        .canvas
        .set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    canvas
}
