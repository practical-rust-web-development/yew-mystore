use serde::Serialize;
use std::fmt::{Error, Formatter};
use std::future::Future;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::prelude::{Component, ComponentLink};

#[derive(Clone)]
pub enum FetchState<T> {
    Fetching,
    Success(T),
    Failed(FetchError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    pub err: JsValue,
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

pub fn send_future<COMP: Component, F>(link: ComponentLink<COMP>, future: F)
where
    F: Future<Output = COMP::Message> + 'static,
{
    spawn_local(async move {
        link.send_message(future.await);
    });
}

pub async fn send_request<T>(url: &str, data: &T, method: &str) -> Result<JsValue, FetchError>
where
    T: Serialize,
{
    let mut opts = RequestInit::new();
    opts.method(method);
    opts.mode(RequestMode::Cors);
    if let Ok(data_str) = serde_json::to_string(&data) {
        opts.body(Some(&JsValue::from_str(&data_str)));
    }

    let request = Request::new_with_str_and_init(url, &opts)?;

    request.headers().set("Content-Type", "application/json")?;

    let window = web_sys::window()
        .ok_or_else(|| JsValue::from_str("Could not get a window object"))
        .unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    Ok(JsFuture::from(resp.json()?).await?)
}
