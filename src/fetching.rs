use serde::{Deserialize, Serialize};
use std::fmt::{Error as FmtError, Formatter};
use std::future::Future;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Headers, HtmlDocument, Request, RequestCredentials, RequestInit, RequestMode, Response,
};
use yew::prelude::{Component, ComponentLink};
use yew::services::ConsoleService;

use crate::routing::{AppRoute, Redirecter};

const TOKEN_KEY: &str = "mystore.key";

pub struct FetchResponse<T> {
    pub headers: Headers,
    pub data: T,
}

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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        std::fmt::Debug::fmt(&self.err, f)
    }
}
impl std::error::Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        FetchError { err: value }
    }
}

impl From<&str> for FetchError {
    fn from(value: &str) -> Self {
        FetchError {
            err: JsValue::from_str(value),
        }
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

pub async fn send_request<'a, T, R>(
    url: &'a str,
    data: Option<&T>,
    method: &str,
) -> Result<FetchResponse<JsValue>, FetchError>
where
    T: Serialize,
    R: Serialize + for<'b> Deserialize<'b>,
{
    let mut opts = RequestInit::new();
    opts.method(method);
    opts.mode(RequestMode::Cors);
    opts.credentials(RequestCredentials::SameOrigin);
    if let Some(maybe_data) = data {
        if let Ok(data_str) = serde_json::to_string(&maybe_data) {
            opts.body(Some(&JsValue::from_str(&data_str)));
        }
    }

    let base_url = "http://localhost:8088";

    let request = Request::new_with_str_and_init(&format!("{}{}", base_url, url), &opts)?;
    request.headers().set("Content-Type", "application/json")?;
    if let Ok(token) = get_token() {
        request.headers().set("x-csrf-token", &token)?;
    }

    let window =
        web_sys::window().ok_or_else(|| JsValue::from_str("Could not get a window object"))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    let headers = resp.headers();
    if let Err(_) = validate_token(&headers) {
        let mut redirecter = Redirecter::new();
        redirecter.redirect(AppRoute::Login);
        ConsoleService::new().log("Redirecting!")
    }

    let json = JsFuture::from(resp.json()?).await?;
    match json.into_serde::<R>() {
        Ok(value) => JsValue::from_serde(&value)
            .map(|data| FetchResponse { headers, data })
            .map_err(|error| FetchError {
                err: JsValue::from_str(&error.to_string()),
            }),
        Err(error) => Err(FetchError {
            err: JsValue::from(error.to_string()),
        }),
    }
}

pub fn save_token(headers: Headers) -> Result<bool, FetchError> {
    let token = headers
        .get("x-csrf-token")?
        .ok_or_else(|| JsValue::from_str("Could not get token from Header"))?;
    store_token(token)
}

pub fn delete_token() -> Result<bool, FetchError> {
    let window =
        web_sys::window().ok_or_else(|| JsValue::from_str("Could not get a window object"))?;
    let maybe_storage = window.local_storage()?;
    if let Some(storage) = maybe_storage {
        storage.remove_item(TOKEN_KEY)?;
        Ok(true)
    } else {
        Err(FetchError {
            err: JsValue::from_str("Could not delete Token!"),
        })
    }
}

pub fn set_cookie(headers: Headers) -> Result<bool, FetchError> {
    let cookie = headers
        .get("set-cookie")?
        .ok_or_else(|| JsValue::from_str("Could not get token from Header"))?;
    html_document()?.set_cookie(&cookie)?;
    Ok(true)
}

fn get_token() -> Result<String, FetchError> {
    let window =
        web_sys::window().ok_or_else(|| JsValue::from_str("Could not get a window object"))?;
    let maybe_storage = window.local_storage()?;
    if let Some(storage) = maybe_storage {
        Ok(storage
            .get_item(TOKEN_KEY)?
            .ok_or_else(|| JsValue::from_str("Could not get token"))?)
    } else {
        Err(FetchError {
            err: JsValue::from_str("Could not get Token!"),
        })
    }
}

fn validate_token(headers: &Headers) -> Result<bool, FetchError> {
    let local_token = get_token()?;
    let result_token = headers
        .get("x-csrf-token")
        .map_err(|err| FetchError { err })?;
    if let Some(header_token) = result_token {
        if local_token != header_token {
            return Err(FetchError {
                err: JsValue::from_str("Wrong token!"),
            });
        }
    }
    Ok(true)
}

fn store_token(token: String) -> Result<bool, FetchError> {
    let window =
        web_sys::window().ok_or_else(|| JsValue::from_str("Could not get a window object"))?;
    let maybe_storage = window.local_storage()?;
    if let Some(storage) = maybe_storage {
        storage.set_item(TOKEN_KEY, &token)?;
    }
    Ok(true)
}

fn html_document() -> Result<HtmlDocument, FetchError> {
    let window =
        web_sys::window().ok_or_else(|| JsValue::from_str("Could not get a window object"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("Could not get a document object"))?;
    Ok(wasm_bindgen::JsValue::from(document).unchecked_into::<web_sys::HtmlDocument>())
}
