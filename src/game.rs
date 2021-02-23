use crate::canvas;

use wasm_bindgen::JsValue;

pub fn run(canvas_id: &str, cell_size: u32, padding: u32, alive_color: &str, dead_color: &str) {
    let config = canvas::CanvasConfig {
        cell_size: cell_size as f64,
        padding: padding as f64,
        alive_color: JsValue::from_str(alive_color),
        dead_color: JsValue::from_str(dead_color),
    };
    let canvas = canvas::Canvas::new(canvas_id, config);

    let state_vec = vec![vec![false; canvas.x_length as usize]; canvas.y_length as usize];
    canvas.draw(state_vec);
}
