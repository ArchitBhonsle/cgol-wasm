use crate::utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

pub struct Canvas {
    pub ctx: web_sys::CanvasRenderingContext2d,
    pub x_length: u32,
    pub y_length: u32,
    pub cell_size: u32,
    alive_color: JsValue,
    dead_color: JsValue,
}

impl Canvas {
    pub fn new(canvas_id: &str, cell_size: u32) -> Canvas {
        let canvas: web_sys::HtmlCanvasElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let height: u32 = canvas.height();
        let width: u32 = canvas.width();

        let x_length = height / cell_size;
        let y_length = width / cell_size;

        Canvas {
            ctx,
            x_length,
            y_length,
            cell_size,
            alive_color: JsValue::from_str("#ffffff"),
            dead_color: JsValue::from_str("#111111"),
        }
    }

    pub fn draw_cell(&self, x: u32, y: u32, state: bool) {
        if state {
            self.ctx.set_fill_style(&self.alive_color);
        } else {
            self.ctx.set_fill_style(&self.dead_color);
        }
        self.ctx.fill_rect(
            (x * self.cell_size) as f64,
            (y * self.cell_size) as f64,
            self.cell_size as f64,
            self.cell_size as f64,
        );
    }

    pub fn draw(&self, state_vec: Vec<Vec<bool>>) {
        for (x_ind, row) in state_vec.iter().enumerate() {
            for (y_ind, state) in row.iter().enumerate() {
                self.draw_cell(x_ind as u32, y_ind as u32, state.clone());
            }
        }
    }
}
