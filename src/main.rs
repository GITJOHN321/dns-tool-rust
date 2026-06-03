use std::io;
mod ui;
mod models;
mod services;
mod controllers;
mod utils;

use crate::ui::matrix_table::render_matrix_table;
use crate::ui::ns_table::render_basic_table;
use crate::controllers::dns_controller::execute_query;
use crate::models::dns_model::DnsQuery;

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
    widgets::{Block, Borders, Paragraph, Wrap},
    style::{Color},
    Terminal,
};

fn main() -> io::Result<()> {

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut input = String::new();
    let mut domain = DnsQuery::default();

    loop {
        terminal.draw(|f| {
            let size = f.area();

            // ==================================================
            // ROOT (70% izquierda, 30% derecha)
            // ==================================================

            let root = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ])
                .split(size);

            // ==================================================
            // IZQUIERDA
            // ==================================================

            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), //input
                    Constraint::Min(15),    // Panel principal
                    Constraint::Length(14), // Panel secundario
                ])
                .split(root[0]);
            
            
            // --------------------------------------------------
            // PANEL INPUT
            // --------------------------------------------------

            // Cursor visual
            let input_display = format!("{}|", input);
            let input_widget = Paragraph::new(input_display)
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .title("Input")
                        .borders(Borders::ALL),
                );

            f.render_widget(input_widget, left[0]);


            // --------------------------------------------------
            // PANEL PRINCIPAL con render_table
            // --------------------------------------------------


            let data = &domain.hosts;

            render_matrix_table(
                f,
                left[1],
                &data,
                &domain.domain,
            );
            // --------------------------------------------------
            // PANEL SECUNDARIO
            // --------------------------------------------------


            render_basic_table(
                f,
                left[2],
                "Email Records".to_string(),
                &format!("- SPF: {}\n- DMARC: {}\n- DKIM: {}",&domain.spf,&domain.dmarc,&domain.dkim),
                Color::White,
            );

            // ==================================================
            // DERECHA
            // ==================================================

            let right = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(4), // Whois
                    Constraint::Length(5), // SSL checker
                    Constraint::Min(1), // NS Records
                    Constraint::Min(1),    // MX Records
                    Constraint::Length(3), // Panel
                ])
                .split(root[1]);



            // --------------------------------------------------
            // PANEL A
            // --------------------------------------------------

            render_basic_table(
                f,
                right[0],
                "WHOIS".to_string(),
                &format!("- Registrant: {}\n- Expire on: {}\n- Estados:\n -{}",&domain.whois.registrar,&domain.whois.expire_date,&domain.whois.statuses),
                Color::LightBlue,
            );

            // --------------------------------------------------
            // PANEL MX
            // --------------------------------------------------

            let ssl_text = match domain.hosts.first() {
                Some(host) => format!(
                    "- Provider:\n{}\n- Expire on: {}",
                    host.ssl.organization,
                    host.ssl.date
                ),
                None => "No SSL information found".to_string(),
            };
            render_basic_table(
                f,
                right[1],
                "SSL Checker".to_string(),
                &ssl_text,
                Color::Cyan,
            );

            // --------------------------------------------------
            // PANEL NS
            // --------------------------------------------------

            render_basic_table(
                f,
                right[2],
                "NS Records".to_string(),
                &domain.ns,
                Color::Green,
            );

            // --------------------------------------------------
            // PANEL MX
            // --------------------------------------------------

            render_basic_table(
                f,
                right[3],
                "MX Records".to_string(),
                &domain.mx,
                Color::Yellow,
            );

            // --------------------------------------------------
            // FOOTER
            // --------------------------------------------------

            render_basic_table(
                f,
                right[4],
                "Panel".to_string(),
                &format!("Panel: {}",&domain.panel),
                Color::Blue,
            );
        })?;

        if crossterm::event::poll(
            std::time::Duration::from_millis(50),
        )? {
            if let crossterm::event::Event::Key(key) =
                crossterm::event::read()?
            {
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
                        domain = execute_query(&input);
                        
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
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    Ok(())
}
