use ratatui::{
    layout::{Alignment, Constraint, Direction, Rect, Layout},
    text::{Span, Spans},
    style::{Color, Style, Modifier},
    terminal::Frame,
    widgets::{
        Block, BorderType, Borders, Paragraph, Wrap, Clear, ListItem, List, ListState,
    },
    backend::Backend,
};

use crossterm::event::KeyCode;

use crate::ui::Component;

pub struct AssociationsComponent {
}

impl AssociationsComponent {
    pub fn new() -> Self {
        Self {}
    }
    pub fn render<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        rect: Rect,
        ) {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [Constraint::Length(8), Constraint::Min(5)].as_ref()
            )
            .split(rect);

        let title_spans = Spans::from(
            vec![
                Span::from(format!("Associations 0/0 | ")),
                Span::styled(format!("Filter: ''"), Style::default().fg(Color::Green)),
            ]
        );

        let p1 = Paragraph::new("Associations") // TARGET
            .block(Block::default().title(title_spans).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        let p2 = Paragraph::new("bla bla")
            .block(Block::default().title(format!("List routes")).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(p1, vertical_chunks[0]);
        f.render_widget(p2, vertical_chunks[1]);
    }

}

impl Component for AssociationsComponent {
    fn command_mode_event(&mut self, key_code: KeyCode) -> Result<String, String> {
        match key_code {
            _ => {}
        }
        Ok(String::from("ok"))
    }
	fn event(&mut self, key_code: KeyCode) -> Result<String, String> {
        match key_code {
            _ => {}
        }
        Ok(String::from("ok"))
    }
}
