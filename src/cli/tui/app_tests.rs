use super::App;
use crate::application::{BoardService, CardService};
use crate::cli::tui::handle_form_input;
use crate::cli::tui::state::{FormField, FormMode, InputMode, Selection, View};
use tempfile::TempDir;

#[test]
fn test_form_navigation() {
    let mut app = App::default();
    app.start_create_card();

    let form_data = app.get_form_data().unwrap();
    assert_eq!(form_data.current_field, FormField::Title);

    app.next_form_field();
    let form_data = app.get_form_data().unwrap();
    assert_eq!(form_data.current_field, FormField::Description);

    app.next_form_field();
    let form_data = app.get_form_data().unwrap();
    assert_eq!(form_data.current_field, FormField::Assignee);

    app.next_form_field();
    let form_data = app.get_form_data().unwrap();
    assert_eq!(form_data.current_field, FormField::Title);

    app.prev_form_field();
    let form_data = app.get_form_data().unwrap();
    assert_eq!(form_data.current_field, FormField::Assignee);
}

#[test]
fn test_form_field_input() {
    let mut app = App::default();
    app.start_create_card();

    *app.get_current_field_input_mut().unwrap() = "Test Title".to_string();
    let form_data = app.get_form_data().unwrap();
    assert_eq!(form_data.title, "Test Title");

    app.next_form_field();
    *app.get_current_field_input_mut().unwrap() = "Test Description".to_string();
    let form_data = app.get_form_data().unwrap();
    assert_eq!(form_data.description, "Test Description");

    app.next_form_field();
    *app.get_current_field_input_mut().unwrap() = "Test Assignee".to_string();
    let form_data = app.get_form_data().unwrap();
    assert_eq!(form_data.assignee, "Test Assignee");
}

#[test]
fn test_auto_enter_edit_mode_when_typing() {
    use crate::cli::tui::handle_form_input;
    use crossterm::event::{KeyCode, KeyEvent};

    let mut app = App::default();
    app.start_create_card();

    assert_eq!(app.get_input_mode(), InputMode::Normal);
    assert!(matches!(
        app.view,
        View::CardForm {
            mode: FormMode::Create { .. }
        }
    ));

    let key = KeyEvent::new(KeyCode::Char('T'), crossterm::event::KeyModifiers::empty());
    handle_form_input(&mut app, &key);

    assert_eq!(app.get_input_mode(), InputMode::Editing);
    let form_data = app.get_form_data().unwrap();
    assert_eq!(form_data.title, "T");

    let key2 = KeyEvent::new(KeyCode::Char('e'), crossterm::event::KeyModifiers::empty());
    handle_form_input(&mut app, &key2);

    assert_eq!(app.get_input_mode(), InputMode::Editing);
    let form_data = app.get_form_data().unwrap();
    assert_eq!(form_data.title, "Te");
}

#[test]
fn test_create_card_full_workflow_with_typing() {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.start_create_card();

    assert!(matches!(
        app.view,
        View::CardForm {
            mode: FormMode::Create { .. }
        }
    ));

    let modifiers = KeyModifiers::empty();

    let key_t = KeyEvent::new(KeyCode::Char('T'), modifiers);
    let key_e = KeyEvent::new(KeyCode::Char('e'), modifiers);
    let key_s = KeyEvent::new(KeyCode::Char('s'), modifiers);
    let key_t2 = KeyEvent::new(KeyCode::Char('t'), modifiers);
    let enter = KeyEvent::new(KeyCode::Enter, modifiers);

    handle_form_input(&mut app, &key_t);
    handle_form_input(&mut app, &key_e);
    handle_form_input(&mut app, &key_s);
    handle_form_input(&mut app, &key_t2);

    let form_data = app.get_form_data().unwrap();
    assert_eq!(form_data.title, "Test");

    handle_form_input(&mut app, &enter);

    let form_data = app.get_form_data().unwrap();
    assert_eq!(form_data.current_field, FormField::Description);
}

#[test]
fn test_create_card_requires_title() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.start_create_card();

    let result = app.submit_card();
    assert!(result.is_ok());

    assert_eq!(app.error_message, Some("Title is required".to_string()));
    assert!(matches!(
        app.view,
        View::CardForm {
            mode: FormMode::Create { .. }
        }
    ));
}

#[test]
fn test_cancel_card_returns_to_board() {
    let mut app = App::default();
    app.start_create_card();

    app.cancel_card();

    assert!(matches!(app.view, View::Board { .. }));
    assert!(app.get_form_data().is_none());
}

#[test]
fn test_move_up_down_navigation() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();

    app.move_down();
    if let View::Board {
        selection: Selection::Highlighted { card_index, .. },
        ..
    } = app.view
    {
        assert_eq!(card_index, 0);
    } else {
        panic!("Expected Highlighted selection after move_down");
    }

    app.move_up();
    if let View::Board {
        selection: Selection::Highlighted { card_index, .. },
        ..
    } = app.view
    {
        assert_eq!(card_index, 0);
    } else {
        panic!("Expected Highlighted selection after move_up");
    }
}

#[test]
fn test_move_left_right_navigation() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();

    let initial_column = app.view.selected_column();
    assert_eq!(initial_column, 0);

    app.move_right();
    assert_eq!(app.view.selected_column(), 1);

    app.move_left();
    assert_eq!(app.view.selected_column(), 0);
}

#[test]
fn test_enter_cards_selects_card() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.move_down();
    app.enter_cards();

    assert!(app.view.is_card_selected());
    assert!(app.view.selected_card_id().is_some());
}

#[test]
fn test_exit_cards_deselects() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.move_down();
    app.enter_cards();

    assert!(app.view.is_card_selected());

    app.exit_cards();

    assert!(!app.view.is_card_selected());
}

#[test]
fn test_toggle_help() {
    let mut app = App::default();

    assert!(!app.view.is_help());

    app.toggle_help();
    assert!(app.view.is_help());

    app.toggle_help();
    assert!(!app.view.is_help());
}

#[test]
fn test_selected_card_id_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.move_down();
    app.enter_cards();

    let card_id = app.view.selected_card_id();
    assert!(card_id.is_some());
    let card_id_str = card_id.unwrap();
    assert!(card_id_str.starts_with("TES-"));
}

#[test]
fn test_get_current_column() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();

    let column_id = app.get_current_column();
    assert_eq!(column_id, Some("todo"));
}

#[test]
fn test_input_mode_management() {
    let mut app = App::default();
    app.start_create_card();

    assert_eq!(app.get_input_mode(), InputMode::Normal);

    app.set_input_mode(InputMode::Editing);
    assert_eq!(app.get_input_mode(), InputMode::Editing);

    app.set_input_mode(InputMode::Normal);
    assert_eq!(app.get_input_mode(), InputMode::Normal);
}

#[test]
fn test_view_default_state() {
    let view = View::default();
    assert!(matches!(view, View::Board { .. }));
}

#[test]
fn test_selection_enum() {
    let none = Selection::default();
    assert!(matches!(none, Selection::None));

    let highlighted = Selection::Highlighted {
        column: 0,
        card_index: 1,
    };
    assert_eq!(highlighted.card_index(), Some(1));
    assert!(highlighted.card_id().is_none());

    let selected = Selection::Selected {
        card_id: "PRJ-001".to_string(),
    };
    assert_eq!(selected.card_id(), Some("PRJ-001"));
    assert!(selected.card_index().is_none());
}

#[test]
fn test_form_field_navigation() {
    assert_eq!(FormField::Title.next(), FormField::Description);
    assert_eq!(FormField::Description.next(), FormField::Assignee);
    assert_eq!(FormField::Assignee.next(), FormField::Title);

    assert_eq!(FormField::Title.prev(), FormField::Assignee);
    assert_eq!(FormField::Description.prev(), FormField::Title);
    assert_eq!(FormField::Assignee.prev(), FormField::Description);
}

#[test]
fn test_card_form_data_validation() {
    let mut data = crate::cli::tui::state::CardFormData::default();
    assert!(!data.is_valid());

    data.title = "Test Title".to_string();
    assert!(data.is_valid());

    let (title, desc, assignee) = data.clone().to_card_values();
    assert_eq!(title, "Test Title");
    assert!(desc.is_none());
    assert!(assignee.is_none());

    data.description = "Test Description".to_string();
    data.assignee = "test@test.com".to_string();

    let (title, desc, assignee) = data.to_card_values();
    assert_eq!(title, "Test Title");
    assert_eq!(desc, Some("Test Description".to_string()));
    assert_eq!(assignee, Some("test@test.com".to_string()));
}

#[test]
fn test_get_selected_card_index() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();

    assert!(app.get_selected_card_index().is_none());

    app.move_down();
    app.enter_cards();

    assert!(app.get_selected_card_index().is_some());
}

#[test]
fn test_get_highlighted_card_index() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();

    assert!(app.get_highlighted_card_index().is_none());

    app.move_down();

    assert_eq!(app.get_highlighted_card_index(), Some(0));
}

#[test]
fn test_deselect_card() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.move_down();
    app.enter_cards();

    assert!(app.view.is_card_selected());

    app.deselect_card();

    assert!(!app.view.is_card_selected());
}

#[test]
fn test_error_message_handling() {
    let mut app = App::default();

    assert!(app.error_message.is_none());

    app.error_message = Some("Test error".to_string());
    assert_eq!(app.error_message, Some("Test error".to_string()));

    app.clear_error();
    assert!(app.error_message.is_none());
}

#[test]
fn test_open_card_detail() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.move_down();

    app.open_card_detail();

    assert!(matches!(app.view, View::CardDetail { .. }));
}

#[test]
fn test_selection_is_in_column() {
    let selection = Selection::Highlighted {
        column: 1,
        card_index: 2,
    };
    assert!(!selection.is_in_column(0));
    assert!(selection.is_in_column(1));

    let selected = Selection::Selected {
        card_id: "PRJ-001".to_string(),
    };
    assert!(selected.is_in_column(0));
    assert!(selected.is_in_column(1));
}
