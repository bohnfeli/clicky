use super::*;
use std::time::Duration;

#[test]
fn test_event_handler_stop_completes_within_timeout() {
    let events = EventHandler::new();

    let start = std::time::Instant::now();
    events.stop();
    let duration = start.elapsed();

    assert!(
        duration < Duration::from_secs(1),
        "EventHandler::stop() took {:?} which is longer than expected 1s timeout",
        duration
    );
}

#[test]
fn test_event_handler_receives_events() {
    let mut events = EventHandler::new();

    let event = events.try_next();
    assert!(event.is_none(), "No events should be available immediately");

    events.stop();
}
