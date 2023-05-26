use ratatui::{
    layout::{Alignment, Constraint, Direction, Rect, Layout},
    text::{Span, Spans},
    style::{Color, Style},
    terminal::Frame,
    widgets::{
        Block, Borders, Paragraph, Wrap
    },
    backend::Backend,
};

use crate::ui::Component;
use crate::routes::Routes;
// use crossterm::event::Event;
use crossterm::event::KeyCode;

pub struct RoutesComponent {
    paragraph_title: String,
    routes: Routes,
    index_route: usize,
}

impl RoutesComponent {
    // pub const fn new() -> Self {
    pub fn new(routes_path: &str) -> Self {
        let routes = Routes::new("http://localhost:3000", "routes.txt").unwrap();

        let paragraph_title = String::from("Routes title");
        Self {
            paragraph_title,
            routes,
            index_route: 0,
        }
    }

    pub fn render<B: Backend>(
        &self,
        f: &mut Frame<B>,
        // chunks: &mut Vec<Rect>,
        // rect: &mut Rect,
        rect: Rect,
        ) {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [Constraint::Length(8), Constraint::Min(5)].as_ref()
            )
            .split(rect);

        let index_route = self.index_route;
        let route_node = self.routes.get_node_route(index_route);

        // let p2 = Paragraph::new(String::from(text_2))
        let p1 = Paragraph::new(route_node) // TARGET
            .block(Block::default().title(format!("Details route {}/{}", index_route, self.routes.length)).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        let mut text = self.routes.get_original_lines_span();

        let p2 = Paragraph::new(text)
            .block(Block::default().title(format!("List routes")).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        f.render_widget(p1, vertical_chunks[0]);
        f.render_widget(p2, vertical_chunks[1]);

    }
}

impl Component for RoutesComponent {
    fn command_mode_event(&mut self, ev: KeyCode) -> Result<String, String> {
        Ok(String::from("ok"))
    }
	fn event(&mut self, key_code: KeyCode) -> Result<String, String> {
        match key_code {
            KeyCode::Up => { self.index_route = self.index_route.saturating_sub(1) }
            KeyCode::Down => { self.index_route = self.index_route.saturating_add(1) }
            KeyCode::Tab => {
                // TODO: Reset selected to zero to prevent bug when attempting to look at a
                // commit that there is not anymore
            }
            KeyCode::BackTab => {
            }
            _ => {}
        }
        Ok(String::from("ok"))
    }
}


