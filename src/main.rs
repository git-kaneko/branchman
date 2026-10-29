mod app;
mod git;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::{env, io, path::PathBuf};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let repo_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    if !git::is_git_repo(&repo_path)? {
        eprintln!("エラー: 指定されたディレクトリはGitリポジトリではありません: {:?}", repo_path);
        std::process::exit(1);
    }

    let mut app = app::App::new(repo_path)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("エラー: {}", err);
    }

    Ok(())
}

fn run_app<B>(terminal: &mut Terminal<B>, app: &mut app::App) -> Result<()>
where
    B: ratatui::backend::Backend,
{
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                app::AppMode::Normal => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('j') | KeyCode::Down => app.next(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous(),
                        KeyCode::Char(' ') => app.toggle_selection(),
                        KeyCode::Char('d') => {
                            if !app.get_selected_branches().is_empty() {
                                app.mode = app::AppMode::ConfirmDelete;
                            }
                        }
                        KeyCode::Char('g') => {
                            if key.modifiers == KeyModifiers::NONE {
                                app.jump_to_top();
                            }
                        }
                        KeyCode::Char('G') => app.jump_to_bottom(),
                        _ => {}
                    }
                }
                app::AppMode::ConfirmDelete => {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Enter => {
                            app.delete_selected_branches()?;
                            app.mode = app::AppMode::Normal;
                        }
                        KeyCode::Char('n') | KeyCode::Esc => {
                            app.mode = app::AppMode::Normal;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}