use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::post,
    Json, Router
};
use code_executor::Rules;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    net::SocketAddr,
};
use axum_macros::debug_handler;


const FILE_PATH: &str = "rules.yaml";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let rules: Rules = serde_yaml::from_str(&contents).expect("failed to parse rules");
    
    let app = Router::new()
        .route("/:engine_id", post(handle_run))
        .with_state(rules);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
struct ToRun {
    content: String,
}

#[debug_handler]
async fn handle_run(
    Path(engine_id): Path<String>,
    State(rules): State<Rules>,
    Json(input): Json<ToRun>,
) -> impl IntoResponse {
    let engine = rules
        .rules
        .get(&engine_id)
        .expect("Missing runner configuration");
    let result = engine.run(&input.content).unwrap();
    Json(result)
}
