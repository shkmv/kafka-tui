use ratatui::prelude::*;

use crate::app::state::{AppState, ModalType, Screen};
use crate::ui::components::{
    confirm_modal::ConfirmModal,
    help_modal::HelpModal,
    input_modal::InputModal,
    toast::Toast,
    ConnectionFormModal, Header, Sidebar, StatusBar, TopicCreateFormModal,
};
use crate::ui::layout::{welcome_layout, AppLayout};
use crate::ui::screens::{
    consumer_groups::{details::ConsumerGroupDetailsScreen, list::ConsumerGroupsListScreen},
    messages::{browser::MessageBrowserScreen, producer::MessageProducerScreen},
    topics::{create::TopicCreateScreen, details::TopicDetailsScreen, list::TopicsListScreen},
    welcome::WelcomeScreen,
};

/// Main render function - dispatches to appropriate screen renderer
pub fn render_app(frame: &mut Frame, state: &AppState) {
    // Render based on current screen
    match &state.active_screen {
        Screen::Welcome => render_welcome_screen(frame, state),
        _ => render_main_screen(frame, state),
    }

    // Render overlays (modals, help, toasts)
    render_overlays(frame, state);
}

fn render_welcome_screen(frame: &mut Frame, state: &AppState) {
    let layout = welcome_layout(frame.area());

    // Header (simplified for welcome)
    Header::render(frame, layout.header, state);

    // Welcome content
    WelcomeScreen::render(frame, layout.content, state);

    // Status bar
    StatusBar::render(frame, layout.status, state);
}

fn render_main_screen(frame: &mut Frame, state: &AppState) {
    let layout = AppLayout::new(frame.area());

    // Header
    Header::render(frame, layout.header, state);

    // Sidebar
    Sidebar::render(frame, layout.sidebar, state);

    // Content area - dispatch based on screen
    render_content(frame, layout.content, state);

    // Status bar
    StatusBar::render(frame, layout.status, state);
}

fn render_content(frame: &mut Frame, area: Rect, state: &AppState) {
    match &state.active_screen {
        Screen::Welcome => {
            // Shouldn't reach here, but handle gracefully
            WelcomeScreen::render(frame, area, state);
        }
        Screen::Topics => {
            TopicsListScreen::render(frame, area, state);
        }
        Screen::TopicDetails { topic_name } => {
            TopicDetailsScreen::render(frame, area, state, topic_name);
        }
        Screen::TopicCreate => {
            TopicCreateScreen::render(frame, area, state);
        }
        Screen::Messages { topic_name } => {
            MessageBrowserScreen::render(frame, area, state, topic_name);
        }
        Screen::MessageProducer { topic_name } => {
            MessageProducerScreen::render(frame, area, state, topic_name);
        }
        Screen::ConsumerGroups => {
            ConsumerGroupsListScreen::render(frame, area, state);
        }
        Screen::ConsumerGroupDetails { group_id } => {
            ConsumerGroupDetailsScreen::render(frame, area, state, group_id);
        }
    }
}

fn render_overlays(frame: &mut Frame, state: &AppState) {
    // Help modal (highest priority)
    if state.ui_state.show_help {
        HelpModal::render(frame, &state.active_screen);
        return; // Don't render other overlays when help is shown
    }

    // Modal dialogs
    if let Some(ref modal) = state.ui_state.active_modal {
        match modal {
            ModalType::Confirm { title, message, .. } => {
                ConfirmModal::render(frame, title, message);
            }
            ModalType::Input {
                title,
                placeholder,
                value,
                ..
            } => {
                InputModal::render(frame, title, placeholder, value);
            }
            ModalType::ConnectionForm(form_state) => {
                ConnectionFormModal::render(frame, form_state);
            }
            ModalType::TopicCreateForm(form_state) => {
                TopicCreateFormModal::render(frame, form_state);
            }
        }
    }

    // Toast notifications (always render on top)
    Toast::render(frame, &state.ui_state.toast_messages);
}
