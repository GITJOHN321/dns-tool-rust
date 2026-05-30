mod dns;
mod addresses;
mod ptr;
mod ping;
use dns::lookup_domain;
use std::io;
use ratatui::text::{Line, Span};
use ratatui::style::{Style, Color, Modifier};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    // Texto actual del input
    let mut input = String::new();

    // Texto enviado con Enter
    let mut output_text = String::new();

    loop {
        terminal.draw(|frame| {
            let size = frame.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                ])
                .split(size);

            // Cursor visual
            let input_display = format!("{}|", input);

            let input_widget = Paragraph::new(input_display)
                .block(
                    Block::default()
                        .title("Input")
                        .borders(Borders::ALL),
                );

            let output_widget = Paragraph::new(output_text.as_str())
                .block(
                    Block::default()
                        .title("Output")
                );

            frame.render_widget(input_widget, chunks[0]);

            frame.render_widget(output_widget, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {

                // Escribir caracteres
                KeyCode::Char(c) => {
                    input.push(c);
                }

                // Borrar caracteres
                KeyCode::Backspace => {
                    input.pop();
                }

                // Enter envía contenido
                KeyCode::Enter => {
                    output_text = lookup_domain(&input);

                    // Limpiar input
                    input.clear();
                }

                // Salir
                KeyCode::Esc => {
                    break;
                }

                _ => {}
            }
        }
    }

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    terminal.show_cursor()?;

    Ok(())
}