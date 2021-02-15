use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use std::f64;
use std::rc::Rc;
use std::cell::RefCell;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();

    let document = window
        .document()
        .unwrap();

    let body = document.body().unwrap();

    let header = document.create_element("p")?;
    header.set_inner_html("Hello from Rust!");

    body.append_child(&header)?;

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    canvas.set_width(300);
    canvas.set_height(300);

    body.append_child(&canvas)?;

    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    context.begin_path();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let (mut x, mut y) = (10.0, 10.0);

    enum Direction {
        Left,
        Right
    }
    let mut direction = Direction::Right;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.set_fill_style(&JsValue::from_str("white"));
        context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        context.set_fill_style(&JsValue::from_str("black"));
        context.fill_rect(x, y, 10.0, 10.0);

        if x > canvas.width() as f64 - 10.0 {
            direction = Direction::Left;
        } else if x < 0.0 {
            direction = Direction::Right;
        }

        match direction {
            Direction::Right => {
                x += 1.0;
                y += 1.0;
            },
            Direction::Left => {
                x -= 1.0;
                y -= 1.0;
            }
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}