use anyhow::Context;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{self, StatusCode},
    response::{IntoResponse, Response},
    routing::{post, put},
};
use serde_json::Value;

use crate::ui::AppState;

/*

POST /chaos/rules/{session_id}: create rule, return rule object with assigned ID

GET /chaos/rules/{session_id}: list active rules for session

GET /chaos/rules/{session_id}/{rule_id}: get specific rule

PUT /chaos/rules/{session_id}/{rule_id}: update rule

DELETE /chaos/rules/{session_id}/{rule_id}: delete rule

DELETE /chaos/rules/{session_id}: clear all rules for session*/

// TODO(alex): Ok, so this works sort of like this:
// Some random runs `mirrord ui`, it starts up the axum server (let's say the address is
// `ui:localhost/chaos`), and it's also running an axum server in the intproxy (address
// `monitor:localhost/chaos`), something called `session_monitor` (that's your keyword to search).
//
// The random wants to go mid (I mean, wants to create a rule), so they click some button in the ui
// that sends a POST request to `POST ui:localhost/chaos/1234`, it hits the route you're seeing
// here, and we use a `reqwest::Client` that's in the `TrackedSession` that's in `AppState` that's
// in this codebase that's in my computer that's in ...
//
// This `Client` is used to send a reqwest (lol) to `monitor:localhost/chaos/1234`, and in there ...
// (go to the file `chaos.rs` in `/intproxy`).
pub(super) fn chaos_router() -> Router<AppState> {
    Router::new()
        .route(
            "/rules/{session_id}",
            post(post_create_rule)
                .delete(delete_clear_session_rules)
                .get(get_list_active_rules_for_session),
        )
        .route(
            "/{session_id}/{rule_id}",
            put(put_update_rule).delete(delete_rule).get(get_rule),
        )
}

#[derive(Debug)]
struct ApiError(anyhow::Error);

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

type ChaosResult<T> = Result<T, ApiError>;

async fn post_create_rule(
    Path(session_id): Path<String>,
    State(state): State<AppState>,
    Json(new_rule): Json<Value>,
) -> ChaosResult<()> {
    let sessions = state.sessions.read().await;

    println!("{session_id} !!! {new_rule:?}");

    match sessions.get(&session_id) {
        Some(session) => {
            session
                .client
                .post(format!("http://localhost/chaos/rules/{session_id}"))
                .json(&new_rule)
                .send()
                .await?;
        }
        None => todo!(),
    }

    Ok(())
}

async fn get_list_active_rules_for_session(
    Path(session_id): Path<String>,
    State(_): State<AppState>,
) -> () {
}

async fn delete_clear_session_rules(
    Path(session_id): Path<String>,
    State(_): State<AppState>,
) -> () {
}

async fn put_update_rule(
    Path(session_id): Path<String>,
    Path(rule_id): Path<String>,
    State(_): State<AppState>,
) -> () {
}
async fn delete_rule(
    Path(session_id): Path<String>,
    Path(rule_id): Path<String>,
    State(_): State<AppState>,
) -> () {
}
async fn get_rule(
    Path(session_id): Path<String>,
    Path(rule_id): Path<String>,
    State(_): State<AppState>,
) -> () {
}
