use super::*;
use tempfile::TempDir;

fn setup_test_board() -> (TempDir, CardService) {
    let temp_dir = TempDir::new().unwrap();
    let board_service = BoardService::new();
    board_service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();
    let card_service = CardService::new();
    (temp_dir, card_service)
}

#[test]
fn test_create_card() {
    // Arrange
    let (temp_dir, service) = setup_test_board();

    // Act
    let result = service
        .create(
            temp_dir.path(),
            "Test Task".to_string(),
            Some("Description".to_string()),
            Some("Alice".to_string()),
            None,
        )
        .unwrap();

    // Assert
    assert_eq!(result.card_id, "TES-001");
    assert_eq!(result.board.cards.len(), 1);
}

#[test]
fn test_create_card_invalid_column() {
    // Arrange
    let (temp_dir, service) = setup_test_board();

    // Act
    let result = service.create(
        temp_dir.path(),
        "Test".to_string(),
        None,
        None,
        Some("invalid".to_string()),
    );

    // Assert
    assert!(matches!(result, Err(CardServiceError::ColumnNotFound(_))));
}

#[test]
fn test_move_card() {
    // Arrange
    let (temp_dir, service) = setup_test_board();
    let created = service
        .create(temp_dir.path(), "Task".to_string(), None, None, None)
        .unwrap();

    // Act
    let board = service
        .move_to(temp_dir.path(), &created.card_id, "in_progress")
        .unwrap();

    // Assert
    let card = board.get_card(&created.card_id).unwrap();
    assert_eq!(card.column_id, "in_progress");
}

#[test]
fn test_move_card_not_found() {
    // Arrange
    let (temp_dir, service) = setup_test_board();

    // Act
    let result = service.move_to(temp_dir.path(), "TES-999", "done");

    // Assert
    assert!(matches!(result, Err(CardServiceError::CardNotFound(_))));
}

#[test]
fn test_update_card() {
    // Arrange
    let (temp_dir, service) = setup_test_board();
    let created = service
        .create(temp_dir.path(), "Old Title".to_string(), None, None, None)
        .unwrap();

    // Act
    let board = service
        .update(
            temp_dir.path(),
            &created.card_id,
            Some("New Title".to_string()),
            Some(Some("New Desc".to_string())),
            Some(Some("Bob".to_string())),
        )
        .unwrap();

    // Assert
    let card = board.get_card(&created.card_id).unwrap();
    assert_eq!(card.title, "New Title");
    assert_eq!(card.description, Some("New Desc".to_string()));
    assert_eq!(card.assignee, Some("Bob".to_string()));
}

#[test]
fn test_delete_card() {
    // Arrange
    let (temp_dir, service) = setup_test_board();
    let created = service
        .create(temp_dir.path(), "Task".to_string(), None, None, None)
        .unwrap();

    // Act
    let board = service.delete(temp_dir.path(), &created.card_id).unwrap();

    // Assert
    assert!(board.get_card(&created.card_id).is_none());
    assert_eq!(board.cards.len(), 0);
}
