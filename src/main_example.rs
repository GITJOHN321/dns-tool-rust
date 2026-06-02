use std::io;
mod ui;
use crate::ui::matrix_table::render_matrix_table;
mod models;

use crate::models::model_query_example::{DnsQuery, Host, Panel, Whois, Ssl};
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
    Terminal,
};

fn main() -> io::Result<()> {

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut input = String::new();
    let mut output_text = String::new();
    
    let ssl = Ssl {
        date: "2026-12-31".to_string(),
        organization: "Let's Encrypt".to_string(),
        active: "true".to_string(),
    };

    let host = Host {
        name: "mail.example.com".to_string(),
        ip: "192.168.1.10".to_string(),
        ptr: "mail.example.com".to_string(),
        ping: "24ms".to_string(),
        ssl,
    };

    let consulta = DnsQuery {
        domain: "example.com".to_string(),

        hosts: vec![host],

        spf: "v=spf1 include:_spf.google.com ~all".to_string(),
        dmarc: "v=DMARC1; p=none".to_string(),
        dkim: "selector._domainkey".to_string(),

        ns: "ns1.example.com, ns2.example.com".to_string(),
        mx: "mail.example.com".to_string(),

        panel: Panel {
            name: "cPanel".to_string(),
            version: "122".to_string(),
        },

        whois: Whois {
            date_register: "2020-01-01".to_string(),
            date_expire: "2030-01-01".to_string(),
            date_update: "2025-01-01".to_string(),
            status: "active".to_string(),
        },

        details: "Consulta completada".to_string(),
    };

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


            let data = vec![
                vec![
                    consulta.hosts[0].name.clone(),
                    "255.255.255.255".to_string(),
                    "dns.google".to_string(),
                    "10ms".to_string(),
                    "OK".to_string(),
                ],
                vec![
                    "mail".to_string(),
                    "1.1.1.1\n1.1.1.1\n1.1.1.1\n1.1.1.1\n1.1.1.1".to_string(),
                    "mail.google.com".to_string(),
                    "8ms".to_string(),

                    "OK".to_string(),
                ],
                vec![
                    "ftp".to_string(),
                    "1.1.1.1".to_string(),
                    "ftp.google.com".to_string(),
                    "8ms".to_string(),
                    "OK".to_string(),
                ],
                vec![
                    "webmail".to_string(),
                    "1.1.1.1".to_string(),
                    "webmail.google.com".to_string(),
                    "8ms".to_string(),
                    "OK".to_string(),
                ],
            ];
            let data = &domain.hosts;

            render_matrix_table(
                f,
                left[1],
                &data,
                &consulta.domain,
            );
            // --------------------------------------------------
            // PANEL SECUNDARIO
            // --------------------------------------------------

            let panel2 = Paragraph::new(
                "Panel secundario\n\n\
                Información complementaria.v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvn4MTg/2Jk0jwxHzA9Tu8oC0UsGZphsJ/AhyO1mVIlU84PHAU7wqkT+KETOBq9ibK4E0mHYzXDuF90iFEKAPpdXAxHTQVFp7FiVyFriaEPEP1acwIZsoJQ/r0PzGnFsvzyMKdmLu6RZvhts398pCgxp3lAtgEMNfg7QJ6cvNk7xyOIz1lEM/kIfkFvEvwTVz07EdLUiuYcPvTOh04eHYLXBSddjAyIpJMJa3Op3XHxbwXVShooEJTttOnP99i3xEZuwzHSHLeqr46mh0rw5RdcSaR+bHFCy3zH0LkmUElTG7NH+AqTEuUa3n3an91sC9ePdMBddvfHOtWnGJQJD7/QIDAQAB"
            )
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Panel Secundario")
                    .borders(Borders::ALL),
            );

            f.render_widget(panel2, left[2]);

            // ==================================================
            // DERECHA
            // ==================================================

            let right = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(5), // A
                    Constraint::Length(5), // MX
                    Constraint::Min(1), // NS
                    Constraint::Min(1),    // SPF/DKIM
                    Constraint::Length(3), // Footer
                ])
                .split(root[1]);



            // --------------------------------------------------
            // PANEL A
            // --------------------------------------------------

            let panel4_a = Paragraph::new(
                "142.250.184.14\n142.250.184.36"
            )
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("A Records")
                    .borders(Borders::ALL),
            );

            f.render_widget(panel4_a, right[0]);

            // --------------------------------------------------
            // PANEL MX
            // --------------------------------------------------

            let panel4_mx = Paragraph::new(
                "mail.google.com"
            )
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("MX Records")
                    .borders(Borders::ALL),
            );

            f.render_widget(panel4_mx, right[1]);

            // --------------------------------------------------
            // PANEL NS
            // --------------------------------------------------

            let panel4_ns = Paragraph::new(
                "ns1.google.com\nns2.google.com"
            )
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("NS Records")
                    .borders(Borders::ALL),
            );

            f.render_widget(panel4_ns, right[2]);

            // --------------------------------------------------
            // PANEL SPF / DKIM
            // --------------------------------------------------

            let panel4_spf = Paragraph::new(
                "SPF: OK\n\
                 DKIM: OK\n\n\
                 Información adicional."
            )
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("SPF / DKIM")
                    .borders(Borders::ALL),
            );

            f.render_widget(panel4_spf, right[3]);

            // --------------------------------------------------
            // FOOTER
            // --------------------------------------------------

            let panel5 = Paragraph::new(
                "Status: OK | Lookup: 12ms"
            )
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Footer")
                    .borders(Borders::ALL),
            );

            f.render_widget(panel5, right[4]);
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
                        output_text = "google.com".to_string();

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
