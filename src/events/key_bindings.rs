use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::actions::Action;
use crate::app::state::*;

pub fn global_key_binding(key: KeyEvent) -> Option<Action> {
    match (key.modifiers, key.code) {
        (KeyModifiers::CONTROL, KeyCode::Char('c' | 'q')) => Some(Action::Quit),
        (KeyModifiers::NONE, KeyCode::Char('?')) | (_, KeyCode::F(1)) => Some(Action::ShowHelp),
        (KeyModifiers::NONE, KeyCode::Tab) => Some(Action::FocusContent),
        (KeyModifiers::SHIFT, KeyCode::BackTab) => Some(Action::FocusSidebar),
        (KeyModifiers::NONE, KeyCode::Esc) => Some(Action::GoBack),
        (KeyModifiers::NONE, KeyCode::Char('1')) => Some(Action::SelectSidebarItem(SidebarItem::Topics)),
        (KeyModifiers::NONE, KeyCode::Char('2')) => Some(Action::SelectSidebarItem(SidebarItem::ConsumerGroups)),
        (KeyModifiers::NONE, KeyCode::Char('3')) => Some(Action::SelectSidebarItem(SidebarItem::Brokers)),
        _ => None,
    }
}

pub fn help_key_binding(key: KeyEvent) -> Option<Action> {
    matches!(key.code, KeyCode::Esc | KeyCode::Char('?' | 'q') | KeyCode::Enter)
        .then_some(Action::HideHelp)
}

pub fn modal_key_binding(key: KeyEvent, modal: &ModalType) -> Option<Action> {
    match modal {
        ModalType::Confirm { .. } => match key.code {
            KeyCode::Enter | KeyCode::Char('y' | 'Y') => Some(Action::ModalConfirm),
            KeyCode::Esc | KeyCode::Char('n' | 'N') => Some(Action::ModalCancel),
            _ => None,
        },
        ModalType::Input { value, .. } => match key.code {
            KeyCode::Enter => Some(Action::ModalConfirm),
            KeyCode::Esc => Some(Action::ModalCancel),
            KeyCode::Char(c) => Some(Action::UpdateModalInput(format!("{}{}", value, c))),
            KeyCode::Backspace => Some(Action::UpdateModalInput(value[..value.len().saturating_sub(1)].into())),
            _ => None,
        },
        ModalType::ConnectionForm(f) => connection_form_key(key, f),
        ModalType::TopicCreateForm(f) => topic_form_key(key, f),
    }
}

fn connection_form_key(key: KeyEvent, f: &ConnectionFormState) -> Option<Action> {
    let mut s = f.clone();
    match key.code {
        KeyCode::Esc => return Some(Action::ModalCancel),
        KeyCode::Enter => {
            let ok = !f.name.is_empty() && !f.brokers.is_empty()
                && (!f.auth_type.requires_credentials() || (!f.username.is_empty() && !f.password.is_empty()));
            return ok.then_some(Action::ModalConfirm);
        }
        KeyCode::Tab | KeyCode::Down => s.focused_field = conn_field_next(&f.focused_field, &f.auth_type),
        KeyCode::BackTab | KeyCode::Up => s.focused_field = conn_field_prev(&f.focused_field, &f.auth_type),
        KeyCode::Left if f.focused_field == ConnectionFormField::AuthType => {
            s.auth_type = f.auth_type.prev();
            if !s.auth_type.requires_credentials() { s.username.clear(); s.password.clear(); }
        }
        KeyCode::Right if f.focused_field == ConnectionFormField::AuthType => {
            s.auth_type = f.auth_type.next();
            if !s.auth_type.requires_credentials() { s.username.clear(); s.password.clear(); }
        }
        KeyCode::Char(c) => match f.focused_field {
            ConnectionFormField::Name => s.name.push(c),
            ConnectionFormField::Brokers => s.brokers.push(c),
            ConnectionFormField::AuthType => return None,
            ConnectionFormField::Username => s.username.push(c),
            ConnectionFormField::Password => s.password.push(c),
        },
        KeyCode::Backspace => match f.focused_field {
            ConnectionFormField::Name => { s.name.pop(); }
            ConnectionFormField::Brokers => { s.brokers.pop(); }
            ConnectionFormField::AuthType => return None,
            ConnectionFormField::Username => { s.username.pop(); }
            ConnectionFormField::Password => { s.password.pop(); }
        },
        _ => return None,
    }
    Some(Action::UpdateConnectionForm(s))
}

fn conn_field_next(f: &ConnectionFormField, auth: &AuthType) -> ConnectionFormField {
    match f {
        ConnectionFormField::Name => ConnectionFormField::Brokers,
        ConnectionFormField::Brokers => ConnectionFormField::AuthType,
        ConnectionFormField::AuthType => if auth.requires_credentials() { ConnectionFormField::Username } else { ConnectionFormField::Name },
        ConnectionFormField::Username => ConnectionFormField::Password,
        ConnectionFormField::Password => ConnectionFormField::Name,
    }
}

fn conn_field_prev(f: &ConnectionFormField, auth: &AuthType) -> ConnectionFormField {
    match f {
        ConnectionFormField::Name => if auth.requires_credentials() { ConnectionFormField::Password } else { ConnectionFormField::AuthType },
        ConnectionFormField::Brokers => ConnectionFormField::Name,
        ConnectionFormField::AuthType => ConnectionFormField::Brokers,
        ConnectionFormField::Username => ConnectionFormField::AuthType,
        ConnectionFormField::Password => ConnectionFormField::Username,
    }
}

fn topic_form_key(key: KeyEvent, f: &TopicCreateFormState) -> Option<Action> {
    let mut s = f.clone();
    match key.code {
        KeyCode::Esc => return Some(Action::ModalCancel),
        KeyCode::Enter => return (!f.name.is_empty()).then_some(Action::ModalConfirm),
        KeyCode::Tab | KeyCode::Down => s.focused_field = match f.focused_field {
            TopicCreateFormField::Name => TopicCreateFormField::Partitions,
            TopicCreateFormField::Partitions => TopicCreateFormField::ReplicationFactor,
            TopicCreateFormField::ReplicationFactor => TopicCreateFormField::Name,
        },
        KeyCode::BackTab | KeyCode::Up => s.focused_field = match f.focused_field {
            TopicCreateFormField::Name => TopicCreateFormField::ReplicationFactor,
            TopicCreateFormField::Partitions => TopicCreateFormField::Name,
            TopicCreateFormField::ReplicationFactor => TopicCreateFormField::Partitions,
        },
        KeyCode::Char(c) => match f.focused_field {
            TopicCreateFormField::Name => s.name.push(c),
            TopicCreateFormField::Partitions if c.is_ascii_digit() => s.partitions.push(c),
            TopicCreateFormField::ReplicationFactor if c.is_ascii_digit() => s.replication_factor.push(c),
            _ => return None,
        },
        KeyCode::Backspace => match f.focused_field {
            TopicCreateFormField::Name => { s.name.pop(); }
            TopicCreateFormField::Partitions => { s.partitions.pop(); }
            TopicCreateFormField::ReplicationFactor => { s.replication_factor.pop(); }
        },
        _ => return None,
    }
    Some(Action::UpdateTopicCreateForm(s))
}

pub fn screen_key_binding(screen: &Screen, key: KeyEvent, sidebar_focused: bool) -> Option<Action> {
    if sidebar_focused {
        return match key.code {
            KeyCode::Up | KeyCode::Char('k') => Some(Action::MoveUp),
            KeyCode::Down | KeyCode::Char('j') => Some(Action::MoveDown),
            KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => Some(Action::Select),
            _ => None,
        };
    }

    // List navigation
    let nav = match key.code {
        KeyCode::Up | KeyCode::Char('k') => Some(Action::MoveUp),
        KeyCode::Down | KeyCode::Char('j') => Some(Action::MoveDown),
        KeyCode::PageUp => Some(Action::PageUp),
        KeyCode::PageDown => Some(Action::PageDown),
        KeyCode::Home | KeyCode::Char('g') => Some(Action::ScrollToTop),
        KeyCode::End | KeyCode::Char('G') => Some(Action::ScrollToBottom),
        _ => None,
    };
    if nav.is_some() { return nav; }

    match screen {
        Screen::Welcome => match key.code {
            KeyCode::Enter => Some(Action::Select),
            KeyCode::Char('n') => Some(Action::ShowModal(ModalType::ConnectionForm(Default::default()))),
            KeyCode::Char('d') => Some(Action::RequestDeleteConnection),
            _ => None,
        },
        Screen::Topics => match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Enter | KeyCode::Char('m')) => Some(Action::Select),
            (KeyModifiers::NONE, KeyCode::Char('n')) => Some(Action::ShowModal(ModalType::TopicCreateForm(Default::default()))),
            (KeyModifiers::NONE, KeyCode::Char('/')) => Some(Action::ShowModal(ModalType::Input {
                title: "Filter Topics".into(), placeholder: "Enter filter".into(), value: String::new(), action: InputAction::FilterTopics,
            })),
            (KeyModifiers::CONTROL, KeyCode::Char('l')) => Some(Action::ClearTopicFilter),
            (KeyModifiers::CONTROL, KeyCode::Char('r')) | (_, KeyCode::F(5)) => Some(Action::FetchTopics),
            _ => None,
        },
        Screen::TopicDetails { topic_name } => match key.code {
            KeyCode::Left | KeyCode::Char('h') => Some(Action::MoveLeft),
            KeyCode::Right | KeyCode::Char('l') => Some(Action::MoveRight),
            KeyCode::Char('m') => Some(Action::ViewTopicMessages(topic_name.clone())),
            KeyCode::F(5) => Some(Action::FetchTopics),
            _ => None,
        },
        Screen::Messages { topic_name } => match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Char('v') | KeyCode::Enter) => Some(Action::ToggleMessageDetail),
            (KeyModifiers::NONE, KeyCode::Char('p')) => Some(Action::ShowModal(ModalType::Input {
                title: "Produce Message".into(), placeholder: "Enter message".into(), value: String::new(),
                action: InputAction::ProduceMessage { topic: topic_name.clone() },
            })),
            (KeyModifiers::CONTROL, KeyCode::Char('r')) | (_, KeyCode::F(5)) => Some(Action::FetchMessages {
                topic: topic_name.clone(), offset_mode: OffsetMode::Latest, partition: None,
            }),
            (KeyModifiers::CONTROL, KeyCode::Char('l')) => Some(Action::ClearMessages),
            _ => None,
        },
        Screen::ConsumerGroups => match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Enter) => Some(Action::Select),
            (KeyModifiers::NONE, KeyCode::Char('/')) => Some(Action::ShowModal(ModalType::Input {
                title: "Filter Groups".into(), placeholder: "Enter filter".into(), value: String::new(), action: InputAction::FilterConsumerGroups,
            })),
            (KeyModifiers::CONTROL, KeyCode::Char('l')) => Some(Action::ClearConsumerGroupFilter),
            (KeyModifiers::CONTROL, KeyCode::Char('r')) | (_, KeyCode::F(5)) => Some(Action::FetchConsumerGroups),
            _ => None,
        },
        Screen::ConsumerGroupDetails { .. } => match key.code {
            KeyCode::Left | KeyCode::Char('h') => Some(Action::MoveLeft),
            KeyCode::Right | KeyCode::Char('l') => Some(Action::MoveRight),
            KeyCode::F(5) => Some(Action::FetchConsumerGroups),
            _ => None,
        },
        Screen::TopicCreate | Screen::MessageProducer { .. } => match key.code {
            KeyCode::Esc => Some(Action::GoBack),
            _ => None,
        },
    }
}

pub fn get_help_text(screen: &Screen) -> Vec<(&'static str, &'static str)> {
    let mut h = vec![("q/Ctrl+C", "Quit"), ("?", "Help"), ("Tab", "Switch panel"), ("Esc", "Back"), ("1-3", "Navigate")];
    h.extend(match screen {
        Screen::Welcome => vec![("Enter", "Connect"), ("n", "New"), ("d", "Delete")],
        Screen::Topics => vec![("j/k", "Navigate"), ("Enter", "Messages"), ("n", "New"), ("/", "Filter"), ("Ctrl+R", "Refresh")],
        Screen::Messages { .. } => vec![("j/k", "Navigate"), ("v", "Detail"), ("p", "Produce"), ("Ctrl+R", "Refresh")],
        Screen::ConsumerGroups => vec![("j/k", "Navigate"), ("Enter", "Details"), ("/", "Filter"), ("Ctrl+R", "Refresh")],
        Screen::TopicDetails { .. } | Screen::ConsumerGroupDetails { .. } => vec![("h/l", "Tabs"), ("F5", "Refresh")],
        _ => vec![],
    });
    h
}
