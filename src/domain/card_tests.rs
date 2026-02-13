use super::*;

#[test]
fn test_card_creation() {
    // Arrange
    let id = "PRJ-001".to_string();
    let title = "Test Task".to_string();
    let column_id = "todo".to_string();

    // Act
    let card = Card::new(id.clone(), title.clone(), column_id.clone());

    // Assert
    assert_eq!(card.id, id);
    assert_eq!(card.title, title);
    assert_eq!(card.column_id, column_id);
    assert!(card.description.is_none());
    assert!(card.assignee.is_none());
}

#[test]
fn test_card_move() {
    // Arrange
    let mut card = Card::new(
        "PRJ-002".to_string(),
        "Task".to_string(),
        "todo".to_string(),
    );

    // Act
    card.move_to("in_progress".to_string());

    // Assert
    assert_eq!(card.column_id, "in_progress");
}

#[test]
fn test_card_update_title() {
    // Arrange
    let mut card = Card::new(
        "PRJ-003".to_string(),
        "Old Title".to_string(),
        "todo".to_string(),
    );

    // Act
    card.set_title("New Title".to_string());

    // Assert
    assert_eq!(card.title, "New Title");
}

#[test]
fn test_card_set_assignee() {
    // Arrange
    let mut card = Card::new(
        "PRJ-004".to_string(),
        "Task".to_string(),
        "todo".to_string(),
    );

    // Act
    card.set_assignee(Some("Alice".to_string()));

    // Assert
    assert_eq!(card.assignee, Some("Alice".to_string()));
}
