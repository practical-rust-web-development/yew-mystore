use chrono::NaiveDate;
use graphql_client::{GraphQLQuery, QueryBody};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::JsValue;

use crate::fetching::{send_request, FetchError, FetchResponse};

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct Dashboard;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct ListSale;

//macro_rules! fetch_graphql {
//    ($name: ident, $struct: ident, $variables: ident, $($element: ident: $ty: expr),*) => {
//        paste::item! {
//            pub async fn [<fetch_graphql_ $name>]() -> Result<FetchResponse<JsValue>, FetchError> {
//                let request_body = $struct::build_query($variables { $($element: $ty),* });
//
//                send_request::<QueryBody<$name::Variables>, String>("/graphql", Some(&request_body), "POST")
//                    .await
//            }
//        }
//    };
//}
//
//use dashboard::Variables as dashboard_variables;
//use list_sale::Variables as sale_list_variables;
//fetch_graphql!(dashboard, Dashboard, dashboard_variables,);
//fetch_graphql!(list_sale, ListSale, sale_list_variables, search: Some("".to_string()), limit: 10);

//let request_body = Dashboard::build_query(variables);
pub async fn fetch_graphql<T, R>(request_body: T) -> Result<FetchResponse<JsValue>, FetchError>
where
    T: Serialize,
    R: Serialize + for<'b> Deserialize<'b>,
{
    send_request::<T, R>("/graphql", Some(&request_body), "POST").await
}
