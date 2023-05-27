use ratatui::{
    layout::{Alignment, Constraint, Direction, Rect, Layout},
    text::{Span, Spans},
    style::{Color, Style},
    terminal::Frame,
    widgets::{
        Block, Borders, Paragraph, Wrap, Clear
    },
    backend::Backend,
};

use crate::ui::Component;
use crate::routes::Routes;
// use crossterm::event::Event;
use crossterm::event::KeyCode;

pub struct RoutesComponent {
    paragraph_title: String,
    routes: Result<Routes, Box<dyn std::error::Error>>,
    index_route: usize,
    filter_string: String,
    show_popup: bool,
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

impl RoutesComponent {
    // pub const fn new() -> Self {
    // pub fn new(routes_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
    //     let routes = Routes::new("http://localhost:3000", "routes.txt")?;
    pub fn new(routes_path: &str) -> Self {
        let routes = Routes::new("http://localhost:3000", "routes.txt");

        let paragraph_title = String::from("Routes title");
        Self {
            paragraph_title,
            routes,
            index_route: 0,
            filter_string: String::new(),
            show_popup: false,
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
        match &self.routes {
            Ok(routes) => {

                let route_node = routes.get_route_node(index_route, &self.filter_string);
                match route_node {
                    Some(route_node) => {
                        let text = routes.get_original_lines_span(&self.filter_string);

                        // let p2 = Paragraph::new(String::from(text_2))
                        let p1 = Paragraph::new(Into::<String>::into(route_node)) // TARGET
                        // let p1 = Paragraph::new(route_node.into::<String>()) // TARGET
                            .block(Block::default().title(format!("Details route {}/{} | Filter: '{}'", index_route, text.len(), self.filter_string)).borders(Borders::ALL))
                            .style(Style::default().fg(Color::White).bg(Color::Black))
                            .alignment(Alignment::Left)
                            .wrap(Wrap { trim: true });


                        let p2 = Paragraph::new(text)
                            .block(Block::default().title(format!("List routes")).borders(Borders::ALL))
                            .style(Style::default().fg(Color::White).bg(Color::Black))
                            .alignment(Alignment::Left)
                            .wrap(Wrap { trim: true });

                        f.render_widget(p1, vertical_chunks[0]);
                        f.render_widget(p2, vertical_chunks[1]);

                        if self.show_popup {
                            // let popup = Paragraph::new(route_node.into::<Spans>()) // TARGET
                            // let popup = Paragraph::new(Into::<Spans>::into(route_node)) // TARGET
                            let popup = Paragraph::new(Into::<Vec<Spans>>::into(route_node)) // TARGET
                                .block(Block::default().title("Route Details").borders(Borders::ALL))
                                .style(Style::default().fg(Color::White).bg(Color::Black))
                                .alignment(Alignment::Left)
                                .wrap(Wrap { trim: true });
                            let size = f.size();

                            let block = Block::default().title("Popup").borders(Borders::ALL);
                            let area = centered_rect(60, 20, size);
                            f.render_widget(Clear, area); //this clears out the background
                            // f.render_widget(block, area);
                            f.render_widget(popup, area);
                        }
                    }
                    None => {
                        let text = routes.get_original_lines_span(&self.filter_string);
                        let p1 = Paragraph::new("FAILED TO LOAD SELECTED NODE") // TARGET
                        // let p1 = Paragraph::new(route_node.into::<String>()) // TARGET
                            .block(Block::default().title(format!("Details route {}/{} | Filter: '{}'", index_route, text.len(), self.filter_string)).borders(Borders::ALL))
                            .style(Style::default().fg(Color::White).bg(Color::Black))
                            .alignment(Alignment::Left)
                            .wrap(Wrap { trim: true });
                        let p2 = Paragraph::new(text)
                            .block(Block::default().title(format!("List routes")).borders(Borders::ALL))
                            .style(Style::default().fg(Color::White).bg(Color::Black))
                            .alignment(Alignment::Left)
                            .wrap(Wrap { trim: true });
                        f.render_widget(p1, vertical_chunks[0]);
                        f.render_widget(p2, vertical_chunks[1]);

                    }
                }
            },
            Err(e) => {
                // let p2 = Paragraph::new(String::from(text_2))
                let p1 = Paragraph::new("Unable to load Routes module. It is probably missing the routes.txt file.") // TARGET
                    .block(Block::default().title("ROUTES ERROR").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });


                let p2 = Paragraph::new(format!("Details: {}", e))
                    .block(Block::default().title(format!("List routes")).borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });

                f.render_widget(p1, vertical_chunks[0]);
                f.render_widget(p2, vertical_chunks[1]);
            }
        };

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
            KeyCode::Right => { self.show_popup = true }
            KeyCode::Left => { self.show_popup = false }
            KeyCode::Tab => {
                // TODO: Reset selected to zero to prevent bug when attempting to look at a
                // commit that there is not anymore
            }
            KeyCode::Char(c) => {
                self.filter_string.push(c);
            },
            KeyCode::Backspace => {
                self.filter_string.pop();
            }
            _ => {}
        }
        Ok(String::from("ok"))
    }
}


