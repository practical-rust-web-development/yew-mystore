use crate::routing::AppRoute;

use yew_router::prelude::RouterAnchor;
use yew::prelude::{html, Component, ComponentLink, ShouldRender};
use yew::virtual_dom::VNode;

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
            <div>
                <nav class="navbar navbar-expand-lg navbar-light bg-light">
                    <a class="navbar-brand" href="#">
                        <i class="fas fa-store"></i>
                        {"My Store"}
                    </a>
                    <RouterAnchor<AppRoute> route=AppRoute::Login> {"Login"} </RouterAnchor<AppRoute>>
                </nav>
                <div class="row">
                    <div class="mx-auto">
                        <RouterAnchor<AppRoute> route=AppRoute::Register>
                            <button class="btn btn-success" >
                                {"Sign Up"} 
                            </button>
                            </RouterAnchor<AppRoute>>
                    </div>
                </div>
                <hr />
                <div class="row">
                    <div class="mx-auto">
                        <i class="fas fa-store fa-10x text-center"></i>
                        <h2 class="text-center">{ "My Store" }</h2>
                    </div>
                </div>
            </div>
        }
    }
}
