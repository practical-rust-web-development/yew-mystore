use yew_router::switch::Permissive;
use yew_router::Switch;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/register"]
    Register,
    #[to = "/login"]
    Login,
    #[to = "/"]
    Index,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}
