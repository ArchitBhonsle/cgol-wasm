use crate::utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

pub struct Canvas {
    pub ctx: web_sys::CanvasRenderingContext2d,
    pub x_length: f64,
    pub y_length: f64,
    cell_size: f64,
    x_just: f64,
    y_just: f64,
    alive_color: JsValue,
    dead_color: JsValue,
}

impl Canvas {
    pub fn new(canvas_id: &str, cell_size: f64) -> Canvas {
        let canvas: web_sys::HtmlCanvasElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let height = canvas.height() as f64;
        let width = canvas.width() as f64;

        let ctx: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let x_length = (width / cell_size).floor();
        let y_length = (height / cell_size).floor();

        let x_just = (width - (x_length * cell_size)) / 2.0;
        let y_just = (height - (y_length * cell_size)) / 2.0;

        Canvas {
            ctx,
            x_length,
            y_length,
            cell_size,
            x_just,
            y_just,
            alive_color: JsValue::from_str("#111111"),
            dead_color: JsValue::from_str("#DDDDDD"),
        }
    }

    fn draw_cell(&self, x: f64, y: f64, state: bool) {
        let padding = 2.0;
        if state {
            self.ctx.set_fill_style(&self.alive_color);
        } else {
            self.ctx.set_fill_style(&self.dead_color);
        }
        self.ctx.fill_rect(
            x * self.cell_size + padding + self.x_just,
            y * self.cell_size + padding + self.y_just,
            self.cell_size - 2.0 * padding,
            self.cell_size - 2.0 * padding,
        );
    }

    pub fn draw(&self, state_vec: Vec<Vec<bool>>) {
        for (y_ind, row) in state_vec.iter().enumerate() {
            for (x_ind, state) in row.iter().enumerate() {
                self.draw_cell(x_ind as f64, y_ind as f64, state.clone());
            }
        }
    }
}
