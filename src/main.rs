use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use snek::app::{App, AppResult};
use snek::event::{Event, EventHandler};
use snek::tui::Tui;
use std::io;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(150);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => app.handle_input(key_event)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => app.handle_resize()?,
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
