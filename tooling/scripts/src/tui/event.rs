//! TUI event handling

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

use super::app::App;

/// Application events
pub enum AppEvent {
    Key(KeyEvent),
    Tick,
    Resize,
}

/// Event handler
pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    /// Create a new event handler
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            tick_rate: Duration::from_millis(tick_rate_ms),
        }
    }

    /// Poll for the next event
    pub fn next(&self) -> std::io::Result<AppEvent> {
        if event::poll(self.tick_rate)? {
            match event::read()? {
                Event::Key(key) => Ok(AppEvent::Key(key)),
                Event::Resize(_, _) => Ok(AppEvent::Resize),
                _ => Ok(AppEvent::Tick),
            }
        } else {
            Ok(AppEvent::Tick)
        }
    }
}

/// Handle a key event
pub fn handle_key(app: &mut App, key: KeyEvent) -> bool {
    // Ctrl+C or 'q' to quit
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.should_quit = true;
        return true;
    }

    match key.code {
        KeyCode::Char('q') => {
            app.should_quit = true;
            true
        }
        KeyCode::Char('j') | KeyCode::Down => {
            app.select_next();
            false
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.select_previous();
            false
        }
        KeyCode::Tab => {
            app.toggle_focus();
            false
        }
        KeyCode::Enter => {
            app.run_selected_task();
            false
        }
        _ => false,
    }
}
