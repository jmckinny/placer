mod board;
mod tile;
use axum::{
    extract::{self, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use board::Board;
use std::sync::{Arc, RwLock};
use tile::TileReq;

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(RwLock::new(StateData::default()));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/v1/place", post(place))
        .with_state(shared_state);

    println!("Running at http://127.0.0.1:3000/");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type AppState = Arc<RwLock<StateData>>;

#[derive(Default)]
struct StateData {
    board: Board,
}

async fn place(
    State(state): State<AppState>,
    extract::Json(req): extract::Json<TileReq>,
) -> StatusCode {
    if !req.is_valid() {
        return StatusCode::BAD_REQUEST;
    }
    let lock = state.as_ref();
    let mut state_data = lock.write().unwrap();
    let board = &mut state_data.board;
    board.set_tile(&req);

    println!("Processed: {req:?}");
    StatusCode::OK
}
