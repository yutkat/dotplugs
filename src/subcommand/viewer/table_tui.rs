mod event;

use std::io;

use self::event::{Event, Events};
use anyhow::Result;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, TableState},
    Terminal,
};

struct StatefulTable {
    state: TableState,
    items: prettytable::Table,
}

impl StatefulTable {
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub fn display(header: &Vec<&str>, table: &prettytable::Table) -> Result<()> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut table = StatefulTable {
        state: TableState::default(),
        items: table.clone(),
    };

    // Input
    loop {
        terminal.draw(|mut f| {
            let rects = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .margin(5)
                .split(f.size());

            let selected_style = Style::default().fg(Color::Yellow).modifier(Modifier::BOLD);
            let normal_style = Style::default().fg(Color::White);

            let rows = table.items.row_iter().enumerate().map(|(_, item)| {
                let iter = item
                    .iter()
                    .map(|m| m.get_content())
                    .collect::<Vec<_>>()
                    .into_iter();
                Row::StyledData(iter, normal_style)
            });
            let t = Table::new(header.iter(), rows)
                .block(Block::default().borders(Borders::ALL).title("Table"))
                .highlight_style(selected_style)
                .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Length(30),
                    Constraint::Max(10),
                ]);
            f.render_stateful_widget(t, rects[0], &mut table.state);
        })?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => {
                    break;
                }
                Key::Esc => {
                    break;
                }
                Key::Down => {
                    table.next();
                }
                Key::Up => {
                    table.previous();
                }
                _ => {}
            },
            _ => {}
        };
    }

    Ok(())
}
