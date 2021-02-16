use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use js_sys::Function;
use wasm_bindgen::{JsCast, JsValue};
use crate::utils::{Size, Position};

pub struct Canvas {
    html_canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn from(html_canvas: HtmlCanvasElement) -> Result<Canvas, JsValue> {
        let context: CanvasRenderingContext2d = html_canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        context.begin_path();
        html_canvas.set_tab_index(1);

        Ok(Canvas {
            html_canvas,
            context
        })
    }

    pub fn set_size(&mut self, size: Size) {
        self.html_canvas.set_width(size.w);
        self.html_canvas.set_height(size.h);
    }

    pub fn width(&self) -> u32 {
        self.html_canvas.width()
    }

    pub fn height(&self) -> u32 {
        self.html_canvas.height()
    }

    pub fn draw_rect(&self, pos: Position, size: Size, color: &str) {
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context.fill_rect(
            pos.x as f64,
            pos.y as f64,
            size.w as f64,
            size.h as f64
        );
    }

    pub fn add_event_listener(&self, option: &str, callback: &Function ) -> Result<(), JsValue> {
        self.html_canvas.add_event_listener_with_callback(option, callback)
    }
}