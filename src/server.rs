use std::{net::SocketAddr, sync::Arc};

use crate::{
    common::{Ack, Address, PubKey, Receipt, State, Transfer},
    error::Error,
    validator::Validator,
};
use axum::{
    extract,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use tokio::{
    sync::{oneshot, Mutex},
    task::JoinHandle,
};

struct Context {
    validator: Validator,
}

type Ctx = Arc<Mutex<Context>>;

pub async fn serve(
    address: &str,
    validator: Validator,
) -> (SocketAddr, impl FnOnce(), JoinHandle<()>) {
    let ctx = Arc::new(Mutex::new(Context { validator }));

    let app = Router::new()
        .route("/id", get(id))
        .route("/accept", post(accept))
        .route("/confirm", post(confirm))
        .route("/state/{id}", get(lookup))
        .with_state(ctx);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let address = listener.local_addr().unwrap();
    tracing::debug!("listening on {}", address);

    let (tx, rx) = oneshot::channel();
    let shutdown = move || {
        let _ = tx.send(());
    };

    let handle = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = rx.await;
            })
            .await
            .unwrap();
    });

    (address, shutdown, handle)
}

enum AppError {
    Error(Error),
    AddressMissing(String),
}

impl<E> From<E> for AppError
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        Self::Error(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Error(e) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            AppError::AddressMissing(message) => {
                (axum::http::StatusCode::NOT_FOUND, message).into_response()
            }
        }
    }
}

async fn id(extract::State(ctx): extract::State<Ctx>) -> Result<Json<PubKey>, AppError> {
    let id = ctx.lock().await.validator.id()?;
    Ok(Json(id))
}

async fn lookup(
    extract::State(ctx): extract::State<Ctx>,
    extract::Path(pubkey): extract::Path<Address>,
) -> Result<Json<State>, AppError> {
    let Some(state) = ctx.lock().await.validator.lookup(&pubkey) else {
        return Err(AppError::AddressMissing(format!(
            "No such address: {}",
            hex::encode(pubkey.0)
        )));
    };
    Ok(Json(state))
}

async fn accept(
    extract::State(ctx): extract::State<Ctx>,
    extract::Json(tx): extract::Json<Transfer>,
) -> Result<Json<Ack>, AppError> {
    tx.check()?;
    let ack = ctx.lock().await.validator.accept(tx)?;
    Ok(Json(ack))
}

async fn confirm(
    extract::State(ctx): extract::State<Ctx>,
    extract::Json(receipt): extract::Json<Receipt>,
) -> Result<Json<Receipt>, AppError> {
    ctx.lock().await.validator.confirm(&receipt)?;
    Ok(Json(receipt))
}
