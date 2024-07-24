use crate::{TitlePopup, TitleSelectPopup, ORANGE};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs},
    Frame,
};

pub struct EditCommandsPopup {
    pub visible: bool,
}

impl EditCommandsPopup {
    pub fn new() -> Self {
        EditCommandsPopup { visible: false }
    }
}
impl Default for EditCommandsPopup {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_edit_commands_popup(f: &mut Frame) {
    let area = centered_rect(80, 80, f.size());
    f.render_widget(ratatui::widgets::Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ORANGE))
        .title("Editing Commands");

    let header = Row::new(vec![
        Cell::from("MAPPINGS").style(Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
        Cell::from("DESCRIPTIONS").style(Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
    ])
    .height(2);

    let commands: Vec<Row> = vec![
        Row::new(vec![
            "Ctrl+H, Backspace",
            "Delete one character before cursor",
        ]),
        Row::new(vec!["Ctrl+K", "Delete from cursor until the end of line"]),
        Row::new(vec![
            "Ctrl+W, Alt+Backspace",
            "Delete one word before cursor",
        ]),
        Row::new(vec!["Alt+D, Alt+Delete", "Delete one word next to cursor"]),
        Row::new(vec!["Ctrl+U", "Undo"]),
        Row::new(vec!["Ctrl+R", "Redo"]),
        Row::new(vec!["Ctrl+C, Copy", "Copy selected text"]),
        Row::new(vec!["Ctrl+X, Cut", "Cut selected text"]),
        Row::new(vec!["Ctrl+P, ↑", "Move cursor up by one line"]),
        Row::new(vec!["Ctrl+→", "Move cursor forward by word"]),
        Row::new(vec!["Ctrl+←", "Move cursor backward by word"]),
        Row::new(vec!["Ctrl+↑", "Move cursor up by paragraph"]),
        Row::new(vec!["Ctrl+↓", "Move cursor down by paragraph"]),
        Row::new(vec![
            "Ctrl+E, End, Ctrl+Alt+F, Ctrl+Alt+→",
            "Move cursor to the end of line",
        ]),
        Row::new(vec![
            "Ctrl+A, Home, Ctrl+Alt+B, Ctrl+Alt+←",
            "Move cursor to the head of line",
        ]),
        Row::new(vec!["Ctrl+M", "Format markdown block"]),
        Row::new(vec!["Ctrl+J", "Format JSON"]),
    ];

    let table = Table::new(commands, [Constraint::Length(5), Constraint::Length(5)])
        .header(header)
        .block(block)
        .widths([Constraint::Percentage(30), Constraint::Percentage(70)])
        .column_spacing(2)
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">> ");

    f.render_widget(table, area);
}

pub fn render_header(f: &mut Frame, area: Rect, is_edit_mode: bool) {
    let available_width = area.width as usize;

    let normal_commands = vec![
        "Q:Quit".to_string(),
        "^N:Add".to_string(),
        "^D:Del".to_string(),
        "^Y:Copy".to_string(),
        "^V:Paste".to_string(),
        "Enter:Edit".to_string(),
        "^F:Focus".to_string(),
        "Esc:Exit".to_string(),
        "^T:Title".to_string(),
        "^S:Select".to_string(),
        "^J:Format JSON".to_string(),
        "^M:Format Markdown".to_string(),
    ];

    let edit_commands = vec![
        "Esc:Exit Edit".to_string(),
        "^G:Move Cursor Top".to_string(),
        "^B:Copy Sel".to_string(),
        "Shift+↑↓:Sel".to_string(),
        "^Y:Copy All".to_string(),
        "^S:Select".to_string(),
        "^T:Title".to_string(),
        "^E:External Editor".to_string(),
        "^H:Help".to_string(),
    ];

    let commands = if is_edit_mode {
        &edit_commands
    } else {
        &normal_commands
    };

    let thoth = "Thoth  ";
    let separator = " | ";

    let mut display_commands: Vec<String> = Vec::new();
    let mut total_length = thoth.len();

    for cmd in commands {
        if total_length + cmd.len() + separator.len() > available_width {
            break;
        }
        display_commands.push(cmd.to_owned());
        total_length += cmd.len() + separator.len();
    }

    let command_string = display_commands.join(separator);
    let remaining_space = available_width.saturating_sub(total_length);

    let header = Line::from(vec![
        Span::styled(command_string, Style::default().fg(ORANGE)),
        Span::styled(" ".repeat(remaining_space), Style::default().fg(ORANGE)),
        Span::styled(thoth, Style::default().fg(ORANGE)),
    ]);

    let tabs = Tabs::new(vec![header])
        .style(Style::default().bg(Color::Black))
        .divider(Span::styled("|", Style::default().fg(ORANGE)));

    f.render_widget(tabs, area);
}

pub fn render_title_popup(f: &mut Frame, popup: &TitlePopup) {
    let area = centered_rect(60, 20, f.size());
    f.render_widget(ratatui::widgets::Clear, area);

    let text = Paragraph::new(popup.title.as_str())
        .style(Style::default().bg(Color::Black))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ORANGE))
                .title("Change Title"),
        );
    f.render_widget(text, area);
}

pub fn render_title_select_popup(f: &mut Frame, popup: &TitleSelectPopup) {
    let area = centered_rect(80, 80, f.size());
    f.render_widget(ratatui::widgets::Clear, area);

    let items: Vec<Line> = popup
        .titles
        .iter()
        .enumerate()
        .map(|(i, title)| {
            if i == popup.selected_index {
                Line::from(vec![Span::styled(
                    format!("> {}", title),
                    Style::default().fg(Color::Yellow),
                )])
            } else {
                Line::from(vec![Span::raw(format!("  {}", title))])
            }
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ORANGE))
        .title("Select Title");

    let paragraph = Paragraph::new(items)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(paragraph, area);
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use ratatui::{backend::TestBackend, Terminal};

    use super::*;

    #[test]
    fn test_centered_rect() {
        let r = Rect::new(0, 0, 100, 100);
        let centered = centered_rect(50, 50, r);
        assert_eq!(centered.width, 50);
        assert_eq!(centered.height, 50);
        assert_eq!(centered.x, 25);
        assert_eq!(centered.y, 25);
    }

    #[test]
    fn test_render_header() {
        let backend = TestBackend::new(100, 1);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let area = f.size();
                render_header(f, area, false);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();

        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("Q")));
        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("u")));
        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("i")));
        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("t")));

        assert!(buffer.content.iter().any(|cell| cell.fg == ORANGE));
    }

    #[test]
    fn test_render_title_popup() {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();
        let popup = TitlePopup {
            title: "Test Title".to_string(),
            visible: true,
        };

        terminal
            .draw(|f| {
                render_title_popup(f, &popup);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();

        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("T")));

        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("e")));

        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("s")));

        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("t")));

        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol() == "─" || cell.symbol() == "│"));
    }

    #[test]
    fn test_render_title_select_popup() {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();
        let popup = TitleSelectPopup {
            titles: vec!["Title1".to_string(), "Title2".to_string()],
            selected_index: 0,
            visible: true,
        };

        terminal
            .draw(|f| {
                render_title_select_popup(f, &popup);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();

        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains(">")));
        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("2")));

        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("1")));
    }

    #[test]
    fn test_render_edit_commands_popup() {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                render_edit_commands_popup(f);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();

        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("E")));

        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("H")));
        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("K")));

        assert!(buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("I") && cell.fg == ORANGE));
    }
}
