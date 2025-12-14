use ratatui::prelude::*;

use crate::app::state::{AppState, ModalType, Screen};
use crate::ui::components::{
    AddPartitionsFormModal, AlterConfigFormModal, ConfirmModal, ConnectionFormModal,
    Header, HelpModal, InputModal, ProduceFormModal, PurgeTopicFormModal, Sidebar,
    StatusBar, Toast, TopicCreateFormModal,
};
use crate::ui::layout::{welcome_layout, AppLayout};
use crate::ui::screens::{
    brokers::BrokersScreen,
    consumer_groups::{ConsumerGroupDetailsScreen, ConsumerGroupsListScreen},
    messages::MessageBrowserScreen,
    topics::{TopicDetailsScreen, TopicsListScreen},
    welcome::WelcomeScreen,
};

pub fn render_app(frame: &mut Frame, state: &AppState) {
    match &state.active_screen {
        Screen::Welcome => render_welcome(frame, state),
        _ => render_main(frame, state),
    }
    render_overlays(frame, state);
}

fn render_welcome(frame: &mut Frame, state: &AppState) {
    let layout = welcome_layout(frame.area());
    Header::render(frame, layout.header, state);
    WelcomeScreen::render(frame, layout.content, state);
    StatusBar::render(frame, layout.status, state);
}

fn render_main(frame: &mut Frame, state: &AppState) {
    let layout = AppLayout::new(frame.area());
    Header::render(frame, layout.header, state);
    Sidebar::render(frame, layout.sidebar, state);
    render_content(frame, layout.content, state);
    StatusBar::render(frame, layout.status, state);
}

fn render_content(frame: &mut Frame, area: Rect, state: &AppState) {
    match &state.active_screen {
        Screen::Welcome => WelcomeScreen::render(frame, area, state),
        Screen::Topics => TopicsListScreen::render(frame, area, state),
        Screen::TopicDetails { topic_name } => TopicDetailsScreen::render(frame, area, state, topic_name),
        Screen::Messages { topic_name } => MessageBrowserScreen::render(frame, area, state, topic_name),
        Screen::ConsumerGroups => ConsumerGroupsListScreen::render(frame, area, state),
        Screen::ConsumerGroupDetails { group_id } => ConsumerGroupDetailsScreen::render(frame, area, state, group_id),
        Screen::Brokers => BrokersScreen::render(frame, area, state),
    }
}

fn render_overlays(frame: &mut Frame, state: &AppState) {
    if state.ui_state.show_help {
        HelpModal::render(frame, &state.active_screen);
        return;
    }

    if let Some(modal) = &state.ui_state.active_modal {
        match modal {
            ModalType::Confirm { title, message, .. } => ConfirmModal::render(frame, title, message),
            ModalType::Input { title, placeholder, value, .. } => InputModal::render(frame, title, placeholder, value),
            ModalType::ConnectionForm(f) => ConnectionFormModal::render(frame, f),
            ModalType::TopicCreateForm(f) => TopicCreateFormModal::render(frame, f),
            ModalType::ProduceForm(f) => ProduceFormModal::render(frame, f),
            ModalType::AddPartitionsForm(f) => AddPartitionsFormModal::render(frame, f),
            ModalType::AlterConfigForm(f) => AlterConfigFormModal::render(frame, f),
            ModalType::PurgeTopicForm(f) => PurgeTopicFormModal::render(frame, f),
        }
    }

    Toast::render(frame, &state.ui_state.toast_messages);
}
