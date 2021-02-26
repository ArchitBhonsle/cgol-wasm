use crate::canvas;
// use crate::utils;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

pub struct Game {
    canvas: Rc<canvas::Canvas>,
    button: Rc<web_sys::HtmlButtonElement>,
    state_vec: Rc<RefCell<Vec<Vec<bool>>>>,
    paused: Rc<RefCell<bool>>,
}

impl Game {
    pub fn new(
        canvas_id: &str,
        button_id: &str,
        cell_size: u32,
        padding: u32,
        alive_color: &str,
        dead_color: &str,
    ) -> Game {
        let config = canvas::CanvasConfig {
            cell_size: cell_size as f64,
            padding: padding as f64,
            alive_color: JsValue::from_str(alive_color),
            dead_color: JsValue::from_str(dead_color),
        };
        let canvas = canvas::Canvas::new(canvas_id, config);
        let state_vec = vec![vec![false; canvas.x_length as usize]; canvas.y_length as usize];
        let state_vec = Rc::new(RefCell::new(state_vec));
        let paused = Rc::new(RefCell::new(true));

        let button = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(button_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlButtonElement>()
            .unwrap();
        let button = Rc::new(button);

        Game {
            canvas,
            button,
            state_vec,
            paused,
        }
    }

    fn attach_canvas_onclick(&self) {
        let paused = self.paused.clone();
        let state_vec_cell = self.state_vec.clone();
        let canvas = self.canvas.clone();
        let (x_just, y_just) = canvas.get_justs();
        let (x_dim, y_dim) = canvas.get_canvas_dims();
        let cell_size = canvas.config.cell_size;

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x_offset = event.offset_x() as f64;
            let y_offset = event.offset_y() as f64;

            if !paused.borrow().clone()
                && x_offset > x_just
                && x_offset < x_dim - x_just
                && y_offset > y_just
                && y_offset < y_dim - y_just
            {
                let mut state_vec = state_vec_cell.borrow_mut();
                let x = ((x_offset - x_just) / cell_size).floor() as usize;
                let y = ((y_offset - y_just) / cell_size).floor() as usize;

                state_vec[y][x] = !state_vec[y][x];
            }
        }) as Box<dyn FnMut(_)>);

        canvas
            .canvas
            .set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    fn attach_button_onclick(&self) {
        let button = self.button.clone();
        let button_to_canvas = self.button.clone();
        let paused_cell = self.paused.clone();

        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            let mut paused = paused_cell.borrow_mut();
            button_to_canvas.set_inner_text(if *paused { "pause" } else { "play" });
            *paused = !(*paused);
        }) as Box<dyn FnMut(_)>);

        button.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    pub fn attach_onclicks(&self) {
        self.attach_canvas_onclick();
        self.attach_button_onclick();
    }
    pub fn tick(&self) {
        self.canvas.draw(self.state_vec.borrow().to_vec());
    }
}
