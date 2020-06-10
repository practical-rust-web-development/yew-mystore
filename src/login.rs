use crate::fetching::{send_future, send_request, FetchError, FetchState};
use crate::routing::AppRoute;
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use wasm_bindgen::prelude::JsValue;
use yew::prelude::{html, Component, ComponentLink, InputData, ShouldRender};
use yew::services::ConsoleService;
use yew::virtual_dom::VNode;
use yew_router::prelude::RouterAnchor;

#[derive(Clone)]
pub struct Model {
    link: ComponentLink<Self>,
    login_user: LoginUser,
}

#[derive(Serialize, Validate, Deserialize, Clone)]
pub struct LoginUser {
    #[validate(email)]
    #[validate(length(min = 1))]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

pub enum FormField {
    Email,
    Password,
}

pub enum Msg {
    Login,
    Logout,
    Logged(FetchState<JsValue>),
    UpdateForm(String, FormField),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            login_user: LoginUser {
                email: "".to_string(),
                password: "".to_string(),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let login_user = self.login_user.clone();
        match msg {
            Msg::Login => {
                let future = async move {
                    match login_user.validate() {
                        Ok(_) => {
                            match send_request("http://localhost:8088/login", &login_user, "POST")
                                .await
                            {
                                Ok(user) => Msg::Logged(FetchState::Success(user)),
                                Err(error) => Msg::Logged(FetchState::Failed(error)),
                            }
                        }
                        Err(error) => Msg::Logged(FetchState::Failed(FetchError {
                            err: JsValue::from(error.to_string()),
                        })),
                    }
                };
                send_future(self.link.clone(), future);
                self.link.send_message(Msg::Logged(FetchState::Fetching));
                true
            }
            Msg::Logout => false,
            Msg::Logged(fetch_state) => {
                match fetch_state {
                    FetchState::Success(_) => ConsoleService::new().log("success"),
                    FetchState::Failed(error) => ConsoleService::new().log(&error.to_string()),
                    FetchState::Fetching => ConsoleService::new().log("Fetching"),
                };
                false
            }
            Msg::UpdateForm(value, form_field) => {
                match form_field {
                    FormField::Email => self.login_user.email = value,
                    FormField::Password => self.login_user.password = value,
                };
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <div>
                <input name="email"
                       type="text"
                       oninput=self.link.callback(|e: InputData|
                           Msg::UpdateForm(e.value, FormField::Email)
                       )/>
                <input name="password"
                       type="password"
                       oninput=self.link.callback(|e: InputData|
                           Msg::UpdateForm(e.value, FormField::Password)
                       )/>
                <button onclick=self.link.callback(|_| Msg::Login)>{ "LogIn" }</button>

                <RouterAnchor<AppRoute> route=AppRoute::Index> {"Home"} </RouterAnchor<AppRoute>>
            </div>
        }
    }
}
