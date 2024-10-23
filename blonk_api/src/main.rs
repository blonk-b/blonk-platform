use axum::{
    extract::{Json, Path, State},
    response::Result as AxumResult,
    routing::{get, patch, post},
    Router,
};
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    id: i64,
    transaction_index: i32,
    user_id: String,
    signature: String,
    status: u8,
    message_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct CreateTransactionBody {
    transaction_index: i32,
    user_id: String,
    signature: String,
}

#[derive(Deserialize)]
struct UpdateTransactionBody {
    message_id: String,
}

struct AppState {
    conn: Mutex<Connection>,
}

#[tokio::main]
async fn main() {
    let path = "./db.sqlite";
    let conn = Connection::open(path).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (
            id                      INTEGER PRIMARY KEY,
            transaction_index       INTEGER NOT NULL,
            user_id                 TEXT NOT NULL,
            signature               TEXT NOT NULL,
            status                  INTEGER NOT NULL,
            message_id              TEXT
        )",
        (),
    )
    .unwrap();

    let app_state = Arc::new(AppState {
        conn: Mutex::new(conn),
    });

    let app = Router::new()
        .route("/transactions", post(create_transaction))
        .route("/transactions/:transaction_id", get(get_transaction))
        .route("/transactions/:transaction_id", patch(update_transaction))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn create_transaction(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateTransactionBody>,
) -> AxumResult<Json<Transaction>> {
    let conn = state.conn.lock().await;
    conn.execute(
        "INSERT INTO entries (transaction_index, user_id, signature, status, message_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&body.transaction_index, &body.user_id, &body.signature, 0, None::<String>),
    )
    .unwrap();
    let id = conn.last_insert_rowid();
    Ok(Json(Transaction {
        id,
        transaction_index: body.transaction_index,
        user_id: body.user_id,
        signature: body.signature,
        status: 0,
        message_id: None,
    }))
}

async fn get_transaction(
    State(state): State<Arc<AppState>>,
    Path(transaction_id): Path<i32>,
) -> Json<Option<Transaction>> {
    let conn = state.conn.lock().await;
    let mut stmt = conn
        .prepare("SELECT id, transaction_index, user_id, signature, status, message_id FROM entries WHERE id = ?1")
        .unwrap();
    let transaction = stmt
        .query_row([transaction_id], |row| {
            Ok(Transaction {
                id: row.get(0)?,
                transaction_index: row.get(1)?,
                user_id: row.get(2)?,
                signature: row.get(3)?,
                status: row.get(4)?,
                message_id: row.get(5)?,
            })
        })
        .optional()
        .unwrap();

    Json(transaction)
}

async fn update_transaction(
    State(state): State<Arc<AppState>>,
    Path(transaction_id): Path<i64>,
    Json(body): Json<UpdateTransactionBody>,
) -> Json<Transaction> {
    let conn = state.conn.lock().await;
    let _result = conn.execute(
        "UPDATE entries SET status = 1, message_id = ?1 WHERE id = ?2",
        (&Some(body.message_id), &transaction_id),
    );

    let mut stmt = conn
        .prepare("SELECT id, transaction_index, user_id, signature, status, message_id FROM entries WHERE id = ?1")
        .unwrap();
    let transaction = stmt
        .query_row([transaction_id], |row| {
            Ok(Transaction {
                id: row.get(0)?,
                transaction_index: row.get(1)?,
                user_id: row.get(2)?,
                signature: row.get(3)?,
                status: row.get(4)?,
                message_id: row.get(5)?,
            })
        })
        .unwrap();

    Json(transaction)
}
