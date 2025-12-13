use ratatui::prelude::*;

#[derive(Debug, Clone)]
pub struct AppLayout {
    pub header: Rect,
    pub sidebar: Rect,
    pub content: Rect,
    pub status: Rect,
}

impl AppLayout {
    pub fn new(area: Rect) -> Self {
        let v = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(1),
        ]).split(area);

        let h = Layout::horizontal([
            Constraint::Length(22),
            Constraint::Min(40),
        ]).split(v[1]);

        Self { header: v[0], sidebar: h[0], content: h[1], status: v[2] }
    }
}

#[derive(Debug, Clone)]
pub struct WelcomeLayout {
    pub header: Rect,
    pub content: Rect,
    pub status: Rect,
}

pub fn welcome_layout(area: Rect) -> WelcomeLayout {
    let v = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Length(1),
    ]).split(area);

    let h = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(60),
        Constraint::Percentage(20),
    ]).split(v[1]);

    WelcomeLayout { header: v[0], content: h[1], status: v[2] }
}

#[derive(Debug, Clone)]
pub struct MessagesLayout {
    pub toolbar: Rect,
    pub list: Rect,
    pub detail: Rect,
}

pub fn messages_layout(area: Rect) -> MessagesLayout {
    let v = Layout::vertical([
        Constraint::Length(3),
        Constraint::Percentage(50),
        Constraint::Min(10),
    ]).split(area);

    MessagesLayout { toolbar: v[0], list: v[1], detail: v[2] }
}

pub fn messages_layout_collapsed(area: Rect) -> MessagesLayout {
    let v = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
    ]).split(area);

    MessagesLayout { toolbar: v[0], list: v[1], detail: Rect::default() }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let v = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ]).split(area);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ]).split(v[1])[1]
}

pub fn centered_rect_fixed(width: u16, height: u16, area: Rect) -> Rect {
    Rect::new(
        area.x + (area.width.saturating_sub(width)) / 2,
        area.y + (area.height.saturating_sub(height)) / 2,
        width.min(area.width),
        height.min(area.height),
    )
}

#[derive(Debug, Clone)]
pub struct TopicsLayout {
    pub toolbar: Rect,
    pub list: Rect,
    pub details: Rect,
}

pub fn topics_list_layout(area: Rect) -> TopicsLayout {
    let v = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
    ]).split(area);

    let h = Layout::horizontal([
        Constraint::Percentage(60),
        Constraint::Percentage(40),
    ]).split(v[1]);

    TopicsLayout { toolbar: v[0], list: h[0], details: h[1] }
}

#[derive(Debug, Clone)]
pub struct ConsumerGroupsLayout {
    pub toolbar: Rect,
    pub list: Rect,
}

pub fn consumer_groups_layout(area: Rect) -> ConsumerGroupsLayout {
    let v = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
    ]).split(area);

    ConsumerGroupsLayout { toolbar: v[0], list: v[1] }
}
