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
use crate::utils::{Position, Size, Direction};

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn handle_input(input: &Input, direction: &mut Direction) {
    if input.is_pressed("ArrowUp") {
        *direction = Direction::Up;
    } else if input.is_pressed("ArrowDown") {
        *direction = Direction::Down;
    } else if input.is_pressed("ArrowLeft") {
        *direction = Direction::Left;
    } else if input.is_pressed("ArrowRight") {
        *direction = Direction::Right;
    }
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
    let performance = window.performance().unwrap();

    header.set_inner_html("wasm-snake");
    body.append_child(&header)?;
    body.append_child(&canvas)?;

    let canvas_size = Size { w: 500, h: 500 };
    let mut canvas = Canvas::from(canvas)?;
    canvas.set_size(canvas_size);

    let input = Input::new();
    input.connect_to_canvas(&canvas)?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    const GRID_WIDTH: i32 = 20;
    const GRID_HEIGHT: i32 = 20;
    let cell_size = Size {
        w: canvas_size.w / GRID_WIDTH,
        h: canvas_size.h / GRID_HEIGHT,
    };

    // TODO remove elements except one
    let mut snake = vec![
        Position { x: 4, y: 0 },
        Position { x: 3, y: 0 },
        Position { x: 2, y: 0 },
        Position { x: 1, y: 0 },
        Position { x: 0, y: 0 }
    ];

    let mut direction = Direction::Right;
    let mut last_direction = Direction::Right;

    let mut last = performance.now();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
         handle_input(&input, &mut direction);

        if (direction == Direction::Up && last_direction == Direction::Down)
            || (direction == Direction::Left && last_direction == Direction::Right)
            || (direction == Direction::Right && last_direction == Direction::Left)
            || (direction == Direction::Down && last_direction == Direction::Up)
        {
            direction = last_direction;
        }

        let now = performance.now();
        if now - last > 150.0 {
            last = now;
            last_direction = direction;

            let shift = match direction {
                Direction::Up => Position { x: 0, y: -1 },
                Direction::Down => Position { x: 0, y: 1 },
                Direction::Left => Position { x: -1, y: 0 },
                Direction::Right => Position { x: 1, y: 0 }
            };

            let shifted = snake[0] + shift;

            if !(shifted.x < 0
                || shifted.x >= GRID_WIDTH
                || shifted.y < 0
                || shifted.y >= GRID_HEIGHT
                || snake.iter().skip(1).find(|elem| **elem == shifted).is_some())
            {
                for i in (1..snake.len()).rev() {
                    snake[i] = snake[i - 1];
                }

                snake[0] = shifted;
            }
        }

        let pos: Vec<Position> = snake.iter().map(|elem| {
            Position {
                x: elem.x * cell_size.w,
                y: elem.y * cell_size.h,
            }
        }).collect();

        canvas.draw_rect(
            Position { x: 0, y: 0 },
            Size { w: canvas.width() as i32, h: canvas.height() as i32 },
            "white",
        );

        for elem in pos {
            canvas.draw_rect(
                elem,
                Size { w: cell_size.w, h: cell_size.h },
                "black",
            );
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}