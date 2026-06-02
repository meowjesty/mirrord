use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{post, put},
};
use serde_json::Value;

use crate::session_monitor::api::AppState;

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
            "/{session_id}",
            post(post_create_rule)
                .delete(delete_clear_session_rules)
                .get(get_list_active_rules_for_session),
        )
        .route(
            "/{session_id}/{rule_id}",
            put(put_update_rule).delete(delete_rule).get(get_rule),
        )
}

async fn post_create_rule(State(state): State<AppState>, Json(new_rule): Json<Value>) -> () {
    state.chaos_tx.0.send_modify(|current_rules| {
        current_rules.insert(new_rule);
    });
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
