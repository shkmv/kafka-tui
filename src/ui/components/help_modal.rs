use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::app::state::Screen;
use crate::events::key_bindings::get_help_text;
use crate::ui::layout::centered_rect;
use crate::ui::theme::THEME;

pub struct HelpModal;

impl HelpModal {
    pub fn render(frame: &mut Frame, screen: &Screen) {
        let area = centered_rect(60, 70, frame.area());

        // Clear the background
        frame.render_widget(Clear, area);

        let block = Block::default()
            .title(" Help ")
            .title_style(THEME.header_style())
            .borders(Borders::ALL)
            .border_style(THEME.border_style(true))
            .style(THEME.modal_style());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Build help content
        let help_items = get_help_text(screen);
        let mut lines: Vec<Line> = vec![
            Line::from(vec![
                Span::styled("Keyboard Shortcuts", THEME.header_style()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Global:", THEME.title_style()),
            ]),
        ];

        // Add global shortcuts (first 5)
        for (key, desc) in help_items.iter().take(5) {
            lines.push(Line::from(vec![
                Span::styled(format!("  {:12}", key), THEME.key_hint_style()),
                Span::styled(*desc, THEME.normal_style()),
            ]));
        }

        // Add screen-specific shortcuts
        if help_items.len() > 5 {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled(format!("{} Screen:", screen), THEME.title_style()),
            ]));

            for (key, desc) in help_items.iter().skip(5) {
                lines.push(Line::from(vec![
                    Span::styled(format!("  {:12}", key), THEME.key_hint_style()),
                    Span::styled(*desc, THEME.normal_style()),
                ]));
            }
        }

        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Press ", THEME.muted_style()),
            Span::styled("Esc", THEME.key_hint_style()),
            Span::styled(" or ", THEME.muted_style()),
            Span::styled("?", THEME.key_hint_style()),
            Span::styled(" to close", THEME.muted_style()),
        ]));

        let help_text = Text::from(lines);
        let paragraph = Paragraph::new(help_text)
            .wrap(Wrap { trim: false });

        frame.render_widget(paragraph, inner);
    }
}
