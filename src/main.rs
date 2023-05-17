mod board;
mod tile;
use askama::Template;
use axum::{
    extract::{self, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use board::Board;
use std::net::SocketAddr;
use std::sync::Arc;
use tile::TileReq;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(RwLock::new(StateData::default()));
    // Setup logging
    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/", get(index))
        .route("/static/index.js", get(index_js))
        .route("/health", get(|| async { "Health Check" }))
        .route("/api/v1/place", post(place))
        .route("/api/v1/board", get(get_board))
        .route("/api/v1/size", get(get_size))
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

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

async fn place(
    State(state): State<AppState>,
    extract::Json(req): extract::Json<TileReq>,
) -> StatusCode {
    if !req.is_valid() {
        tracing::info!("Dropped request {req:?}");
        return StatusCode::BAD_REQUEST;
    }
    let mut lock = state.write().await;
    let board = &mut lock.board;
    board.set_tile(&req);

    tracing::info!("Processed request {req:?}");
    StatusCode::OK
}

async fn get_board(State(state): State<AppState>) -> Json<Board> {
    let lock = state.read().await;
    Json(lock.board.clone())
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

async fn index_js() -> impl IntoResponse {
    if let Ok(data) = tokio::fs::read_to_string("static/index.js").await {
        data.into_response()
    } else {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

async fn get_size(State(state): State<AppState>) -> impl IntoResponse {
    let size = state.read().await.board.get_size();
    Json(size)
}
