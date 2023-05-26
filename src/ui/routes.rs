#![allow(unused)]  // FIXME

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
// use crossterm::event::Event;
use crossterm::event::KeyCode;

pub struct RoutesComponent {
    paragraph_title: String,
}

impl RoutesComponent {
    // pub const fn new() -> Self {
    pub fn new() -> Self {
        let paragraph_title = String::from("Routes title");
        Self {
            paragraph_title,
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
                [Constraint::Length(5), Constraint::Min(5)].as_ref()
            )
            .split(rect);

        // let text = Spans::from(git_explorer.branches_strings());
        let mut text = vec![Spans::from("CHANGE ME")];
        
        let head = String::from("HEAD CHANGE ME");

        text.push(
            Spans::from(vec![
                Span::styled(format!("HEAD: CHANGE MY HEAD"), Style::default().fg(Color::White))
            ])
        );
        text.push(
            Spans::from(vec![
                Span::styled(format!("oid: CHANGE MY OID"), Style::default().fg(Color::White))
            ])
        );

        let p1 = Paragraph::new(text)
            .block(Block::default().title(format!("Commit COMPLETE")).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        // .shorthand().unwrap();
        // let text_2 = repo.head().unwrap().peel_to_commit().unwrap();

        let parsed_diff = String::from("PARSED DIFF CHANGE ME");


        // let p2 = Paragraph::new(String::from(text_2))
        let p2 = Paragraph::new(parsed_diff)
            .block(Block::default().title(format!("Commit COMPLETE")).borders(Borders::ALL))
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


