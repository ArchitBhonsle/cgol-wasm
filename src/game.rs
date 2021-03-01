use crate::canvas;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Game {
    canvas: Rc<canvas::Canvas>,
    button: Rc<web_sys::HtmlButtonElement>,
    slider: Rc<web_sys::HtmlInputElement>,
    state_vec: Rc<RefCell<Vec<Vec<bool>>>>,
    paused: Rc<RefCell<bool>>,
    pub fps: Rc<RefCell<f64>>,
}

impl Game {
    pub fn new(
        canvas_id: &str,
        button_id: &str,
        slider_id: &str,
        cell_size: u32,
        padding: u32,
    ) -> Game {
        let canvas = canvas::Canvas::new(canvas_id, padding as f64, cell_size as f64);

        let state_vec = (0..(canvas.y_length as usize))
            .map(|_| {
                (0..(canvas.x_length as usize))
                    .map(|_| random_boolean())
                    .collect()
            })
            .collect();
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

        let slider = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(slider_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap();
        let slider = Rc::new(slider);

        let fps = Rc::new(RefCell::new(60.0));

        Game {
            canvas,
            button,
            slider,
            state_vec,
            paused,
            fps,
        }
    }

    fn attach_canvas_onclick(&self) {
        let paused = self.paused.clone();
        let state_vec_cell = self.state_vec.clone();
        let canvas = self.canvas.clone();
        let (x_just, y_just) = canvas.get_justs();
        let (x_dim, y_dim) = canvas.get_canvas_dims();
        let cell_size = canvas.cell_size;

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x_offset = event.offset_x() as f64;
            let y_offset = event.offset_y() as f64;
            let paused_state = *(*paused).borrow();

            if paused_state
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
        let button_to_closure = self.button.clone();
        let slider_to_closure = self.slider.clone();
        let paused_cell = self.paused.clone();
        let fps_cell = self.fps.clone();

        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            let mut paused = paused_cell.borrow_mut();
            let mut fps = fps_cell.borrow_mut();
            let slider_value: f64 = slider_to_closure.value_as_number();
            if *paused {
                button_to_closure.set_inner_text("⏸");
                *paused = false;
                *fps = slider_value;
            } else {
                button_to_closure.set_inner_text("▶");
                *paused = true;
                *fps = 60.0;
            }
        }) as Box<dyn FnMut(_)>);

        button.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    fn attach_slider_watch(&self) {
        let slider = self.slider.clone();
        let slider_to_closure = self.slider.clone();
        let fps_cell = self.fps.clone();

        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            let mut fps = fps_cell.borrow_mut();
            let slider_value: f64 = slider_to_closure.value_as_number();
            *fps = slider_value;
        }) as Box<dyn FnMut(_)>);

        slider.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    pub fn attach_listeners(&self) {
        self.attach_canvas_onclick();
        self.attach_button_onclick();
        self.attach_slider_watch();
    }

    fn draw(&self) {
        let state_vec = (*self.state_vec).borrow().to_vec();
        self.canvas.draw(state_vec);
    }

    pub fn tick(&self) {
        let paused_cell = self.paused.clone();
        let paused_state = *(*paused_cell).borrow();

        if !paused_state {
            let (x_length, y_length) = self.canvas.get_lengths();
            let current_state_vec = (*self.state_vec).borrow().to_vec();
            let mut next_state_vec = vec![vec![false; x_length as usize]; y_length as usize];
            for (y_ind, row) in current_state_vec.iter().enumerate() {
                for (x_ind, state) in row.iter().enumerate() {
                    let neighbours =
                        count_neighbours(&current_state_vec, x_ind, y_ind, x_length, y_length);

                    if *state && (neighbours == 2 || neighbours == 3) {
                        next_state_vec[y_ind][x_ind] = true;
                    } else if !*state && neighbours == 3 {
                        next_state_vec[y_ind][x_ind] = true;
                    }
                }
            }

            self.state_vec.replace(next_state_vec);
        }

        self.draw()
    }
}

fn random_boolean() -> bool {
    js_sys::Math::random() < 0.5
}

fn count_neighbours(
    state_vec: &Vec<Vec<bool>>,
    x: usize,
    y: usize,
    x_length: f64,
    y_length: f64,
) -> u32 {
    let mut neighbours: u32 = 0;
    let x_diff: [i32; 3] = [1, 0, -1];
    let y_diff: [i32; 3] = [1, 0, -1];
    let (x, y) = (x as i32, y as i32);
    let (x_length, y_length) = (x_length as i32, y_length as i32);

    for x_n in x_diff.iter() {
        for y_n in y_diff.iter() {
            if *x_n == 0 && *y_n == 0 {
                continue;
            };

            if x + x_n < 0 || x + x_n > x_length - 1 || y + y_n < 0 || y + y_n > y_length - 1 {
                continue;
            }

            if state_vec[(y + y_n) as usize][(x + x_n) as usize] {
                neighbours += 1;
            }
        }
    }

    neighbours
}
