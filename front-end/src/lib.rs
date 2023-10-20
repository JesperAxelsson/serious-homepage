#![recursion_limit = "512"]

mod app;
mod utils;
mod views;
mod fetch;
mod components;
mod markdown;

use wasm_bindgen::prelude::*;
// pub use yew::services::console::ConsoleService;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
//#[wasm_bindgen(start)]
// pub fn run_app() -> Result<(), JsValue> {
// pub fn run_app()  {
pub fn run_app(root: web_sys::Element)  {
    utils::set_panic_hook();
    // web_logger::init();
    // yew::start_app::<app::App>();
    yew::Renderer::<app::App>::with_root(root).render();
}
