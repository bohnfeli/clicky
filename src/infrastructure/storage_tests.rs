use super::*;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_json_repository_save_and_load() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let board_path = temp_dir.path().join("board.json");
    let repo = JsonBoardRepository::new();
    let board = Board::new("test".to_string(), "Test Board".to_string());

    // Act
    repo.save(&board, &board_path).unwrap();
    let loaded = repo.load(&board_path).unwrap();

    // Assert
    assert_eq!(board.id, loaded.id);
    assert_eq!(board.name, loaded.name);
    assert_eq!(board.card_id_prefix, loaded.card_id_prefix);
}

#[test]
fn test_json_repository_exists() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let board_path = temp_dir.path().join("board.json");
    let repo = JsonBoardRepository::new();

    // Act & Assert
    assert!(!repo.exists(&board_path));

    // Create the file
    let mut file = fs::File::create(&board_path).unwrap();
    file.write_all(b"{}").unwrap();

    assert!(repo.exists(&board_path));
}

#[test]
fn test_json_repository_load_not_found() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let board_path = temp_dir.path().join("nonexistent.json");
    let repo = JsonBoardRepository::new();

    // Act
    let result = repo.load(&board_path);

    // Assert
    assert!(matches!(result, Err(StorageError::BoardNotFound(_))));
}

#[test]
fn test_board_storage_paths() {
    let base = Path::new("/home/user/project");

    let board_path = BoardStorage::board_path(base);
    assert_eq!(
        board_path,
        Path::new("/home/user/project/.clicky/board.json")
    );

    let clicky_dir = BoardStorage::clicky_dir(base);
    assert_eq!(clicky_dir, Path::new("/home/user/project/.clicky"));
}

#[test]
fn test_find_board_path() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let clicky_dir = temp_dir.path().join(".clicky");
    fs::create_dir(&clicky_dir).unwrap();

    let board_path = clicky_dir.join("board.json");
    fs::write(&board_path, "{}").unwrap();

    let subdir = temp_dir.path().join("src").join("components");
    fs::create_dir_all(&subdir).unwrap();

    // Act
    let found = BoardStorage::find_board_path(&subdir);

    // Assert
    assert_eq!(found, Some(board_path));
}

#[test]
fn test_find_board_path_not_found() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let subdir = temp_dir.path().join("src");
    fs::create_dir(&subdir).unwrap();

    // Act
    let found = BoardStorage::find_board_path(&subdir);

    // Assert
    assert!(found.is_none());
}
