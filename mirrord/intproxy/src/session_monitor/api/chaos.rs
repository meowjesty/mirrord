use anyhow::Context;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{post, put},
};
use serde_json::Value;

use crate::session_monitor::{ChaosRuleJsonThingy, TempChaosRules, api::AppState};

/*

POST /chaos/rules/{session_id}: create rule, return rule object with assigned ID

GET /chaos/rules/{session_id}: list active rules for session

GET /chaos/rules/{session_id}/{rule_id}: get specific rule

PUT /chaos/rules/{session_id}/{rule_id}: update rule

DELETE /chaos/rules/{session_id}/{rule_id}: delete rule

DELETE /chaos/rules/{session_id}: clear all rules for session*/

// TODO(alex): ... we have a shared chaos state in the `AppState` of this thing.
//
// The `Receiver` side of this shared state has been passed to the `OutgoingProxy` (and whatever
// other background task we want), and shall live as the shared state of that task.
//
// > But alex, why a watcher channel? Why not share state with `Arc`?
//
// It's easier, the watcher keeps the most up-to-date info and is shareable, so we don't have to
// keep locking something whenever we want to see if a rule applies, we just need to check the
// channel.
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
    State(state): State<AppState>,
    Json(new_rule): Json<ChaosRuleJsonThingy>,
) -> ChaosResult<()> {
    tracing::info!(?new_rule);
    state.chaos_tx.create_rule(new_rule);

    Ok(())
}

async fn get_list_active_rules_for_session(
    Path(session_id): Path<String>,
    State(state): State<AppState>,
) -> ChaosResult<Json<TempChaosRules>> {
    Ok(Json(state.chaos_tx.list_active_rules_for_session()))
}

async fn delete_clear_session_rules(
    Path(session_id): Path<String>,
    State(state): State<AppState>,
) -> ChaosResult<()> {
    Ok(state.chaos_tx.clear_session_rules())
}

async fn put_update_rule(
    Path((session_id, rule_id)): Path<(String, String)>,
    State(state): State<AppState>,
) -> ChaosResult<()> {
    Ok(state.chaos_tx.update_rule(rule_id))
}

async fn delete_rule(
    Path((session_id, rule_id)): Path<(String, String)>,
    State(state): State<AppState>,
) -> ChaosResult<()> {
    Ok(state.chaos_tx.delete_rule(rule_id))
}

async fn get_rule(
    Path((session_id, rule_id)): Path<(String, String)>,
    State(state): State<AppState>,
) -> ChaosResult<Json<ChaosRuleJsonThingy>> {
    Ok(Json(state.chaos_tx.get_rule(rule_id).context("not found")?))
}
