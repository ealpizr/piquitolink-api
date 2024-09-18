use axum::{
    response::IntoResponse, routing::get, Json, Router
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn hello_world() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "message": "Hello World!"
    });

    Json(json_response)
}