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


pub struct ModelNodeComponent {
    model_name: String,
    show_popup: bool,
}

impl ModelNodeComponent {
    pub fn new(model_name: String) -> Self {
        Self {
            show_popup: false,
            model_name,
        }
    }
    pub fn render<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        rect: Rect,
        ) {
        let title_spans = Spans::from(
            vec![
                Span::from(format!("Model Nodes 0/0 | ")),
                Span::styled(format!("Filter: ''"), Style::default().fg(Color::Green)),
            ]
        );

        let p1 = Paragraph::new(self.model_name.clone()) // TARGET
            .block(Block::default().title(title_spans).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        f.render_widget(p1, rect);
    }

}

