mod canvas;
mod input;
mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::HtmlCanvasElement;

use crate::canvas::Canvas;
use crate::input::Input;
use crate::utils::{Position, Size};

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let header = document.create_element("h1")?;
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    header.set_inner_html("wasm-snake");

    body.append_child(&header)?;
    body.append_child(&canvas)?;

    let mut canvas = Canvas::from(canvas)?;
    canvas.set_size(Size { w: 500, h: 500 });

    let input = Input::new();
    input.connect_to_canvas(&canvas)?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut pos: Position = Position { x: 0, y: 0 };

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if input.is_pressed("ArrowUp") {
            pos.y -= 1;
        }

        if input.is_pressed("ArrowDown") {
            pos.y += 1;
        }

        if input.is_pressed("ArrowLeft") {
            pos.x -= 1;
        }

        if input.is_pressed("ArrowRight") {
            pos.x += 1;
        }

        canvas.draw_rect(
            Position { x: 0, y: 0 },
            Size { w: canvas.width(), h: canvas.height() },
            "white",
        );

        canvas.draw_rect(
            Position { x: pos.x, y: pos.y },
            Size { w: 10, h: 10 },
            "black",
        );

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}