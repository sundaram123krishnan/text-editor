use crate::terminal::Terminal;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(
            stdout,
            "{} {}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();
        self.tilde();
        write!(stdout, "{}", cursor::Goto(3, 1)).unwrap();
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

    fn tilde(&mut self) {
        let size = self.terminal.size();
        for _ in 0..size.height {
            println!("~\r");
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
        Self {
            quit: false,
            terminal: Terminal::default().unwrap(),
        }
    }
}

fn read_key() -> Key {
    loop {
        let key = stdin().keys().next().unwrap();
        return key.unwrap();
    }
}
