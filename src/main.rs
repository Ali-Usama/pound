// use std::{io, io::Read};
use crossterm::{terminal, event};
use crossterm::event::{Event, KeyCode, KeyEvent};
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not turn off raw mode")
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;

    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('q', ..),
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => break,
                    _ => {
                        // todo!()
                    }
                }
                println!("{:?}\r", event);
            };
        } else {
            println!("No input yet\r");
        }
    }

    Ok(())


    // 1 byte to read from the terminal
    // let mut buf = [0; 1];
    // while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {
    //     let character = buf[0] as char;
    //
    //     // control characters are non-printable characters that we don't want to print to the screen
    //     if character.is_control() {
    //         println!("{}\r", character as u8)
    //     } else {
    //         println!("{}\r", character)
    //     }
    // }
}
