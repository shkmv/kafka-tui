//! Application state update logic.
//!
//! This module coordinates action handling by delegating to domain-specific handlers.

use crate::app::actions::{Action, Command};
use crate::app::state::AppState;

use super::handlers;

/// Process an action and return the resulting command.
///
/// Each handler module is tried in sequence. The first handler that returns
/// `Some(Command)` wins. If no handler processes the action, returns `Command::None`.
pub fn update(state: &mut AppState, action: Action) -> Command {
    // Try each handler in sequence
    // System actions (Tick, Quit, Resize)
    if let Some(cmd) = handlers::system::handle(state, &action) {
        return cmd;
    }

    // Navigation actions
    if let Some(cmd) = handlers::navigation::handle(state, &action) {
        return cmd;
    }

    // Connection actions
    if let Some(cmd) = handlers::connection::handle(state, &action) {
        return cmd;
    }

    // Topic actions
    if let Some(cmd) = handlers::topics::handle(state, &action) {
        return cmd;
    }

    // Message actions
    if let Some(cmd) = handlers::messages::handle(state, &action) {
        return cmd;
    }

    // Consumer group actions
    if let Some(cmd) = handlers::consumer_groups::handle(state, &action) {
        return cmd;
    }

    // Broker actions
    if let Some(cmd) = handlers::brokers::handle(state, &action) {
        return cmd;
    }

    // Log actions
    if let Some(cmd) = handlers::logs::handle(state, &action) {
        return cmd;
    }

    // UI/Modal actions
    if let Some(cmd) = handlers::ui::handle(state, &action) {
        return cmd;
    }

    // Action not handled by any handler
    Command::None
}

/// Add a toast message and log it.
///
/// Re-exported from the ui handler for external use.
pub fn toast(state: &mut AppState, msg: &str, level: crate::app::state::Level) {
    handlers::ui::toast(state, msg, level);
}
