use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::actions::Action;
use crate::app::state::*;

pub fn global_key_binding(key: KeyEvent) -> Option<Action> {
    match (key.modifiers, key.code) {
        (KeyModifiers::CONTROL, KeyCode::Char('c' | 'q')) => Some(Action::Quit),
        (KeyModifiers::NONE, KeyCode::Char('q')) => Some(Action::Quit),
        (KeyModifiers::NONE, KeyCode::Char('?')) | (_, KeyCode::F(1)) => Some(Action::ShowHelp),
        (KeyModifiers::NONE, KeyCode::Tab) => Some(Action::FocusContent),
        (KeyModifiers::SHIFT, KeyCode::BackTab) => Some(Action::FocusSidebar),
        (KeyModifiers::NONE, KeyCode::Esc) => Some(Action::GoBack),
        (KeyModifiers::NONE, KeyCode::Char('1')) => Some(Action::SelectSidebarItem(SidebarItem::Topics)),
        (KeyModifiers::NONE, KeyCode::Char('2')) => Some(Action::SelectSidebarItem(SidebarItem::ConsumerGroups)),
        (KeyModifiers::NONE, KeyCode::Char('3')) => Some(Action::SelectSidebarItem(SidebarItem::Brokers)),
        (KeyModifiers::NONE, KeyCode::Char('4')) => Some(Action::SelectSidebarItem(SidebarItem::Logs)),
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
        ModalType::ProduceForm(f) => produce_form_key(key, f),
        ModalType::AddPartitionsForm(f) => add_partitions_form_key(key, f),
        ModalType::AlterConfigForm(f) => alter_config_form_key(key, f),
        ModalType::PurgeTopicForm(f) => purge_topic_form_key(key, f),
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
        KeyCode::Tab | KeyCode::Down => s.focused_field = conn_next(&f.focused_field, &f.auth_type),
        KeyCode::BackTab | KeyCode::Up => s.focused_field = conn_prev(&f.focused_field, &f.auth_type),
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
            ConnectionFormField::ConsumerGroup => s.consumer_group.push(c),
            ConnectionFormField::Username => s.username.push(c),
            ConnectionFormField::Password => s.password.push(c),
            _ => return None,
        },
        KeyCode::Backspace => match f.focused_field {
            ConnectionFormField::Name => { s.name.pop(); }
            ConnectionFormField::Brokers => { s.brokers.pop(); }
            ConnectionFormField::ConsumerGroup => { s.consumer_group.pop(); }
            ConnectionFormField::Username => { s.username.pop(); }
            ConnectionFormField::Password => { s.password.pop(); }
            _ => return None,
        },
        _ => return None,
    }
    Some(Action::UpdateConnectionForm(s))
}

fn conn_next(f: &ConnectionFormField, auth: &AuthType) -> ConnectionFormField {
    match f {
        ConnectionFormField::Name => ConnectionFormField::Brokers,
        ConnectionFormField::Brokers => ConnectionFormField::ConsumerGroup,
        ConnectionFormField::ConsumerGroup => ConnectionFormField::AuthType,
        ConnectionFormField::AuthType => if auth.requires_credentials() { ConnectionFormField::Username } else { ConnectionFormField::Name },
        ConnectionFormField::Username => ConnectionFormField::Password,
        ConnectionFormField::Password => ConnectionFormField::Name,
    }
}

fn conn_prev(f: &ConnectionFormField, auth: &AuthType) -> ConnectionFormField {
    match f {
        ConnectionFormField::Name => if auth.requires_credentials() { ConnectionFormField::Password } else { ConnectionFormField::AuthType },
        ConnectionFormField::Brokers => ConnectionFormField::Name,
        ConnectionFormField::ConsumerGroup => ConnectionFormField::Brokers,
        ConnectionFormField::AuthType => ConnectionFormField::ConsumerGroup,
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

fn produce_form_key(key: KeyEvent, f: &ProduceFormState) -> Option<Action> {
    let mut s = f.clone();
    match key.code {
        KeyCode::Esc => return Some(Action::ModalCancel),
        KeyCode::Enter => return (!f.value.is_empty()).then_some(Action::ModalConfirm),
        KeyCode::Tab | KeyCode::Down | KeyCode::Up | KeyCode::BackTab => {
            s.focused_field = match f.focused_field {
                ProduceFormField::Key => ProduceFormField::Value,
                ProduceFormField::Value => ProduceFormField::Key,
            };
        }
        KeyCode::Char(c) => match f.focused_field {
            ProduceFormField::Key => s.key.push(c),
            ProduceFormField::Value => s.value.push(c),
        },
        KeyCode::Backspace => match f.focused_field {
            ProduceFormField::Key => { s.key.pop(); }
            ProduceFormField::Value => { s.value.pop(); }
        },
        _ => return None,
    }
    Some(Action::UpdateProduceForm(s))
}

fn add_partitions_form_key(key: KeyEvent, f: &AddPartitionsFormState) -> Option<Action> {
    let mut s = f.clone();
    match key.code {
        KeyCode::Esc => return Some(Action::ModalCancel),
        KeyCode::Enter => {
            let new_count: i32 = f.new_count.parse().unwrap_or(0);
            return (new_count > f.current_count).then_some(Action::ModalConfirm);
        }
        KeyCode::Char(c) if c.is_ascii_digit() => s.new_count.push(c),
        KeyCode::Backspace => { s.new_count.pop(); }
        _ => return None,
    }
    Some(Action::UpdateAddPartitionsForm(s))
}

fn alter_config_form_key(key: KeyEvent, f: &AlterConfigFormState) -> Option<Action> {
    let mut s = f.clone();

    if s.editing {
        match key.code {
            KeyCode::Enter => {
                if let Some((_, v, m)) = s.configs.get_mut(s.selected_index) {
                    *v = std::mem::take(&mut s.edit_value);
                    *m = true;
                }
                s.editing = false;
            }
            KeyCode::Esc => { s.editing = false; s.edit_value.clear(); }
            KeyCode::Char(c) => s.edit_value.push(c),
            KeyCode::Backspace => { s.edit_value.pop(); }
            _ => return None,
        }
    } else {
        match key.code {
            KeyCode::Esc => return Some(Action::ModalCancel),
            KeyCode::Enter => {
                if s.configs.iter().any(|(_, _, m)| *m) {
                    return Some(Action::ModalConfirm);
                }
                return None;
            }
            KeyCode::Up | KeyCode::Char('k') => s.selected_index = s.selected_index.saturating_sub(1),
            KeyCode::Down | KeyCode::Char('j') => {
                if s.selected_index + 1 < s.configs.len() { s.selected_index += 1; }
            }
            KeyCode::Char('e') => {
                if let Some((_, v, _)) = s.configs.get(s.selected_index) {
                    s.editing = true;
                    s.edit_value = v.clone();
                }
            }
            _ => return None,
        }
    }
    Some(Action::UpdateAlterConfigForm(s))
}

fn purge_topic_form_key(key: KeyEvent, f: &PurgeTopicFormState) -> Option<Action> {
    let mut s = f.clone();
    match key.code {
        KeyCode::Esc => return Some(Action::ModalCancel),
        KeyCode::Enter => return Some(Action::ModalConfirm),
        KeyCode::Tab | KeyCode::Up | KeyCode::Down | KeyCode::Char(' ') => s.purge_all = !s.purge_all,
        KeyCode::Char(c) if !s.purge_all && c.is_ascii_digit() => s.offset.push(c),
        KeyCode::Backspace if !s.purge_all => { s.offset.pop(); }
        _ => return None,
    }
    Some(Action::UpdatePurgeTopicForm(s))
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
            (KeyModifiers::NONE, KeyCode::Char('i')) => Some(Action::RequestViewTopicDetails),
            (KeyModifiers::NONE, KeyCode::Char('n')) => Some(Action::ShowModal(ModalType::TopicCreateForm(Default::default()))),
            (KeyModifiers::NONE, KeyCode::Char('/')) => Some(Action::ShowModal(ModalType::Input {
                title: "Filter".into(), placeholder: "".into(), value: String::new(), action: InputAction::FilterTopics,
            })),
            (KeyModifiers::CONTROL, KeyCode::Char('l')) => Some(Action::ClearTopicFilter),
            (KeyModifiers::CONTROL, KeyCode::Char('r')) | (_, KeyCode::F(5)) => Some(Action::FetchTopics),
            _ => None,
        },
        Screen::TopicDetails { topic_name } => {
            // Need access to state for partition count - handle 'p' in handler instead
            match key.code {
                KeyCode::Tab | KeyCode::Left | KeyCode::Char('h') | KeyCode::Right | KeyCode::Char('l') => Some(Action::SwitchTopicDetailTab),
                KeyCode::Char('m') => Some(Action::ViewTopicMessages(topic_name.clone())),
                KeyCode::Char('d') => Some(Action::ShowModal(ModalType::Confirm {
                    title: "Delete Topic".into(),
                    message: format!("Delete '{}'?", topic_name),
                    action: ConfirmAction::DeleteTopic(topic_name.clone()),
                })),
                // 'p' - add partitions (handled in handler with state access)
                // 'e' - edit config (handled in handler with state access)
                // 'x' - purge (handled in handler with state access)
                KeyCode::F(5) => Some(Action::ViewTopicDetails(topic_name.clone())),
                _ => None,
            }
        }
        Screen::Messages { topic_name } => match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Char('v') | KeyCode::Enter) => Some(Action::ToggleMessageDetail),
            (KeyModifiers::NONE, KeyCode::Char('p')) => Some(Action::ShowModal(ModalType::ProduceForm(ProduceFormState {
                topic: topic_name.clone(), ..Default::default()
            }))),
            (KeyModifiers::CONTROL, KeyCode::Char('r')) | (_, KeyCode::F(5)) => Some(Action::FetchMessages {
                topic: topic_name.clone(), offset_mode: OffsetMode::Latest, partition: None,
            }),
            (KeyModifiers::CONTROL, KeyCode::Char('l')) => Some(Action::ClearMessages),
            _ => None,
        },
        Screen::ConsumerGroups => match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Enter) => Some(Action::Select),
            (KeyModifiers::NONE, KeyCode::Char('/')) => Some(Action::ShowModal(ModalType::Input {
                title: "Filter".into(), placeholder: "".into(), value: String::new(), action: InputAction::FilterConsumerGroups,
            })),
            (KeyModifiers::CONTROL, KeyCode::Char('l')) => Some(Action::ClearConsumerGroupFilter),
            (KeyModifiers::CONTROL, KeyCode::Char('r')) | (_, KeyCode::F(5)) => Some(Action::FetchConsumerGroups),
            _ => None,
        },
        Screen::ConsumerGroupDetails { group_id } => match key.code {
            KeyCode::Tab | KeyCode::Left | KeyCode::Char('h') | KeyCode::Right | KeyCode::Char('l') => Some(Action::SwitchConsumerGroupDetailTab),
            KeyCode::F(5) => Some(Action::ViewConsumerGroupDetails(group_id.clone())),
            _ => None,
        },
        Screen::Brokers => match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('r')) | (_, KeyCode::F(5)) => Some(Action::FetchBrokers),
            _ => None,
        },
        Screen::Logs => match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Char('c')) => Some(Action::ClearLogs),
            (KeyModifiers::NONE, KeyCode::Char('f') | KeyCode::Char('/')) => Some(Action::CycleLogFilter),
            _ => None,
        },
    }
}

pub fn get_help_text(screen: &Screen) -> Vec<(&'static str, &'static str)> {
    let mut h = vec![("q", "Quit"), ("?", "Help"), ("Tab", "Switch"), ("Esc", "Back")];
    h.extend(match screen {
        Screen::Welcome => vec![("Enter", "Connect"), ("n", "New"), ("d", "Delete")],
        Screen::Topics => vec![("j/k", "Nav"), ("m", "Messages"), ("i", "Details"), ("n", "New"), ("/", "Filter")],
        Screen::Messages { .. } => vec![("j/k", "Nav"), ("v", "Detail"), ("p", "Produce"), ("F5", "Refresh")],
        Screen::ConsumerGroups => vec![("j/k", "Nav"), ("Enter", "Details"), ("/", "Filter"), ("F5", "Refresh")],
        Screen::TopicDetails { .. } => vec![("Tab", "Switch"), ("m", "Messages"), ("d", "Delete"), ("p", "Add Parts"), ("e", "Config"), ("x", "Purge")],
        Screen::ConsumerGroupDetails { .. } => vec![("Tab", "Switch"), ("F5", "Refresh")],
        Screen::Brokers => vec![("F5", "Refresh")],
        Screen::Logs => vec![("j/k", "Nav"), ("c", "Clear"), ("f", "Filter")],
    });
    h
}
