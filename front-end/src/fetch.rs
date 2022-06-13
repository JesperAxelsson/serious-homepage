#![allow(dead_code)]
// use serde::{Deserialize};
use serde::de::DeserializeOwned;
use yew::html::Scope;
// use serde_json::Result;
use std::fmt::{Error, Formatter};
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};

use yew::prelude::*;

/// This method processes a Future that returns a message and sends it back to the component's
/// loop.
///
/// # Panics
/// If the future panics, then the promise will not resolve, and will leak.
// #[cfg(all(target_arch = "wasm32", not(target_os = "wasi"), not(cargo_web)))]
#[allow(unused_must_use)]
pub fn send_future<COMP: Component, F>(link: &Scope<COMP>, future: F)
where
    F: Future<Output = COMP::Message> + 'static,
{
    use wasm_bindgen_futures::future_to_promise;

    let link = link.clone();
    let js_future = async move {
        link.send_message(future.await);
        Ok(JsValue::NULL)
    };

    future_to_promise(js_future);
}

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        std::fmt::Debug::fmt(&self.err, f)
    }
}
impl std::error::Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        FetchError { err: value }
    }
}

/// The possible states a fetch request can be in.
#[derive(Debug)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

/// Fetches stuff
///
/// Consult the following for an example of the fetch api by the team behind web_sys:
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
pub async fn fetch_url(url: &str) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window: Window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}

/// Fetches stuff
///
/// Consult the following for an example of the fetch api by the team behind web_sys:
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
pub async fn fetch_url2<D>(url: &str) -> Result<D, FetchError>
where
    D: DeserializeOwned,
{
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window: Window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();

    let js_text: JsValue = JsFuture::from(resp.text()?).await?;

    let text: String = js_text.as_string().unwrap().to_string();

    // let txt: &str = text.clone().as_str();
    let data: D = serde_json::from_str::<D>(&text).unwrap();

    // let ss = text.as_string().unwrap();
    // let data: D =  serde_json::from_str(&ss).unwrap();

    //let d: D = serde
    Ok(data)
}
