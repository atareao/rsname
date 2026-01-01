mod models; // Asegúrate de crear src/models/mod.rs y exportar app y cleaner

use std::io;
use crossterm::{event::{self, Event, KeyCode}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute};
use ratatui::{prelude::*, widgets::*};

use models::{
    App,
    AppMode,
};


fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        // ... dentro del loop en main ...
if let Event::Key(key) = event::read()? {
    match app.mode {
        AppMode::Reviewing => match key.code {
            KeyCode::Char('q') => break,
            KeyCode::Down | KeyCode::Char('j') => app.next(),
            KeyCode::Up | KeyCode::Char('k') => app.previous(),
            KeyCode::Char(' ') => app.toggle_selection(),
            KeyCode::Enter => app.confirm_rename(), // Pasa a Confirming
            KeyCode::Char('r') => app.scan_current_dir(),
            _ => {}
        },
        AppMode::Confirming => match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => app.run_rename(),
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => app.cancel_confirm(),
            _ => {}
        },
        _ => {}
    }
}
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let rects = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Título
    f.render_widget(Paragraph::new(" rsname - Rename Tool ").block(Block::default().borders(Borders::ALL)), rects[0]);

    // Tabla
    let header = Row::new(vec![" ", "Nombre Actual", "Nuevo Nombre"])
        .style(Style::default().fg(Color::Yellow));
    
    let rows: Vec<Row> = app.items.iter().enumerate().map(|(i, item)| {
        let check = if item.selected { "[x]" } else { "[ ]" };
        let style = if i == app.selected_index { Style::default().bg(Color::DarkGray) } else { Style::default() };
        Row::new(vec![check, item.old_name.as_str(), item.new_name.as_str()]).style(style)
    }).collect();

    let table = Table::new(rows, [Constraint::Length(4), Constraint::Percentage(45), Constraint::Percentage(45)])
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" Archivos "));
    
    f.render_widget(table, rects[1]);

    // Ayuda
    f.render_widget(Paragraph::new(" q: Salir | j/k: Mover | Espacio: Seleccionar | Enter: Renombrar | r: Recargar"), rects[2]);
}
