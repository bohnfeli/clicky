//! Event handling for TUI.

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

pub enum Event {
    Input(KeyEvent),
    Tick,
}

pub struct EventHandler {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
    handler: Option<thread::JoinHandle<()>>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();

        let sender_clone = sender.clone();
        let handler = thread::spawn(move || {
            let tick_rate = Duration::from_millis(250);
            let mut last_tick = Instant::now();

            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).unwrap_or(false) {
                    match event::read() {
                        Ok(CrosstermEvent::Key(key)) => {
                            if sender_clone.send(Event::Input(key)).is_err() {
                                break;
                            }
                        }
                        Ok(_) => {}
                        Err(_) => break,
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if sender_clone.send(Event::Tick).is_err() {
                        break;
                    }
                    last_tick = Instant::now();
                }
            }
        });

        Self {
            sender,
            receiver,
            handler: Some(handler),
        }
    }

    pub fn next(&mut self) -> Result<Event, std::io::Error> {
        self.receiver.recv().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                format!("Event channel error: {}", e),
            )
        })
    }

    pub fn try_next(&mut self) -> Option<Event> {
        match self.receiver.try_recv() {
            Ok(event) => Some(event),
            Err(TryRecvError::Disconnected) => None,
            Err(TryRecvError::Empty) => None,
        }
    }

    pub fn stop(mut self) {
        if let Some(handler) = self.handler.take() {
            let _ = handler.join();
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
