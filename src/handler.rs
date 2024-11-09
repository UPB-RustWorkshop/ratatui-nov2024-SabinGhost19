use crate::app::{App, AppResult};
use crossterm::event::{KeyEvent, KeyCode};
/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // TODO: define actions for quitting the app
        // TODO: define actions for apps functionalities
        KeyCode::Char('q') => {
            app.running = false;
        }
        KeyCode::Char('h') => {
            println!("Help: Press 'q' to quit, 'h' for help.");
        }
        KeyCode::Up => {
            app.previous_city();
        }
        KeyCode::Down => {
            app.next_city(); 
        }
        KeyCode::Enter => {
            //? NOT IMPLEMENTED YET
        }
        
        _ => {}
    }
    Ok(())
}
