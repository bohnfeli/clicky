use super::*;
use tempfile::TempDir;

#[test]
fn test_initialize_board() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let service = BoardService::new();

    // Act
    let board = service
        .initialize(temp_dir.path(), Some("My Project".to_string()))
        .unwrap();

    // Assert
    assert_eq!(board.name, "My Project");
    assert_eq!(board.id, "my-project");
    assert_eq!(board.card_id_prefix, "MYP");
    assert!(service.exists(temp_dir.path()));
}

#[test]
fn test_initialize_board_default_name() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let service = BoardService::new();

    // Act
    let board = service.initialize(temp_dir.path(), None).unwrap();

    // Assert - should use directory name
    assert!(!board.name.is_empty());
}

#[test]
fn test_initialize_already_exists() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let service = BoardService::new();
    service.initialize(temp_dir.path(), None).unwrap();

    // Act
    let result = service.initialize(temp_dir.path(), None);

    // Assert
    assert!(matches!(result, Err(BoardServiceError::AlreadyInitialized)));
}

#[test]
fn test_load_board() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let service = BoardService::new();
    let created = service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    // Act
    let loaded = service.load(temp_dir.path()).unwrap();

    // Assert
    assert_eq!(created.id, loaded.id);
    assert_eq!(created.name, loaded.name);
}

#[test]
fn test_load_not_found() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let service = BoardService::new();

    // Act
    let result = service.load(temp_dir.path());

    // Assert
    assert!(matches!(result, Err(BoardServiceError::BoardNotFound)));
}

#[test]
fn test_sanitize_id() {
    assert_eq!(BoardService::sanitize_id("My Project"), "my-project");
    assert_eq!(BoardService::sanitize_id("My_Project"), "my-project");
    assert_eq!(
        BoardService::sanitize_id("My Project 123"),
        "my-project-123"
    );
    assert_eq!(BoardService::sanitize_id("My@Project!"), "myproject");
}

#[test]
fn test_reorder_card_up() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let service = BoardService::new();
    let mut board = service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    board.create_card("Card 1".to_string(), None, None, None);
    board.create_card("Card 2".to_string(), None, None, None);
    board.create_card("Card 3".to_string(), None, None, None);
    service.save(&board, temp_dir.path()).unwrap();

    let loaded = service.load(temp_dir.path()).unwrap();
    let column = loaded.columns.get(0).unwrap();
    let card_ids = column.cards.clone();

    // Act - move second card up
    let result = service.reorder_card_in_column(temp_dir.path(), &card_ids[1], MoveDirection::Up);

    // Assert
    assert!(result.is_ok());
    let reloaded = service.load(temp_dir.path()).unwrap();
    let column = reloaded.columns.get(0).unwrap();
    assert_eq!(
        column.cards,
        vec![
            card_ids[1].clone(),
            card_ids[0].clone(),
            card_ids[2].clone()
        ]
    );
}

#[test]
fn test_reorder_card_down() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let service = BoardService::new();
    let mut board = service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    board.create_card("Card 1".to_string(), None, None, None);
    board.create_card("Card 2".to_string(), None, None, None);
    board.create_card("Card 3".to_string(), None, None, None);
    service.save(&board, temp_dir.path()).unwrap();

    let loaded = service.load(temp_dir.path()).unwrap();
    let column = loaded.columns.get(0).unwrap();
    let card_ids = column.cards.clone();

    // Act - move second card down
    let result = service.reorder_card_in_column(temp_dir.path(), &card_ids[1], MoveDirection::Down);

    // Assert
    assert!(result.is_ok());
    let reloaded = service.load(temp_dir.path()).unwrap();
    let column = reloaded.columns.get(0).unwrap();
    assert_eq!(
        column.cards,
        vec![
            card_ids[0].clone(),
            card_ids[2].clone(),
            card_ids[1].clone()
        ]
    );
}

#[test]
fn test_reorder_card_at_top() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let service = BoardService::new();
    let mut board = service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    board.create_card("Card 1".to_string(), None, None, None);
    board.create_card("Card 2".to_string(), None, None, None);
    service.save(&board, temp_dir.path()).unwrap();

    let loaded = service.load(temp_dir.path()).unwrap();
    let column = loaded.columns.get(0).unwrap();
    let card_ids = column.cards.clone();

    // Act - try to move first card up
    let result = service.reorder_card_in_column(temp_dir.path(), &card_ids[0], MoveDirection::Up);

    // Assert
    assert!(result.is_err());
}

#[test]
fn test_reorder_card_at_bottom() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let service = BoardService::new();
    let mut board = service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    board.create_card("Card 1".to_string(), None, None, None);
    board.create_card("Card 2".to_string(), None, None, None);
    service.save(&board, temp_dir.path()).unwrap();

    let loaded = service.load(temp_dir.path()).unwrap();
    let column = loaded.columns.get(0).unwrap();
    let card_ids = column.cards.clone();

    // Act - try to move last card down
    let result = service.reorder_card_in_column(temp_dir.path(), &card_ids[1], MoveDirection::Down);

    // Assert
    assert!(result.is_err());
}

#[test]
fn test_reorder_card_not_found() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let service = BoardService::new();
    service
        .initialize(temp_dir.path(), Some("Test".to_string()))
        .unwrap();

    // Act - try to move non-existent card
    let result =
        service.reorder_card_in_column(temp_dir.path(), "NONEXISTENT-123", MoveDirection::Up);

    // Assert
    assert!(result.is_err());
}

#[test]
fn test_reorder_card_board_not_found() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let service = BoardService::new();

    // Act - try to reorder card when no board exists
    let result = service.reorder_card_in_column(temp_dir.path(), "PRJ-001", MoveDirection::Up);

    // Assert
    assert!(matches!(result, Err(BoardServiceError::BoardNotFound)));
}
