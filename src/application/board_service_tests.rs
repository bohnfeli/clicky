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
