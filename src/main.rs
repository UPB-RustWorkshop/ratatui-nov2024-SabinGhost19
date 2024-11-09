use crossterm::event::{self, Event, KeyEvent, KeyCode};
use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
  
    let mut app = App::new();
    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    
  
    // Initialize the terminal.
    let mut event_handler = EventHandler::new(250);
    let mut tui = Tui::new(terminal, event_handler);

     // init the terminal
     tui.init()?;

     println!("Rendering the terminal interface. Press 'q' to quit.");
    // Start the main loop.
    while app.running {
        // Render the user interface.

        tui.draw(&mut app)?;
        
        if let Event::Key(key_event) = event::read()? {
            handle_key_events(key_event, &mut app)?;
        }
      
        }
 
    tui.exit()?;

    Ok(())
}

