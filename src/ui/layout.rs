use ratatui::prelude::*;

/// Main application layout structure
///
/// ```text
/// +------------------------------------------+
/// |              Header Bar                   |
/// +--------+---------------------------------+
/// |        |                                 |
/// | Side   |         Main Content            |
/// | bar    |                                 |
/// |        |                                 |
/// +--------+---------------------------------+
/// |              Status Bar                   |
/// +------------------------------------------+
/// ```
#[derive(Debug, Clone)]
pub struct AppLayout {
    pub header: Rect,
    pub sidebar: Rect,
    pub content: Rect,
    pub status: Rect,
}

impl AppLayout {
    pub fn new(area: Rect) -> Self {
        // Vertical split: header (3), main (rest), status (1)
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Main area
                Constraint::Length(1), // Status bar
            ])
            .split(area);

        // Horizontal split for main area: sidebar (fixed 20), content (rest)
        let sidebar_width = 22;
        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(sidebar_width),
                Constraint::Min(40),
            ])
            .split(vertical[1]);

        Self {
            header: vertical[0],
            sidebar: horizontal[0],
            content: horizontal[1],
            status: vertical[2],
        }
    }
}

/// Layout for welcome screen (no sidebar)
pub fn welcome_layout(area: Rect) -> WelcomeLayout {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Content
            Constraint::Length(1), // Status
        ])
        .split(area);

    // Center the content
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(vertical[1]);

    WelcomeLayout {
        header: vertical[0],
        content: horizontal[1],
        status: vertical[2],
    }
}

#[derive(Debug, Clone)]
pub struct WelcomeLayout {
    pub header: Rect,
    pub content: Rect,
    pub status: Rect,
}

/// Content area layouts for Topics screen
pub fn topics_list_layout(area: Rect) -> TopicsLayout {
    // Vertical: toolbar (3), list+details
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Toolbar
            Constraint::Min(10),   // List + Details
        ])
        .split(area);

    // Horizontal: list (60%), details panel (40%)
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(vertical[1]);

    TopicsLayout {
        toolbar: vertical[0],
        list: horizontal[0],
        details: horizontal[1],
    }
}

#[derive(Debug, Clone)]
pub struct TopicsLayout {
    pub toolbar: Rect,
    pub list: Rect,
    pub details: Rect,
}

/// Content area layout for Messages screen
pub fn messages_layout(area: Rect) -> MessagesLayout {
    // Vertical split: toolbar (3), list (variable), detail (variable)
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Toolbar
            Constraint::Percentage(50), // Message list
            Constraint::Min(10),    // Message detail
        ])
        .split(area);

    MessagesLayout {
        toolbar: vertical[0],
        list: vertical[1],
        detail: vertical[2],
    }
}

#[derive(Debug, Clone)]
pub struct MessagesLayout {
    pub toolbar: Rect,
    pub list: Rect,
    pub detail: Rect,
}

/// Layout for messages screen with collapsed detail
pub fn messages_layout_collapsed(area: Rect) -> MessagesLayout {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Toolbar
            Constraint::Min(10),   // Message list (full)
        ])
        .split(area);

    MessagesLayout {
        toolbar: vertical[0],
        list: vertical[1],
        detail: Rect::default(),
    }
}

/// Content area layout for Consumer Groups screen
pub fn consumer_groups_layout(area: Rect) -> ConsumerGroupsLayout {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Toolbar
            Constraint::Min(10),   // List
        ])
        .split(area);

    ConsumerGroupsLayout {
        toolbar: vertical[0],
        list: vertical[1],
    }
}

#[derive(Debug, Clone)]
pub struct ConsumerGroupsLayout {
    pub toolbar: Rect,
    pub list: Rect,
}

/// Layout for Consumer Group Details screen
pub fn consumer_group_detail_layout(area: Rect) -> ConsumerGroupDetailLayout {
    // Vertical: info header (5), members/offsets
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Info header
            Constraint::Min(10),   // Members + Offsets
        ])
        .split(area);

    // Horizontal split: members (50%), offsets (50%)
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(vertical[1]);

    ConsumerGroupDetailLayout {
        info: vertical[0],
        members: horizontal[0],
        offsets: horizontal[1],
    }
}

#[derive(Debug, Clone)]
pub struct ConsumerGroupDetailLayout {
    pub info: Rect,
    pub members: Rect,
    pub offsets: Rect,
}

/// Layout for Topic Details screen
pub fn topic_detail_layout(area: Rect) -> TopicDetailLayout {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // Topic info
            Constraint::Length(3),  // Tab bar
            Constraint::Min(10),    // Tab content (partitions/config)
        ])
        .split(area);

    TopicDetailLayout {
        info: vertical[0],
        tabs: vertical[1],
        content: vertical[2],
    }
}

#[derive(Debug, Clone)]
pub struct TopicDetailLayout {
    pub info: Rect,
    pub tabs: Rect,
    pub content: Rect,
}

/// Calculate centered rect for modals
pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Calculate fixed-size centered rect for modals
pub fn centered_rect_fixed(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    Rect::new(
        x,
        y,
        width.min(area.width),
        height.min(area.height),
    )
}

/// Create a rect with margin
pub fn with_margin(area: Rect, margin: u16) -> Rect {
    Rect::new(
        area.x + margin,
        area.y + margin,
        area.width.saturating_sub(margin * 2),
        area.height.saturating_sub(margin * 2),
    )
}
