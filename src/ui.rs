use crate::app::{App, AppMode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    draw_branch_list(f, app, chunks[0]);
    draw_help_text(f, app, chunks[1]);

    if app.mode == AppMode::ConfirmDelete {
        draw_confirm_dialog(f, app);
    }
}

fn draw_branch_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .branches
        .iter()
        .enumerate()
        .map(|(i, branch)| {
            let is_selected = i == app.selected_index;
            let is_checked = app.selected_branches.contains(&branch.name);
            let is_protected = app.is_protected_branch(&branch.name);

            let checkbox = if is_protected {
                "[-]"
            } else if is_checked {
                "[x]"
            } else {
                "[ ]"
            };

            let current_marker = if branch.is_current { " ★" } else { "" };

            let date_str = branch
                .last_commit_date
                .format("%Y-%m-%d %H:%M")
                .to_string();

            let content = format!(
                "{} {} | {}{}",
                checkbox, branch.name, date_str, current_marker
            );

            let style = if is_protected {
                Style::default().fg(Color::DarkGray)
            } else if is_selected {
                Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
            } else if is_checked {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" ブランチ一覧 "),
        )
        .highlight_style(Style::default());

    f.render_widget(list, area);
}

fn draw_help_text(f: &mut Frame, app: &App, area: Rect) {
    let help_text = match app.mode {
        AppMode::Normal => {
            vec![
                Line::from(vec![
                    Span::styled("j/↓", Style::default().fg(Color::Cyan)),
                    Span::raw(": 下へ  "),
                    Span::styled("k/↑", Style::default().fg(Color::Cyan)),
                    Span::raw(": 上へ  "),
                    Span::styled("Space", Style::default().fg(Color::Cyan)),
                    Span::raw(": 選択  "),
                    Span::styled("d", Style::default().fg(Color::Cyan)),
                    Span::raw(": 削除  "),
                    Span::styled("g", Style::default().fg(Color::Cyan)),
                    Span::raw(": 先頭へ  "),
                    Span::styled("G", Style::default().fg(Color::Cyan)),
                    Span::raw(": 末尾へ  "),
                    Span::styled("q", Style::default().fg(Color::Cyan)),
                    Span::raw(": 終了"),
                ])
            ]
        }
        AppMode::ConfirmDelete => {
            vec![
                Line::from(vec![
                    Span::styled("y/Enter", Style::default().fg(Color::Red)),
                    Span::raw(": 削除実行  "),
                    Span::styled("n/Esc", Style::default().fg(Color::Green)),
                    Span::raw(": キャンセル"),
                ])
            ]
        }
    };

    let paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn draw_confirm_dialog(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 40, f.area());

    let selected_branches = app.get_selected_branches();

    let mut text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "以下のブランチを削除しますか？",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
    ];

    for branch in &selected_branches {
        text.push(Line::from(format!("- {}", branch)));
    }

    text.push(Line::from(""));
    text.push(Line::from(Span::styled(
        "削除を実行する場合は 'y' または Enter を押してください",
        Style::default().fg(Color::Red),
    )));
    text.push(Line::from(Span::styled(
        "キャンセルする場合は 'n' または Esc を押してください",
        Style::default().fg(Color::Green),
    )));

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(" 削除確認 ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red)),
        )
        .alignment(Alignment::Center);

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}