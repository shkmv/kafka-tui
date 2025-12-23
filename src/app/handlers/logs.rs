//! Log screen action handlers.

use crate::app::actions::{Action, Command};
use crate::app::state::{AppState, Level};

/// Handle log screen actions.
pub fn handle(state: &mut AppState, action: &Action) -> Option<Command> {
    match action {
        Action::ClearLogs => {
            state.logs_state.clear();
            Some(Command::None)
        }

        Action::CycleLogFilter => {
            state.logs_state.filter_level = match state.logs_state.filter_level {
                None => Some(Level::Error),
                Some(Level::Error) => Some(Level::Warning),
                Some(Level::Warning) => Some(Level::Success),
                Some(Level::Success) => Some(Level::Info),
                Some(Level::Info) => None,
            };
            state.logs_state.selected_index = 0;
            Some(Command::None)
        }

        Action::SetLogFilter(level) => {
            state.logs_state.filter_level = *level;
            state.logs_state.selected_index = 0;
            Some(Command::None)
        }

        _ => None,
    }
}
