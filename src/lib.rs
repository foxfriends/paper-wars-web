use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod routes;
mod components;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<routes::Routes>::new().mount_to_body();
}
