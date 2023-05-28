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

use crate::ui::{centered_rect, Component};
use crate::associations::{
    model::Model,
    Associations,
};

pub struct AssociationsComponent {
    associations: Associations,
    show_popup: bool,
}

impl AssociationsComponent {
    pub fn new(application_root_path: String) -> Self {
        let associations = Associations::new(application_root_path);
        Self {
            associations,
            show_popup: false
        }
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

        let models = self.associations.get_models_iter();

        let models_string: Vec<String> = models.map(|m| m.into()).collect();
        let models_spans: Vec<Spans> = models_string
            .into_iter()
            .map(|m| Spans::from(m))
            .collect::<Vec<Spans>>();

        let models = self.associations.get_models_iter();
        // let model = models.collect::<Vec<&Model>>().get(0).unwrap();
        let binding = models.collect::<Vec<&Model>>();
        let model = binding.get(18).unwrap();

        // let p1 = Paragraph::new(Into::<String>::into(model)) // TARGET
        let p1 = Paragraph::new(model.display_string()) // TARGET
            .block(Block::default().title(title_spans).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        // let p2 = Paragraph::new(models_string.iter().map(|m| Spans::from(Span::from(*m))).collect::<Spans>())
        // let p2 = Paragraph::new("bla bla")
        let p2 = Paragraph::new(models_spans)
            .block(Block::default().title(format!("List routes")).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(p1, vertical_chunks[0]);
        f.render_widget(p2, vertical_chunks[1]);

        if self.show_popup {
            let popup = Paragraph::new(model.parse_and_display().unwrap())
                .block(Block::default().title("Association Details").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            let size = f.size();

            let area = centered_rect(60, 20, size);
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(popup, area);
        }
    }

}

impl Component for AssociationsComponent {
    fn command_mode_event(&mut self, key_code: KeyCode) -> Result<String, String> {
        match key_code {
            KeyCode::Enter => {
                self.show_popup = !self.show_popup
            }
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
