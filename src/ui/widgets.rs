use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::theme::THEME;

/// Formats a text input field with cursor and placeholder support.
///
/// - If `value` is empty and `focused` is true, shows a cursor block
/// - If `focused` is true, appends a cursor block to the value
/// - If `value` is empty and not focused, shows the placeholder
/// - Otherwise, shows the value as-is
pub fn format_input(value: &str, focused: bool, placeholder: &str) -> String {
    if value.is_empty() && focused {
        String::from("█")
    } else if focused {
        format!("{}█", value)
    } else if value.is_empty() {
        String::from(placeholder)
    } else {
        value.to_string()
    }
}

/// Returns the appropriate style for a label based on focus state.
pub fn label_style(focused: bool) -> Style {
    if focused {
        THEME.title_style()
    } else {
        THEME.muted_style()
    }
}

/// Renders a labeled input field.
///
/// This is a convenience function that renders both a label and an input field
/// in their respective areas.
pub fn render_labeled_input(
    frame: &mut Frame,
    label_area: Rect,
    input_area: Rect,
    label_text: &str,
    value: &str,
    placeholder: &str,
    focused: bool,
) {
    let label = Paragraph::new(label_text).style(label_style(focused));
    frame.render_widget(label, label_area);

    let display = format_input(value, focused, placeholder);
    let input = Paragraph::new(display).style(THEME.input_style(focused));
    frame.render_widget(input, input_area);
}

/// Creates a standard modal block with consistent styling.
pub fn modal_block(title: &str) -> Block<'_> {
    Block::default()
        .title(format!(" {} ", title))
        .title_style(THEME.header_style())
        .borders(Borders::ALL)
        .border_style(THEME.border_style(true))
        .style(THEME.modal_style())
}

/// Creates a standard content block with consistent styling.
pub fn content_block(title: &str, focused: bool) -> Block<'_> {
    Block::default()
        .title(format!(" {} ", title))
        .title_style(THEME.header_style())
        .borders(Borders::ALL)
        .border_style(THEME.border_style(focused))
}

/// Renders a loading state within the given area.
pub fn render_loading(frame: &mut Frame, area: Rect, message: &str) {
    let loading = Paragraph::new(message)
        .style(THEME.loading_style())
        .alignment(Alignment::Center);
    frame.render_widget(loading, area);
}

/// Renders an empty state within the given area.
pub fn render_empty(frame: &mut Frame, area: Rect, message: &str) {
    let empty = Paragraph::new(message)
        .style(THEME.muted_style())
        .alignment(Alignment::Center);
    frame.render_widget(empty, area);
}
