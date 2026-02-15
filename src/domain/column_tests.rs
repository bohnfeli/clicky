use super::*;

#[test]
fn test_column_creation() {
    // Arrange
    let id = "todo".to_string();
    let name = "To Do".to_string();

    // Act
    let column = Column::new(id.clone(), name.clone(), 0);

    // Assert
    assert_eq!(column.id, id);
    assert_eq!(column.name, name);
    assert_eq!(column.order, 0);
    assert!(column.cards.is_empty());
}

#[test]
fn test_add_card() {
    // Arrange
    let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);

    // Act
    column.add_card("PRJ-001".to_string());
    column.add_card("PRJ-002".to_string());

    // Assert
    assert_eq!(column.cards.len(), 2);
    assert!(column.has_card("PRJ-001"));
    assert!(column.has_card("PRJ-002"));
}

#[test]
fn test_add_duplicate_card() {
    // Arrange
    let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);

    // Act
    column.add_card("PRJ-001".to_string());
    column.add_card("PRJ-001".to_string()); // Duplicate

    // Assert
    assert_eq!(column.cards.len(), 1);
}

#[test]
fn test_remove_card() {
    // Arrange
    let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);
    column.add_card("PRJ-001".to_string());
    column.add_card("PRJ-002".to_string());

    // Act
    let removed = column.remove_card("PRJ-001");

    // Assert
    assert!(removed);
    assert_eq!(column.cards.len(), 1);
    assert!(!column.has_card("PRJ-001"));
}

#[test]
fn test_remove_nonexistent_card() {
    // Arrange
    let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);
    column.add_card("PRJ-001".to_string());

    // Act
    let removed = column.remove_card("PRJ-999");

    // Assert
    assert!(!removed);
    assert_eq!(column.cards.len(), 1);
}

#[test]
fn test_move_card_up() {
    // Arrange
    let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);
    column.add_card("PRJ-001".to_string());
    column.add_card("PRJ-002".to_string());
    column.add_card("PRJ-003".to_string());

    // Act
    let moved = column.move_card_up("PRJ-002");

    // Assert
    assert!(moved);
    assert_eq!(column.cards, vec!["PRJ-002", "PRJ-001", "PRJ-003"]);
}

#[test]
fn test_move_card_up_at_top() {
    // Arrange
    let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);
    column.add_card("PRJ-001".to_string());
    column.add_card("PRJ-002".to_string());

    // Act
    let moved = column.move_card_up("PRJ-001");

    // Assert
    assert!(!moved);
    assert_eq!(column.cards, vec!["PRJ-001", "PRJ-002"]);
}

#[test]
fn test_move_card_up_not_found() {
    // Arrange
    let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);
    column.add_card("PRJ-001".to_string());

    // Act
    let moved = column.move_card_up("PRJ-999");

    // Assert
    assert!(!moved);
    assert_eq!(column.cards, vec!["PRJ-001"]);
}

#[test]
fn test_move_card_down() {
    // Arrange
    let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);
    column.add_card("PRJ-001".to_string());
    column.add_card("PRJ-002".to_string());
    column.add_card("PRJ-003".to_string());

    // Act
    let moved = column.move_card_down("PRJ-002");

    // Assert
    assert!(moved);
    assert_eq!(column.cards, vec!["PRJ-001", "PRJ-003", "PRJ-002"]);
}

#[test]
fn test_move_card_down_at_bottom() {
    // Arrange
    let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);
    column.add_card("PRJ-001".to_string());
    column.add_card("PRJ-002".to_string());

    // Act
    let moved = column.move_card_down("PRJ-002");

    // Assert
    assert!(!moved);
    assert_eq!(column.cards, vec!["PRJ-001", "PRJ-002"]);
}

#[test]
fn test_move_card_down_not_found() {
    // Arrange
    let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);
    column.add_card("PRJ-001".to_string());

    // Act
    let moved = column.move_card_down("PRJ-999");

    // Assert
    assert!(!moved);
    assert_eq!(column.cards, vec!["PRJ-001"]);
}

#[test]
fn test_multiple_card_moves() {
    // Arrange
    let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);
    column.add_card("PRJ-001".to_string());
    column.add_card("PRJ-002".to_string());
    column.add_card("PRJ-003".to_string());

    // Act - move PRJ-003 to the top
    column.move_card_up("PRJ-003");
    column.move_card_up("PRJ-003");

    // Assert
    assert_eq!(column.cards, vec!["PRJ-003", "PRJ-001", "PRJ-002"]);
}
