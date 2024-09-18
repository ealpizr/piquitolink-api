use axum::{
    response::IntoResponse, routing::get, Json, Router
};
use shuttle_runtime::SecretStore;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let secret = secrets.get("SECRET").expect("SECRET not found");
    
    let router = Router::new().route("/", get(hello_world));
    Ok(router.into())
}

pub async fn hello_world() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "message": "Hello World!"
    });

    Json(json_response)
}