use graphql_client::{GraphQLQuery, QueryBody};
use wasm_bindgen::prelude::JsValue;
use yew::prelude::{html, Component, ComponentLink, ShouldRender};
use yew::services::ConsoleService;
use yew::virtual_dom::VNode;

use crate::fetching::{delete_token, send_future, send_request, FetchState};
use crate::graphql;
use crate::routing::{AppRoute, Redirecter};

pub struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {
    OnLoad,
    Loaded(FetchState<JsValue>),
    Logout,
    LoggedOut(FetchState<JsValue>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::OnLoad);
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnLoad => {
                let future = async move {
                    match graphql::fetch_graphql::<QueryBody<graphql::dashboard::Variables>, String>(
                        graphql::Dashboard::build_query(graphql::dashboard::Variables),
                    )
                    .await
                    {
                        Ok(response) => Msg::Loaded(FetchState::Success(response.data)),
                        Err(error) => Msg::Loaded(FetchState::Failed(error)),
                    }
                };
                send_future(self.link.clone(), future);
                true
            }
            Msg::Logout => {
                let future = async move {
                    match send_request::<Option<String>, String>("/logout", None, "DELETE").await {
                        Ok(response) => Msg::LoggedOut(FetchState::Success(response.data)),
                        Err(error) => Msg::LoggedOut(FetchState::Failed(error)),
                    }
                };
                if let Err(error) = delete_token() {
                    ConsoleService::new().log(&format!("Error: {}", &error));
                }
                send_future(self.link.clone(), future);
                true
            }
            Msg::LoggedOut(fetch_state) => {
                match fetch_state {
                    FetchState::Success(_) => {
                        let mut redirecter = Redirecter::new();
                        redirecter.redirect(AppRoute::Login);
                        ConsoleService::new().log("Success")
                    }
                    FetchState::Failed(error) => {
                        ConsoleService::new().log(&format!("Error: {}", &error.to_string()))
                    }
                    FetchState::Fetching => ConsoleService::new().log("Fetching"),
                };
                true
            }
            Msg::Loaded(fetch_state) => {
                match fetch_state {
                    FetchState::Success(_) => ConsoleService::new().log("Success"),
                    FetchState::Failed(error) => {
                        ConsoleService::new().log(&format!("Error: {}", &error.to_string()))
                    }
                    FetchState::Fetching => ConsoleService::new().log("Fetching"),
                };
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <div>
                <nav class="navbar navbar-expand-lg navbar-light bg-light">
                    <a class="navbar-brand" href="#">
                        <i class="fas fa-store"></i>
                        {"My Store"}
                    </a>

                    <button onclick=self.link.callback(|_| Msg::Logout)
                            class="btn btn-info my-4">{ "Logout" }</button>
                </nav>
                <h1> { "Dashboard" } </h1>
            </div>
        }
    }
}
