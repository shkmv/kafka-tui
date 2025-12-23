//! System-level action handlers (Tick, Quit, Resize).

use crate::app::actions::{Action, Command};
use crate::app::state::AppState;

use super::ui::expire_toasts;

/// Handle system-level actions.
pub fn handle(state: &mut AppState, action: &Action) -> Option<Command> {
    match action {
        Action::Tick => {
            expire_toasts(&mut state.ui_state.toast_messages);
            Some(Command::None)
        }
        Action::Quit => {
            state.running = false;
            Some(Command::None)
        }
        Action::Resize(_, _) => Some(Command::None),
        _ => None,
    }
}
