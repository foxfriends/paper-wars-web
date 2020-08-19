use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
mod routes;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<routes::Routes>::new().mount_to_body();
}
