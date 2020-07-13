use crate::State;
use elasticsearch::cat::CatIndicesParts;
use elasticsearch::http::response::Response as EsResponse;

use tide::http::StatusCode;
use tide::Request;
use tide::Response;
use serde_json::{json, Value};

// #[derive(Debug)]
pub async fn health(mut req: Request<State>)-> tide::Result {
    let client = &req.state().es_client;
    
    let response: EsResponse = client
        .cat()
        .indices(CatIndicesParts::Index(&["*"]))
        .send()
        .await?;
    let response_body = response.json::<Value>().await?;
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(response_body);
    Ok(res)
}
