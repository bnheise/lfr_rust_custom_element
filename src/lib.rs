mod component;
use component::Model;
use custom_elements::{inject_stylesheet, CustomElement};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlElement;
use yew::AppHandle;

const ELEMENT_ID: &str = "liferay-rust-app";

struct ComponentWrapper {
    handle: Option<AppHandle<Model>>,
}

impl ComponentWrapper {
    fn new() -> Self {
        Self { handle: None }
    }
}

impl Default for ComponentWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomElement for ComponentWrapper {
    fn inject_children(&mut self, this: &HtmlElement) {
        let app = yew::start_app_in_element::<Model>(this.to_owned().into());
        self.handle = Some(app);
        inject_stylesheet(&this, "http://localhost:9000/main.css");
    }

    fn shadow() -> bool {
        false
    }
}

#[wasm_bindgen]
pub fn run() {
    let window = web_sys::window().expect("window to be available.");
    let document = window.document().expect("document to be available");
    let element = document.get_element_by_id(ELEMENT_ID);
    if element.is_none() {
        ComponentWrapper::define(ELEMENT_ID);
    }
}
