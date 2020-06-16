use yew::agent::{Dispatched, Dispatcher};
use yew_router::switch::Permissive;
use yew_router::Switch;
use yew_router::{agent::RouteAgent, agent::RouteRequest, route::Route};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/register"]
    Register,
    #[to = "/login"]
    Login,
    #[to = "/dashboard"]
    Dashboard,
    #[to = "/"]
    Index,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

pub struct Redirecter {
    pub router: Dispatcher<RouteAgent>,
}

impl Redirecter {
    pub fn new() -> Self {
        let router = RouteAgent::dispatcher();
        Self { router }
    }

    pub fn redirect(&mut self, route: AppRoute) {
        self.router
            .send(RouteRequest::ReplaceRoute(Route::from(route)));
    }
}