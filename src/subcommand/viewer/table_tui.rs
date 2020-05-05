mod event;

use std::io;

use anyhow::Result;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Row, Table, Widget};
use tui::Terminal;

use self::event::{Event, Events};

struct App {
    items: prettytable::Table,
    selected: usize,
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

    let mut app = App {
        items: table.clone(),
        selected: 0,
    };

    // Input
    loop {
        terminal.draw(|mut f| {
            let selected_style = Style::default().fg(Color::Yellow).modifier(Modifier::BOLD);
            let normal_style = Style::default().fg(Color::White);
            let rows = app.items.row_iter().enumerate().map(|(i, item)| {
                let iter = item
                    .iter()
                    .map(|m| m.get_content())
                    .collect::<Vec<_>>()
                    .into_iter();
                if i == app.selected {
                    Row::StyledData(iter, selected_style)
                } else {
                    Row::StyledData(iter, normal_style)
                }
            });

            let rects = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .margin(5)
                .split(f.size());
            Table::new(header.into_iter(), rows)
                .block(Block::default().borders(Borders::ALL).title("Table"))
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Length(30),
                    Constraint::Max(10),
                ])
                .render(&mut f, rects[0]);
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
                    app.selected += 1;
                    if app.selected > app.items.len() - 1 {
                        app.selected = 0;
                    }
                }
                Key::Up => {
                    if app.selected > 0 {
                        app.selected -= 1;
                    } else {
                        app.selected = app.items.len() - 1;
                    }
                }
                _ => {}
            },
            _ => {}
        };
    }

    Ok(())
}
