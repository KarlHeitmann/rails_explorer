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

use crate::ui::{centered_rect, Component};
use crate::routes::route_node::RouteNode;
use crate::routes::Routes;
// use crossterm::event::Event;
use crossterm::event::KeyCode;

pub struct RoutesComponent {
    route_index_state: ListState,
    filtered_size: usize,
    paragraph_title: String,
    routes: Result<Routes, Box<dyn std::error::Error>>,
    filter_string: String,
    show_popup: bool,
}

impl RoutesComponent {
    pub fn new(routes_path: &str) -> Self {
        let routes = Routes::new("http://localhost:3000", "routes.txt");
        let mut route_index_state = ListState::default();
        route_index_state.select(Some(0));
        let paragraph_title = String::from("Routes title");
        Self {
            paragraph_title,
            routes,
            route_index_state,
            filtered_size: 0,
            filter_string: String::new(),
            show_popup: false,
        }
    }

    pub fn render<B: Backend>(
        &mut self,
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

        let index_route = self.route_index_state.selected().unwrap();
        match &self.routes {
            Ok(routes) => {

                let rs: Vec<&RouteNode> = routes.route_nodes
                    .iter()
                    .filter(|route_node| route_node.uri_pattern.contains(&self.filter_string))
                    // .filter(|route_node| route_node.uri_pattern.contains(self.filter_string))
                    .collect();
                let items: Vec<ListItem> = rs
                    .iter()
                    // .map(|node| *node.into())
                    .map(|node| Into::<ListItem>::into(*node))
                    .collect();

                self.filtered_size = items.len();

                let route_node = rs.get(index_route);
                match route_node {
                    Some(route_node) => {
                        let title_spans = Spans::from(
                            vec![
                                Span::from(format!("Details route {}/{} | ", index_route, self.filtered_size)),
                                Span::styled(format!("Filter: '{}'", self.filter_string), Style::default().fg(Color::Green)),
                            ]
                        );

                        let p1 = Paragraph::new(Into::<String>::into(*route_node)) // TARGET
                            .block(Block::default().title(title_spans).borders(Borders::ALL))
                            .style(Style::default().fg(Color::White).bg(Color::Black))
                            .alignment(Alignment::Left)
                            .wrap(Wrap { trim: true });

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
                        f.render_stateful_widget(list, vertical_chunks[1], &mut self.route_index_state);


                        if self.show_popup {
                            let popup = Paragraph::new(Into::<Vec<Spans>>::into(*route_node)) // TARGET
                                .block(Block::default().title("Route Details").borders(Borders::ALL))
                                .style(Style::default().fg(Color::White).bg(Color::Black))
                                .alignment(Alignment::Left)
                                .wrap(Wrap { trim: true });
                            let size = f.size();

                            let block = Block::default().title("Popup").borders(Borders::ALL);
                            let area = centered_rect(60, 20, size);
                            f.render_widget(Clear, area); //this clears out the background
                            f.render_widget(popup, area);
                        }
                    }
                    None => {
                        let text = routes.get_original_lines_span(&self.filter_string);
                        let title_spans = Spans::from(
                            vec![
                                Span::from(format!("Details route {}/{} | ", index_route, self.filtered_size)),
                                Span::styled(format!("Filter: '{}'", self.filter_string), Style::default().fg(Color::Red)),
                            ]
                        );
                        let p1 = Paragraph::new("FAILED TO LOAD SELECTED NODE") // TARGET
                            .block(Block::default().title(title_spans).borders(Borders::ALL))
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
    fn command_mode_event(&mut self, key_code: KeyCode) -> Result<String, String> {
        match key_code {
            KeyCode::Up => {
                // self.index_route = self.index_route.saturating_sub(1)
                if let Some(selected) = self.route_index_state.selected() {
                    if selected == 0 {
                        let selected = self.filtered_size.saturating_sub(1);
                        self.route_index_state.select(Some(selected));
                    } else {
                        self.route_index_state.select(Some(selected.saturating_sub(1)));
                    }
                }
            }
            KeyCode::Down => {
                if let Some(selected) = self.route_index_state.selected() {
                    if selected >= (self.filtered_size.saturating_sub(1)) {
                        self.route_index_state.select(Some(0));
                    } else {
                        self.route_index_state.select(Some(selected + 1));
                    }
                }

                // self.index_route = self.index_route.saturating_add(1)
            }
            KeyCode::Right => { self.show_popup = true }
            KeyCode::Left => { self.show_popup = false }
            _ => {}
        }
        Ok(String::from("ok"))
    }
	fn event(&mut self, key_code: KeyCode) -> Result<String, String> {
        match key_code {
            KeyCode::Up => {
                // self.index_route = self.index_route.saturating_sub(1)
                if let Some(selected) = self.route_index_state.selected() {
                    if selected == 0 {
                        let selected = self.filtered_size.saturating_sub(1);
                        self.route_index_state.select(Some(selected));
                    } else {
                        self.route_index_state.select(Some(selected.saturating_sub(1)));
                    }
                }
            }
            KeyCode::Down => {
                if let Some(selected) = self.route_index_state.selected() {
                    if selected >= (self.filtered_size.saturating_sub(1)) {
                        self.route_index_state.select(Some(0));
                    } else {
                        self.route_index_state.select(Some(selected + 1));
                    }
                }

                // self.index_route = self.index_route.saturating_add(1)
            }
            KeyCode::Tab => {
                // TODO: Reset selected to zero to prevent bug when attempting to look at a
                // commit that there is not anymore
            }
            KeyCode::Char(c) => {
                // TODO: move the filter code to here. Is it possible? only filter route_nodes when a key applied to filter is hit? so it doesn't filter every time it renders this page
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


