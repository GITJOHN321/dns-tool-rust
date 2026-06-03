use ratatui::{
    prelude::*,
    style::{Color,Modifier, Style},
    text::Span,
    widgets::{Block,Wrap,Paragraph, Borders},
};

pub fn render_basic_table(
    f: &mut Frame,
    area: Rect,
    title:String,
    domain:&str,
    color: Color
) {

    let panel4_ns = Paragraph::new(
        format!("{}",&domain)
    )
    .style(
        Style::default().fg(color)
    )
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .title(Span::styled(title,Style::default().fg(Color::White).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL),
    );

    f.render_widget(panel4_ns, area);
}