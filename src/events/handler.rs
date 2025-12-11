use crossterm::event::{Event, KeyEvent, KeyEventKind};

use crate::app::actions::Action;
use crate::app::state::AppState;
use crate::events::key_bindings::{
    global_key_binding, help_key_binding, modal_key_binding, screen_key_binding,
};

pub struct EventHandler;

impl EventHandler {
    /// Convert a crossterm event to an Action
    pub fn handle_event(event: Event, state: &AppState) -> Option<Action> {
        match event {
            Event::Key(key) => Self::handle_key_event(key, state),
            Event::Resize(width, height) => Some(Action::Resize(width, height)),
            Event::Mouse(_) => None, // Mouse events not handled for now
            Event::FocusGained | Event::FocusLost => None,
            Event::Paste(_) => None, // Paste not handled for now
        }
    }

    /// Handle keyboard events
    pub fn handle_key_event(key: KeyEvent, state: &AppState) -> Option<Action> {
        // Only handle key press events (not release or repeat)
        // This ensures consistent behavior across platforms
        if key.kind != KeyEventKind::Press {
            return None;
        }

        // 1. If help is shown, handle help-specific keys
        if state.ui_state.show_help {
            return help_key_binding(key);
        }

        // 2. If a modal is open, handle modal-specific keys
        if let Some(ref modal) = state.ui_state.active_modal {
            return modal_key_binding(key, modal);
        }

        // 3. Try global key bindings first
        if let Some(action) = global_key_binding(key) {
            return Some(action);
        }

        // 4. Try screen-specific key bindings
        screen_key_binding(&state.active_screen, key, state.ui_state.sidebar_focused)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};

    fn make_key_event(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent::new_with_kind(code, modifiers, KeyEventKind::Press)
    }

    #[test]
    fn test_quit_keys() {
        let state = AppState::default();

        // Ctrl+C should quit
        let action = EventHandler::handle_key_event(
            make_key_event(KeyCode::Char('c'), KeyModifiers::CONTROL),
            &state,
        );
        assert!(matches!(action, Some(Action::Quit)));

        // Ctrl+Q should quit
        let action = EventHandler::handle_key_event(
            make_key_event(KeyCode::Char('q'), KeyModifiers::CONTROL),
            &state,
        );
        assert!(matches!(action, Some(Action::Quit)));
    }

    #[test]
    fn test_help_key() {
        let state = AppState::default();

        // ? should show help
        let action = EventHandler::handle_key_event(
            make_key_event(KeyCode::Char('?'), KeyModifiers::NONE),
            &state,
        );
        assert!(matches!(action, Some(Action::ShowHelp)));
    }

    #[test]
    fn test_navigation_keys() {
        let state = AppState::default();

        // Tab should focus content
        let action = EventHandler::handle_key_event(
            make_key_event(KeyCode::Tab, KeyModifiers::NONE),
            &state,
        );
        assert!(matches!(action, Some(Action::FocusContent)));

        // Shift+Tab should focus sidebar
        let action = EventHandler::handle_key_event(
            make_key_event(KeyCode::BackTab, KeyModifiers::SHIFT),
            &state,
        );
        assert!(matches!(action, Some(Action::FocusSidebar)));

        // Esc should go back
        let action = EventHandler::handle_key_event(
            make_key_event(KeyCode::Esc, KeyModifiers::NONE),
            &state,
        );
        assert!(matches!(action, Some(Action::GoBack)));
    }
}
