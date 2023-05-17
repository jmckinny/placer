mod board;
mod tile;
use axum::{
    extract::{self, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use board::Board;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tile::TileReq;

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(RwLock::new(StateData::default()));
    // Setup logging
    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/", get(|| async { "Health Check" }))
        .route("/api/v1/place", post(place))
        .route("/api/v1/board", get(get_board))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on http://{:?}", addr);

    axum::Server::bind(&addr)
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
        tracing::info!("Dropped request {req:?}");
        return StatusCode::BAD_REQUEST;
    }
    let mut lock = state.write().unwrap();
    let board = &mut lock.board;
    board.set_tile(&req);

    tracing::info!("Processed request {req:?}");
    StatusCode::OK
}

async fn get_board(State(state): State<AppState>) -> Json<Board> {
    let lock = state.read().unwrap();
    tracing::info!("Processed get board state");
    Json(lock.board.clone())
}
