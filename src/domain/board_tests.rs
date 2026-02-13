use super::*;

#[test]
fn test_board_creation() {
    // Arrange
    let id = "myproject".to_string();
    let name = "My Project".to_string();

    // Act
    let board = Board::new(id.clone(), name.clone());

    // Assert
    assert_eq!(board.id, id);
    assert_eq!(board.name, name);
    assert_eq!(board.card_id_prefix, "MYP");
    assert_eq!(board.columns.len(), 3);
    assert_eq!(board.next_card_number, 1);
}

#[test]
fn test_generate_prefix() {
    assert_eq!(Board::generate_prefix("myproject"), "MYP");
    assert_eq!(Board::generate_prefix("my-project"), "MYP");
    assert_eq!(Board::generate_prefix("my_project_123"), "MYP");
    assert_eq!(Board::generate_prefix("ab"), "AB");
}

#[test]
fn test_create_card() {
    // Arrange
    let mut board = Board::new("test".to_string(), "Test".to_string());

    // Act
    let card_id = board.create_card(
        "Test Task".to_string(),
        Some("Description".to_string()),
        Some("Alice".to_string()),
        None,
    );

    // Assert
    assert_eq!(card_id, "TES-001");
    assert_eq!(board.cards.len(), 1);
    assert_eq!(board.next_card_number, 2);

    let card = board.get_card(&card_id).unwrap();
    assert_eq!(card.title, "Test Task");
    assert_eq!(card.description, Some("Description".to_string()));
    assert_eq!(card.assignee, Some("Alice".to_string()));

    let todo_column = board.columns.iter().find(|c| c.id == "todo").unwrap();
    assert!(todo_column.has_card(&card_id));
}

#[test]
fn test_move_card() {
    // Arrange
    let mut board = Board::new("test".to_string(), "Test".to_string());
    let card_id = board.create_card("Task".to_string(), None, None, None);

    // Act
    let success = board.move_card(&card_id, "in_progress");

    // Assert
    assert!(success);
    let card = board.get_card(&card_id).unwrap();
    assert_eq!(card.column_id, "in_progress");

    let todo_column = board.columns.iter().find(|c| c.id == "todo").unwrap();
    assert!(!todo_column.has_card(&card_id));

    let progress_column = board
        .columns
        .iter()
        .find(|c| c.id == "in_progress")
        .unwrap();
    assert!(progress_column.has_card(&card_id));
}

#[test]
fn test_delete_card() {
    // Arrange
    let mut board = Board::new("test".to_string(), "Test".to_string());
    let card_id = board.create_card("Task".to_string(), None, None, None);

    // Act
    let deleted = board.delete_card(&card_id);

    // Assert
    assert!(deleted);
    assert!(board.get_card(&card_id).is_none());

    let todo_column = board.columns.iter().find(|c| c.id == "todo").unwrap();
    assert!(!todo_column.has_card(&card_id));
}
