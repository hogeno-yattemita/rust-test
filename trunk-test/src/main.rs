use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let el = document.create_element("button").unwrap();
    let mut cnt = 0;
    let el = Rc::new(el);
    let el2 = el.clone();
    el.set_text_content(Some(&format!("Clicked {} times", cnt)));
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        cnt += 1;
        el.set_text_content(Some(&format!("Clicked {} times", cnt)));
    }) as Box<dyn FnMut(_)>);
    el2.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();

    body.append_child(&el2).unwrap();
    web_sys::console::log_1(&"Hello, world!".into());
}
