//! TUI rendering

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::app::{App, FocusedPanel, LogLevel, TaskStatus};

/// Render the UI
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Min(10),   // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(frame.area());

    render_title(frame, chunks[0]);
    render_main(frame, chunks[1], app);
    render_status_bar(frame, chunks[2], app);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new(" Macroforge Tooling Dashboard")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    frame.render_widget(title, area);
}

fn render_main(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    render_task_list(frame, chunks[0], app);
    render_output_log(frame, chunks[1], app);
}

fn render_task_list(frame: &mut Frame, area: Rect, app: &App) {
    let focused = matches!(app.focused, FocusedPanel::TaskList);
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let status_icon = match &task.status {
                TaskStatus::Pending => "[ ]",
                TaskStatus::Running => "[*]",
                TaskStatus::Completed => "[✓]",
                TaskStatus::Failed(_) => "[✗]",
            };

            let style = if i == app.selected_task {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let content = format!("{} {}", status_icon, task.name);
            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" Tasks (j/k nav, Enter run) ")
            .borders(Borders::ALL)
            .border_style(border_style),
    );

    frame.render_widget(list, area);
}

fn render_output_log(frame: &mut Frame, area: Rect, app: &App) {
    let focused = matches!(app.focused, FocusedPanel::OutputLog);
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let logs: Vec<Line> = app
        .logs
        .iter()
        .rev()
        .take(area.height as usize - 2)
        .rev()
        .map(|entry| {
            let (prefix, color) = match entry.level {
                LogLevel::Info => (">", Color::White),
                LogLevel::Success => ("✓", Color::Green),
                LogLevel::Warning => ("!", Color::Yellow),
                LogLevel::Error => ("✗", Color::Red),
            };

            Line::from(vec![
                Span::styled(format!("{} ", prefix), Style::default().fg(color)),
                Span::raw(&entry.message),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(logs).block(
        Block::default()
            .title(" Output Log ")
            .borders(Borders::ALL)
            .border_style(border_style),
    );

    frame.render_widget(paragraph, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    let status = if app.running {
        "Running..."
    } else {
        "Ready"
    };

    let time = chrono::Local::now().format("%H:%M:%S").to_string();

    let status_line = Line::from(vec![
        Span::styled(" Status: ", Style::default().fg(Color::DarkGray)),
        Span::styled(status, Style::default().fg(Color::Green)),
        Span::raw(" │ "),
        Span::styled("q", Style::default().fg(Color::Yellow)),
        Span::raw(" quit │ "),
        Span::styled(time, Style::default().fg(Color::DarkGray)),
    ]);

    let paragraph = Paragraph::new(status_line)
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    frame.render_widget(paragraph, area);
}
