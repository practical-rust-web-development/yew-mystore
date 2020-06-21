use chrono::NaiveDate;
use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::JsValue;

use crate::fetching::{send_request, FetchError, FetchResponse};

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/dashboard.graphql",
    response_derives = "Debug"
)]
pub struct Dashboard;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/list_sale.graphql",
    response_derives = "Debug"
)]
pub struct ListSale;

pub async fn fetch_graphql<T, R>(request_body: T) -> Result<FetchResponse<JsValue>, FetchError>
where
    T: Serialize,
    R: Serialize + for<'b> Deserialize<'b>,
{
    send_request::<T, R>("/graphql", Some(&request_body), "POST").await
}
