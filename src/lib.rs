#![recursion_limit = "1024"]

#[macro_use]
extern crate validator_derive;
extern crate validator;

mod fetching;
mod login;
mod routing;
mod register;
mod index;
mod dashboard;
mod graphql;

use login::Model as Login;
use register::Model as Register;
use index::Model as Index;
use dashboard::Model as Dashboard;
use routing::AppRoute;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::switch::Permissive;
use yew_router::{prelude::Router, route::Route};

#[derive(Serialize, Deserialize)]
pub struct CurrentUser {
    id: i32,
    email: String,
    company: String,
    created_at: String
}

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <div class="container">
                <Router <AppRoute>
                  render = Router::render(|switch: AppRoute| {
                      match switch {
                        AppRoute::Login => html!{ <Login />},
                        AppRoute::Register => html! { <Register /> },
                        AppRoute::Index => html!{ <Index /> },
                        AppRoute::Dashboard => html!{ <Dashboard /> },
                        AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                        AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                      }
                  })
                  redirect = Router::redirect(|route: Route| {
                      AppRoute::PageNotFound(Permissive(Some(route.route)))
                  })
                />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}
