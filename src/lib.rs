use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let header = document.create_element("p")?;
    header.set_inner_html("Hello from Rust!");

    body.append_child(&header)?;

    Ok(())
}