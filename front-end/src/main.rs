#![recursion_limit = "512"]

mod app;
mod components;
mod fetch;
mod markdown;
mod utils;
mod views;

use wasm_bindgen::prelude::*;
// pub use yew::services::console::ConsoleService;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
// #[wasm_bindgen]
//#[wasm_bindgen(start)]
// pub fn run_app() -> Result<(), JsValue> {
// pub fn run_app()  {
// #[function_component(App)]
// fn app() -> Html {
// pub fn run_app(root: web_sys::Element)  {
fn main() {
    utils::set_panic_hook();
    // web_logger::init();
    // yew::start_app::<app::App>();
    // yew::Renderer::<app::App>::with_root(root).render();
    yew::Renderer::<app::App>::new().render();
}
