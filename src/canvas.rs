use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};

pub struct CanvasConfig {
    pub cell_size: f64,
    pub padding: f64,
    pub alive_color: JsValue,
    pub dead_color: JsValue,
}

pub struct Canvas {
    pub canvas: web_sys::HtmlCanvasElement,
    pub ctx: web_sys::CanvasRenderingContext2d,
    pub x_length: f64,
    pub y_length: f64,
    pub x_just: f64,
    pub y_just: f64,
    pub config: CanvasConfig,
}

impl Canvas {
    pub fn new(canvas_id: &str, config: CanvasConfig) -> Rc<Canvas> {
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

        let cell_size = config.cell_size;

        let x_length = (width / cell_size).floor();
        let y_length = (height / cell_size).floor();

        let x_just = (width - (x_length * cell_size)) / 2.0;
        let y_just = (height - (y_length * cell_size)) / 2.0;

        Rc::new(Canvas {
            canvas,
            ctx,
            x_length,
            y_length,
            x_just,
            y_just,
            config,
        })
    }

    // pub fn get_lengths(&self) -> (f64, f64) {
    //     (self.x_length, self.y_length)
    // }
    pub fn get_justs(&self) -> (f64, f64) {
        (self.x_just, self.y_just)
    }
    pub fn get_canvas_dims(&self) -> (f64, f64) {
        (self.canvas.width() as f64, self.canvas.height() as f64)
    }

    fn draw_cell(&self, x: f64, y: f64, state: bool) {
        let cell_size = self.config.cell_size;
        let padding = self.config.padding;

        if state {
            self.ctx.set_fill_style(&self.config.alive_color);
        } else {
            self.ctx.set_fill_style(&self.config.dead_color);
        }
        self.ctx.fill_rect(
            x * cell_size + padding + self.x_just,
            y * cell_size + padding + self.y_just,
            cell_size - 2.0 * padding,
            cell_size - 2.0 * padding,
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
