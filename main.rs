use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // enable raw mode to get input without pressing enter
    enable_raw_mode()?;

    // print instructions
    println!("press 'q' to exit");

    loop {

        // poll for events (non-blocking check)
        if event::poll(std::time::Duration::from_millis(10))? {

            // read the event if available
            if let Event::Key(key_event) = event::read()? {

                // we only care about keypress events (not keyrelease or repeat)
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            // exit the loop
                            break;                         
                        }
                        KeyCode::Up => {
                            println!("Up arrow pressed!");
                        }
                        KeyCode::Down => {
                            println!("Down arrow pressed!");
                        }
                        KeyCode::Char(c) => {
                            println!("Key '{}' pressed!", c);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Disable raw mode before exiting
    disable_raw_mode()?;
    Ok(())
}
