use crate::fetching::{send_future, send_request, FetchError, FetchState};
use crate::routing::AppRoute;
use crate::CurrentUser;
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use wasm_bindgen::prelude::JsValue;
use yew::agent::{Bridge, Bridged};
use yew::prelude::{html, Component, ComponentLink, InputData, ShouldRender};
use yew::services::ConsoleService;
use yew::virtual_dom::VNode;
use yew_router::{agent::RouteAgent, agent::RouteRequest, prelude::RouterAnchor, route::Route};

pub struct Model {
    link: ComponentLink<Self>,
    register_user: RegisterUser,
    router: Box<dyn Bridge<RouteAgent>>,
}

#[derive(Serialize, Validate, Deserialize, Clone)]
pub struct RegisterUser {
    #[validate(email)]
    #[validate(length(min = 1))]
    email: String,
    company: String,
    #[validate(length(min = 8))]
    password: String,
    #[validate(length(min = 8))]
    password_confirmation: String,
}

pub enum FormField {
    Email,
    Company,
    Password,
    PasswordConfirmation,
}

pub enum Msg {
    Register,
    Registered(FetchState<JsValue>),
    UpdateForm(String, FormField),
    NoOp,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::NoOp);
        let router = RouteAgent::bridge(callback);

        Self {
            link,
            register_user: RegisterUser {
                email: "".to_string(),
                company: "".to_string(),
                password: "".to_string(),
                password_confirmation: "".to_string(),
            },
            router,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let register_user = self.register_user.clone();
        match msg {
            Msg::Register => {
                let future = async move {
                    match register_user.validate() {
                        Ok(_) => match send_request::<RegisterUser, CurrentUser>("/register", &register_user, "POST").await {
                            Ok(user) => Msg::Registered(FetchState::Success(user)),
                            Err(error) => Msg::Registered(FetchState::Failed(error)),
                        },
                        Err(error) => Msg::Registered(FetchState::Failed(FetchError {
                            err: JsValue::from(error.to_string()),
                        })),
                    }
                };
                send_future(self.link.clone(), future);
                true
            }
            Msg::Registered(fetch_state) => {
                match fetch_state {
                    FetchState::Success(_) => {
                        self.router
                            .send(RouteRequest::ReplaceRoute(Route::from(AppRoute::Index)));
                        ConsoleService::new().log("Success");
                    }
                    FetchState::Failed(error) => ConsoleService::new().log(&error.to_string()),
                    FetchState::Fetching => ConsoleService::new().log("Fetching"),
                };
                true
            }
            Msg::UpdateForm(value, form_field) => {
                match form_field {
                    FormField::Email => self.register_user.email = value,
                    FormField::Company => self.register_user.company = value,
                    FormField::Password => self.register_user.password = value,
                    FormField::PasswordConfirmation => {
                        self.register_user.password_confirmation = value
                    }
                };
                true
            }
            Msg::NoOp => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <div class="col-lg-6 text-center border mx-auto p-5">
                <p class="h4 mb-4"> { "Sign Up" }</p>
                <div class="form-row mb-4">
                    <input name="email"
                        type="text"
                        placeholder="email"
                        class="form-control"
                        oninput=self.link.callback(|e: InputData|
                            Msg::UpdateForm(e.value, FormField::Email)
                        )/>
                </div>
                <div class="form-row mb-4">
                    <input name="company"
                        type="text"
                        placeholder="company"
                        class="form-control"
                        oninput=self.link.callback(|e: InputData|
                            Msg::UpdateForm(e.value, FormField::Company)
                        )/>
                </div>
                <div class="form-row mb-4">
                    <input name="password"
                        type="password"
                        placeholder="password"
                        class="form-control"
                        oninput=self.link.callback(|e: InputData|
                            Msg::UpdateForm(e.value, FormField::Password)
                        )/>
                </div>
                <div class="form-row mb-4">
                    <input name="password_confirmation"
                        type="password"
                        placeholder="password confirmation"
                        class="form-control"
                        oninput=self.link.callback(|e: InputData|
                            Msg::UpdateForm(e.value, FormField::PasswordConfirmation)
                        )/>
                </div>
                <button onclick=self.link.callback(|_| Msg::Register)
                        class="btn btn-info my-4 btn-block">{ "Sign Up" }</button>

                <hr />
                <RouterAnchor<AppRoute> route=AppRoute::Index> {"Home"} </RouterAnchor<AppRoute>>
            </div>
        }
    }
}
