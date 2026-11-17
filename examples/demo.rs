//! Interactive showcase for the shared MamboUI components.

use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use mambo_ui::ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    widgets::{Row, TableState, Widget},
};
use mambo_ui::{
    EmptyState, Help, KeyHint, Panel, Scheme, Shell, Status, StatusLine, Theme, responsive_columns,
    selection_table,
};

fn main() -> io::Result<()> {
    let mut terminal = mambo_ui::ratatui::init();
    let result = run(&mut terminal);
    mambo_ui::ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut scheme = Scheme::Dark;
    loop {
        terminal.draw(|frame| draw(frame, scheme))?;
        if !event::poll(Duration::from_millis(250))? {
            continue;
        }
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
            KeyCode::Char('t') => {
                scheme = match scheme {
                    Scheme::Dark => Scheme::Light,
                    Scheme::Light => Scheme::Dark,
                };
            }
            _ => {}
        }
    }
}

fn draw(frame: &mut Frame<'_>, scheme: Scheme) {
    let theme = Theme::new(scheme);
    let content = Shell::new("MamboUI")
        .subtitle("shared terminal components")
        .footer("Project Mambo")
        .render(frame, &theme);
    let [main, status, help] = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(content);
    let [packages, details] = responsive_columns(main, 28);

    let package_panel = Panel::new("Packages").focused(true).block(&theme);
    let package_content = package_panel.inner(packages);
    frame.render_widget(package_panel, packages);
    let rows = [
        Row::new(["MamboTools", "ready"]),
        Row::new(["MamboFinance", "active"]),
        Row::new(["MamboUI", "v0.2"]),
    ];
    let mut state = TableState::default().with_selected(0);
    frame.render_stateful_widget(
        selection_table(rows, [Constraint::Length(16), Constraint::Min(8)], &theme)
            .header(Row::new(["Application", "State"]).style(theme.title())),
        package_content,
        &mut state,
    );

    let details_panel = Panel::new("Updates").block(&theme);
    let details_content = details_panel.inner(details);
    frame.render_widget(details_panel, details);
    EmptyState::new("Everything is current", &theme)
        .message("New releases will appear here")
        .render(details_content, frame.buffer_mut());

    frame.render_widget(
        StatusLine::new(Status::Success, "3 packages ready", &theme),
        status,
    );
    frame.render_widget(
        Help::new(
            [
                KeyHint::new("↑/↓", "select"),
                KeyHint::new("Enter", "open"),
                KeyHint::new("t", "theme"),
                KeyHint::new("q", "quit"),
            ],
            &theme,
        ),
        help,
    );
}
