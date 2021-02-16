use std::collections::HashMap;
use crate::canvas::Canvas;
use wasm_bindgen::closure::Closure;
use web_sys::KeyboardEvent;
use wasm_bindgen::{JsCast, JsValue};
use std::rc::Rc;
use std::cell::RefCell;

type KeyMap = HashMap<String, bool>;
type KeyCallback = Closure<dyn FnMut(KeyboardEvent)>;

pub struct Input {
    rc_key_map: Rc<RefCell<HashMap<String, bool>>>
}

impl Input {
    pub fn new() -> Input {
        let rc_key_map: Rc<RefCell<KeyMap>> = Rc::new(RefCell::new([
            (String::from("ArrowUp"), false),
            (String::from("ArrowDown"), false),
            (String::from("ArrowLeft"), false),
            (String::from("ArrowRight"), false)
        ].iter().cloned().collect()));

        Input { rc_key_map }
    }

    pub fn connect_to_canvas(&self, canvas: &Canvas) -> Result<(), JsValue> {
        let add_callback = |option, callback: KeyCallback| -> Result<(), JsValue> {
            canvas.add_event_listener(option, callback.as_ref().unchecked_ref())?;
            callback.forget(); // TODO weak ref flag
            Ok(())
        };

        add_callback(
            "keydown",
            make_keydown_callback(Rc::clone(&self.rc_key_map))
        )?;

        add_callback(
            "keyup",
            make_keyup_callback(Rc::clone(&self.rc_key_map))
        )?;

        Ok(())
    }

    pub fn is_pressed(&self, key_name: &str) -> bool {
        self.rc_key_map
            .borrow()
            .get(key_name)
            .expect("wrong key name")
            .clone()
    }
}

fn make_keydown_callback(key_map: Rc<RefCell<KeyMap>>) -> KeyCallback {
    Closure::wrap(Box::new(move |event: KeyboardEvent| {
        event.prevent_default();

        if let Some(value) = key_map.borrow_mut().get_mut(&event.key()) {
            *value = true
        }
    }) as Box<dyn FnMut(_)>)
}

fn make_keyup_callback(key_map: Rc<RefCell<KeyMap>>) -> KeyCallback {
    Closure::wrap(Box::new(move |event: KeyboardEvent| {
        if let Some(value) = key_map.borrow_mut().get_mut(&event.key()) {
            *value = false
        }
    }) as Box<dyn FnMut(_)>)
}