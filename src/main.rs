use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{post, get},
    Json, Router,
};
use axum_macros::debug_handler;
use code_executor::Rules;
use serde::{Deserialize, Serialize};
use std::{fs, net::SocketAddr};

use clap::Parser;

/// Simple Code runner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port for the app to run on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    /// If should only respond to localhost hotname or 0.0.0.0
    #[arg(short, long, default_value_t = true)]
    local: bool,

    #[arg(short, long, default_value = "rules.yaml")]
    filename: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let contents =
        fs::read_to_string(&args.filename).expect("Failed to read the config file");
    let rules: Rules = serde_yaml::from_str(&contents).expect("failed to parse rules");

    let app = Router::new()
        .route("/:engine_id", post(handle_run))
        .route("/", get(list_runners))
        .with_state(rules);

    let addr = {
        if args.local {
            SocketAddr::from(([127, 0, 0, 1], args.port))
        } else {
            SocketAddr::from(([0, 0, 0, 0], args.port))
        }
    };

    tracing::info!("listening on {}", addr);

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

async fn list_runners(State(rules): State<Rules>) -> impl IntoResponse {
    let keys = rules
        .rules
        .keys()
        .map(|k| k.to_owned())
        .collect::<Vec<String>>();
    Json(keys)
}
