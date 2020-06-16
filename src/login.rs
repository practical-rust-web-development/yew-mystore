use crate::fetching::{
    save_token, send_future, send_request, FetchError, FetchResponse, FetchState,
};
use crate::routing::{AppRoute, Redirecter};
use crate::CurrentUser;
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use wasm_bindgen::prelude::JsValue;
use yew::prelude::{html, Component, ComponentLink, InputData, ShouldRender};
use yew::services::ConsoleService;
use yew::virtual_dom::VNode;
use yew_router::prelude::RouterAnchor;

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
    Logged(FetchState<FetchResponse<JsValue>>),
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
                        Ok(_) => match send_request::<LoginUser, CurrentUser>(
                            "/login",
                            Some(&login_user),
                            "POST",
                        )
                        .await
                        {
                            Ok(response) => Msg::Logged(FetchState::Success(response)),
                            Err(error) => Msg::Logged(FetchState::Failed(error)),
                        },
                        Err(error) => Msg::Logged(FetchState::Failed(FetchError {
                            err: JsValue::from(error.to_string()),
                        })),
                    }
                };
                send_future(self.link.clone(), future);
                true
            }
            Msg::Logged(fetch_state) => {
                match fetch_state {
                    FetchState::Success(response) => {
                        if let Err(_) = save_token(response.headers) {
                            ConsoleService::new().log("Error saving token!");
                        }
                        let mut redirecter = Redirecter::new();
                        redirecter.redirect(AppRoute::Dashboard);
                        ConsoleService::new().log("Success")
                    }
                    FetchState::Failed(error) => {
                        ConsoleService::new().log(&format!("Error: {}", &error.to_string()))
                    }
                    FetchState::Fetching => ConsoleService::new().log("Fetching"),
                };
                true
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
            <div class="col-lg-6 text-center border mx-auto p-5">
                <p class="h4 mb-4"> { "LogIn" }</p>

                <div class="form-row mb-4">
                    <input name="email"
                        type="text"
                        class="form-control"
                        placeholder="email"
                        oninput=self.link.callback(|e: InputData|
                            Msg::UpdateForm(e.value, FormField::Email)
                        )/>
                </div>
                <div class="form-row mb-4">
                <input name="password"
                       type="password"
                       class="form-control"
                        placeholder="password"
                       oninput=self.link.callback(|e: InputData|
                           Msg::UpdateForm(e.value, FormField::Password)
                       )/>
                </div>
                <button onclick=self.link.callback(|_| Msg::Login)
                        class="btn btn-info my-4 btn-block">{ "LogIn" }</button>

                <hr />
                <RouterAnchor<AppRoute> route=AppRoute::Index> {"Home"} </RouterAnchor<AppRoute>>
            </div>
        }
    }
}
