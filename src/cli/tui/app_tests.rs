use super::*;
use crate::application::BoardService;
use tempfile::TempDir;

#[test]
fn test_form_navigation() {
    let mut app = App::default();

    app.start_create_card();
    assert_eq!(app.form_field, FormField::Title);

    app.next_form_field();
    assert_eq!(app.form_field, FormField::Description);

    app.next_form_field();
    assert_eq!(app.form_field, FormField::Assignee);

    app.next_form_field();
    assert_eq!(app.form_field, FormField::Title);

    app.prev_form_field();
    assert_eq!(app.form_field, FormField::Assignee);
}

#[test]
fn test_form_field_input() {
    let mut app = App::default();
    app.start_create_card();

    *app.get_current_field_input_mut() = "Test Title".to_string();
    assert_eq!(app.form_data.title, "Test Title");

    app.next_form_field();
    *app.get_current_field_input_mut() = "Test Description".to_string();
    assert_eq!(app.form_data.description, "Test Description");

    app.next_form_field();
    *app.get_current_field_input_mut() = "Test Assignee".to_string();
    assert_eq!(app.form_data.assignee, "Test Assignee");
}

#[test]
fn test_submit_card_empty_title() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.start_create_card();

    *app.get_current_field_input_mut() = "".to_string();
    app.submit_card().unwrap();

    assert!(app.error_message.is_some());
    assert_eq!(app.error_message, Some("Title is required".to_string()));
    assert_eq!(app.state, AppState::CreateCard);
}

#[test]
fn test_create_card_success() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.start_create_card();

    *app.get_current_field_input_mut() = "Test Card".to_string();
    app.next_form_field();
    *app.get_current_field_input_mut() = "Test Description".to_string();
    app.next_form_field();
    *app.get_current_field_input_mut() = "Test Assignee".to_string();

    app.submit_card().unwrap();

    assert!(app.error_message.is_none());
    assert_eq!(app.state, AppState::Board);
    assert!(app.board.is_some());
    assert_eq!(app.board.as_ref().unwrap().cards.len(), 1);
    assert_eq!(app.board.as_ref().unwrap().cards[0].title, "Test Card");
}

#[test]
fn test_cancel_card() {
    let mut app = App::default();
    app.start_create_card();

    *app.get_current_field_input_mut() = "Test Title".to_string();
    app.cancel_card();

    assert_eq!(app.state, AppState::Board);
    assert_eq!(app.form_data.title, "");
    assert_eq!(app.form_data.description, "");
    assert_eq!(app.form_data.assignee, "");
}

#[test]
fn test_delete_card() {
    use crate::application::CardService;

    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    let created = card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    assert_eq!(app.board.as_ref().unwrap().cards.len(), 1);

    card_service
        .delete(temp_dir.path(), &created.card_id)
        .unwrap();

    app.load_board().unwrap();
    assert_eq!(app.board.as_ref().unwrap().cards.len(), 0);
}

#[test]
fn test_start_move_card() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    let _created = card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.selected_column = 0;
    app.selected_card = Some(0);

    app.start_move_card();

    assert_eq!(app.state, AppState::MoveCard);
    assert!(app.error_message.is_none());
}

#[test]
fn test_move_card_navigation() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    let _created = card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.selected_column = 0;
    app.selected_card = Some(0);

    app.start_move_card();
    let initial_target = app.selected_target_column;

    app.move_card_right();
    assert_eq!(app.selected_target_column, initial_target + 1);

    app.move_card_right();
    assert_eq!(app.selected_target_column, initial_target + 2);

    app.move_card_left();
    assert_eq!(app.selected_target_column, initial_target + 1);
}

#[test]
fn test_move_card_right_boundary() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    let _created = card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.selected_column = 0;
    app.selected_card = Some(0);

    app.start_move_card();
    app.selected_target_column = app.board.as_ref().unwrap().columns.len() - 1;

    app.move_card_right();
    assert_eq!(
        app.selected_target_column,
        app.board.as_ref().unwrap().columns.len() - 1
    );
}

#[test]
fn test_move_card_left_boundary() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    let _created = card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.selected_column = 0;
    app.selected_card = Some(0);

    app.start_move_card();
    app.selected_target_column = 0;

    app.move_card_left();
    assert_eq!(app.selected_target_column, 0);
}

#[test]
fn test_confirm_move_card_success() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    let created = card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.selected_column = 0;
    app.selected_card = Some(0);

    app.start_move_card();
    app.move_card_right();

    app.confirm_move_card().unwrap();

    assert_eq!(app.state, AppState::CardDetail);
    assert!(app.error_message.is_none());

    let card = app
        .board
        .as_ref()
        .unwrap()
        .get_card(&created.card_id)
        .unwrap();
    assert_eq!(card.column_id, "in_progress");
}

#[test]
fn test_cancel_move_card() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    let _created = card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.selected_column = 0;
    app.selected_card = Some(0);

    app.start_move_card();
    assert_eq!(app.state, AppState::MoveCard);

    app.cancel_move_card();

    assert_eq!(app.state, AppState::CardDetail);
    assert!(app.error_message.is_none());
}

#[test]
fn test_complete_move_flow() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    let created = card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.selected_column = 0;
    app.selected_card = Some(0);

    app.enter_cards();
    assert_eq!(app.state, AppState::Board);
    assert_eq!(app.focus, Focus::Cards);
    assert_eq!(app.selected_card, Some(0));

    app.start_move_card();
    assert_eq!(app.state, AppState::MoveCard);

    app.move_card_right();
    assert_eq!(app.selected_target_column, 1);

    app.confirm_move_card().unwrap();

    assert_eq!(app.state, AppState::CardDetail);
    assert!(app.error_message.is_none());

    let card = app
        .board
        .as_ref()
        .unwrap()
        .get_card(&created.card_id)
        .unwrap();
    assert_eq!(card.column_id, "in_progress");
}

#[test]
fn test_enter_cards_focus_transitions_to_card_detail() {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    let card_service = CardService::new();
    let _created = card_service
        .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
        .unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf());
    app.load_board().unwrap();
    app.selected_column = 0;

    assert_eq!(app.state, AppState::Board);
    assert_eq!(app.focus, Focus::Columns);

    app.enter_cards();
    assert_eq!(app.focus, Focus::Cards);
    assert_eq!(app.selected_card, Some(0));

    app.enter_card_detail();
    assert_eq!(app.state, AppState::CardDetail);
}
