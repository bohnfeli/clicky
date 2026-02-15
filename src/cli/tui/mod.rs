//! Terminal UI (TUI) mode for Clicky.
//!
//! Provides a full-featured interactive kanban board in the terminal.

pub mod app;
pub mod components;
pub mod events;
pub mod state;
pub mod ui;

pub use app::App;

/// Run the TUI application.
pub fn run(board_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    use crossterm::{
        cursor::Hide,
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::backend::CrosstermBackend;
    use std::io;

    // Setup terminal
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture, Hide)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = ratatui::Terminal::new(backend)?;

    // Create app and load board
    let mut app = App::new(board_path.to_path_buf());
    app.load_board()?;

    // Setup event handler
    let mut events = events::EventHandler::new();

    // Run the application loop
    let res = run_app(&mut terminal, &mut app, &mut events);

    // Restore terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    events.stop();

    res
}

fn run_app<B>(
    terminal: &mut ratatui::Terminal<B>,
    app: &mut App,
    events: &mut events::EventHandler,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: ratatui::backend::Backend,
    <B as ratatui::backend::Backend>::Error: std::error::Error + Send + Sync + 'static,
{
    use crossterm::cursor::{Hide, Show};
    use crossterm::event::KeyCode;
    use crossterm::execute;
    use std::io;

    let mut should_quit = false;

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        let should_show_cursor = matches!(
            app.state,
            state::AppState::CreateCard | state::AppState::EditCard
        ) && app.input_mode == state::InputMode::Editing;

        if should_show_cursor {
            execute!(io::stdout(), Show)?;
        } else {
            execute!(io::stdout(), Hide)?;
        }

        if let Some(event) = events.try_next() {
            match event {
                events::Event::Input(key) => {
                    match app.state {
                        state::AppState::Board => handle_board_input(app, &key),
                        state::AppState::CardDetail => handle_card_detail_input(app, &key),
                        state::AppState::CreateCard => handle_create_card_input(app, &key),
                        state::AppState::EditCard => handle_edit_card_input(app, &key),
                        state::AppState::ConfirmDelete => handle_confirm_delete_input(app, &key),
                        state::AppState::MoveCard => handle_move_card_input(app, &key),
                        state::AppState::Help => {
                            if key.code == KeyCode::Esc || key.code == KeyCode::Char('?') {
                                app.toggle_help();
                            }
                        }
                    }

                    if app.state == state::AppState::Board && key.code == KeyCode::Char('q') {
                        should_quit = true;
                    }
                }
                events::Event::Tick => {}
            }
        }

        if should_quit {
            execute!(io::stdout(), Show)?;
            return Ok(());
        }
    }
}

fn handle_board_input(app: &mut App, key: &crossterm::event::KeyEvent) {
    use crate::cli::tui::state::Focus;
    use crossterm::event::KeyCode;

    match key.code {
        KeyCode::Left | KeyCode::Char('h') => {
            if app.card_selected {
                let _ = app.quick_move_card_left();
            } else {
                app.move_left();
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            if app.card_selected {
                let _ = app.quick_move_card_right();
            } else {
                app.move_right();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
        KeyCode::Enter => {
            if app.focus == Focus::Cards {
                app.enter_card_detail();
            } else {
                app.enter_cards();
            }
        }
        KeyCode::Esc => {
            if app.card_selected {
                app.deselect_card();
            } else {
                app.exit_cards();
            }
        }
        KeyCode::Char('c') => {
            app.start_create_card();
        }
        KeyCode::Char('d') => {
            app.open_card_detail();
        }
        KeyCode::Char('m') => {
            if app.get_selected_card_index().is_some() && !app.card_selected {
                app.start_move_card();
            }
        }
        KeyCode::Char('q') => {}
        KeyCode::Char('?') => {
            app.toggle_help();
        }
        _ => {}
    }
}

fn handle_card_detail_input(app: &mut App, key: &crossterm::event::KeyEvent) {
    use crossterm::event::KeyCode;

    match key.code {
        KeyCode::Char('e') => {
            app.state = state::AppState::EditCard;
        }
        KeyCode::Char('d') => {
            app.state = state::AppState::ConfirmDelete;
        }
        KeyCode::Char('m') => {
            app.start_move_card();
        }
        KeyCode::Esc | KeyCode::Char('q') => {
            app.state = state::AppState::Board;
            app.card_selected = false;
        }
        KeyCode::Char('?') => {
            app.toggle_help();
        }
        _ => {}
    }
}

fn handle_create_card_input(app: &mut App, key: &crossterm::event::KeyEvent) {
    use crossterm::event::KeyCode;

    match app.input_mode {
        state::InputMode::Normal => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                app.prev_form_field();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.next_form_field();
            }
            KeyCode::Enter => {
                let _ = app.submit_card();
            }
            KeyCode::Esc => {
                app.cancel_card();
            }
            KeyCode::Char('?') => {
                app.toggle_help();
            }
            KeyCode::Char(c) => {
                app.input_mode = state::InputMode::Editing;
                let input = app.get_current_field_input_mut();
                input.push(c);
            }
            _ => {}
        },
        state::InputMode::Editing => {
            let input = app.get_current_field_input_mut();

            match key.code {
                KeyCode::Char(c) => {
                    input.push(c);
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Enter => {
                    app.next_form_field();
                    app.input_mode = state::InputMode::Normal;
                }
                KeyCode::Esc => {
                    app.input_mode = state::InputMode::Normal;
                }
                _ => {}
            }
        }
    }
}

fn handle_edit_card_input(app: &mut App, key: &crossterm::event::KeyEvent) {
    handle_create_card_input(app, key);
}

fn handle_confirm_delete_input(app: &mut App, key: &crossterm::event::KeyEvent) {
    use crate::application::CardService;
    use crossterm::event::KeyCode;

    match key.code {
        KeyCode::Char('y') => {
            if let Some(card_id) = app.selected_card_id() {
                let card_service = CardService::new();
                match card_service.delete(&app.board_path, &card_id) {
                    Ok(_) => {
                        app.selected_card = None;
                        app.selected_card_id = None;
                        let _ = app.load_board();
                        app.state = state::AppState::Board;
                    }
                    Err(e) => {
                        app.error_message = Some(format!("Failed to delete: {}", e));
                        app.state = state::AppState::CardDetail;
                    }
                }
            }
        }
        KeyCode::Char('n') | KeyCode::Esc => {
            app.state = state::AppState::CardDetail;
        }
        KeyCode::Char('?') => {
            app.toggle_help();
        }
        _ => {}
    }
}

fn handle_move_card_input(app: &mut App, key: &crossterm::event::KeyEvent) {
    use crossterm::event::KeyCode;

    match key.code {
        KeyCode::Left | KeyCode::Char('h') => {
            app.move_card_left();
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.move_card_right();
        }
        KeyCode::Enter => {
            if let Err(e) = app.confirm_move_card() {
                app.error_message = Some(format!("Failed to move card: {}", e));
            }
        }
        KeyCode::Esc => {
            app.cancel_move_card();
        }
        KeyCode::Char('?') => {
            app.toggle_help();
        }
        _ => {}
    }
}
