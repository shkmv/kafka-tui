//! Broker action handlers.

use crate::app::actions::{Action, Command};
use crate::app::state::{AppState, Level};

use super::super::update::toast;

/// Handle broker actions.
pub fn handle(state: &mut AppState, action: &Action) -> Option<Command> {
    match action {
        Action::FetchBrokers => {
            state.brokers_state.loading = true;
            Some(Command::FetchBrokerList)
        }

        Action::BrokersFetched { brokers, cluster_id } => {
            state.brokers_state.brokers = brokers.clone();
            state.brokers_state.cluster_id = cluster_id.clone();
            state.brokers_state.loading = false;
            Some(Command::None)
        }

        Action::BrokersFetchFailed(e) => {
            state.brokers_state.loading = false;
            toast(state, &format!("Failed to fetch brokers: {}", e), Level::Error);
            Some(Command::None)
        }

        _ => None,
    }
}
