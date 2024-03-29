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
use crate::model_names::{
    model::Model,
    ModelNames,
};

pub struct ModelNamesComponent {
    model_names: ModelNames,
    model_name_index_state: ListState,
    show_popup: bool,
}

impl ModelNamesComponent {
    pub fn new(application_root_path: String) -> Self {
        let model_names = ModelNames::new(application_root_path);
        let mut model_name_index_state = ListState::default();
        model_name_index_state.select(Some(0));
        Self {
            model_names,
            model_name_index_state,
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
                Span::from(format!("Model Names 0/0 | ")),
                Span::styled(format!("Filter: ''"), Style::default().fg(Color::Green)),
            ]
        );

        let p1 = Paragraph::new("Filter") // TARGET
            .block(Block::default().title(title_spans).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        let models = self.model_names.get_models_iter();
        let items: Vec<ListItem> = models
            .map(|node| Into::<ListItem>::into(node))
            .collect();

        let style_list = Style::default().fg(Color::White);
        let nodes_block:Block = Block::default()
            .borders(Borders::ALL)
            .style(style_list)
            .title(format!("Routes list"))
            .border_type(BorderType::Plain);

        let list = List::new(items).block(nodes_block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

        f.render_widget(p1, vertical_chunks[0]);
        f.render_stateful_widget(list, vertical_chunks[1], &mut self.model_name_index_state);
    }

}

impl Component for ModelNamesComponent {
    fn command_mode_event(&mut self, key_code: KeyCode) -> Result<String, String> {
        match key_code {
            KeyCode::Up => {
                // self.index_route = self.index_route.saturating_sub(1)
                if let Some(selected) = self.model_name_index_state.selected() {
                    self.model_name_index_state.select(Some(selected.saturating_sub(1)));
                    /*
                    if selected == 0 {
                        let selected = self.filtered_size.saturating_sub(1);
                        self.model_name_index_state.select(Some(selected));
                    } else {
                        self.model_name_index_state.select(Some(selected.saturating_sub(1)));
                    }
                    */
                }
            }
            KeyCode::Down => {
                if let Some(selected) = self.model_name_index_state.selected() {
                    self.model_name_index_state.select(Some(selected + 1));
                    /*
                    if selected >= (self.filtered_size.saturating_sub(1)) {
                        self.model_name_index_state.select(Some(0));
                    } else {
                        self.model_name_index_state.select(Some(selected + 1));
                    }
                    */
                }

                // self.index_route = self.index_route.saturating_add(1)
            }
            KeyCode::Enter => {
                if let Some(selected) = self.model_name_index_state.selected() {
                    let model_name: String = self.model_names.get_model_name(selected);
                    // let models: Vec<Model> = self.model_names.get_models_iter().collect();

                    return Ok(model_name)
                }
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
