use std::io::{stdin, stdout, Write};
use std::process::exit;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        loop {
            let pressed_key = self.process_keys();

            if self.quit == true {
                exit(1);
            } else {
                if let Some(_c) = pressed_key {
                    let pressed_key = match pressed_key {
                        Some(pressed_key) => pressed_key,
                        None => exit(1),
                    };
                    write!(stdout, "{pressed_key}").unwrap();
                }
            }
            stdout.flush().unwrap();
        }
    }

    fn process_keys(&mut self) -> Option<char> {
        let key_pressed = read_key();
        match key_pressed {
            Key::Ctrl('q') => {
                self.quit = true;
                return None;
            }
            Key::Char(c) => return Some(c),
            _ => return Some('c'),
        }
    }

    pub fn default() -> Self {
        Self { quit: false }
    }
}

fn read_key() -> Key {
    loop {
        let key = stdin().keys().next().unwrap();
        return key.unwrap();
    }
}
