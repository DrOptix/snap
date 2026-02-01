use axum::{
    Router,
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_rusqlite::Connection;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db: Arc<Connection>,
}

#[tokio::main]
async fn main() {
    let conn = Connection::open("snap.db")
        .await
        .expect("Failed to open database");

    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS snaps (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                created INTEGER NOT NULL
            )",
            [],
        )
        .map_err(tokio_rusqlite::Error::from)
    })
    .await
    .expect("Failed to create table");

    let state = AppState { db: Arc::new(conn) };

    let app = Router::new()
        .route("/", get(handler))
        .route("/snap", post(snap_post_handler))
        .route("/snap/{id}", get(snap_get_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    match tokio::fs::read_to_string("./html/index.html").await {
        Ok(html) => (StatusCode::OK, Html(html)).into_response(),
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                (StatusCode::NOT_FOUND, "Not Found").into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
    }
}

async fn snap_post_handler(
    State(state): State<AppState>,
    Form(params): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    let content = params.get("content").cloned().unwrap_or_default();
    if content.is_empty() {
        return (StatusCode::BAD_REQUEST, "Content cannot be empty").into_response();
    }

    let id = Uuid::new_v4().to_string();
    let created = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let id_clone = id.clone();
    let content_clone = content.clone();

    let result = state
        .db
        .call(move |conn| {
            conn.execute(
                "INSERT INTO snaps (id, content, created) VALUES (?, ?, ?)",
                (id_clone, content_clone, created),
            )
            .map_err(tokio_rusqlite::Error::from)
        })
        .await;

    match result {
        Ok(_) => Redirect::to(&format!("/snap/{}", id)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save snap").into_response(),
    }
}

async fn snap_get_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let id_clone = id.clone();
    let result = state
        .db
        .call(move |conn| {
            let mut stmt = conn.prepare("SELECT content FROM snaps WHERE id = ?")?;
            let content: String = stmt.query_row([id_clone], |row| row.get(0))?;
            Ok(content)
        })
        .await;

    match result {
        Ok(content) => match tokio::fs::read_to_string("./html/snap.html").await {
            Ok(template) => {
                let escaped_html = html_escape::encode_safe(&content);
                let html = template.replace("{{ content }}", &escaped_html);
                (StatusCode::OK, Html(html)).into_response()
            }
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
        },
        Err(_) => (StatusCode::NOT_FOUND, "Snap not found").into_response(),
    }
}
