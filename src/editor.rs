use crate::terminal::Terminal;
use colored::Colorize;
use std::io::Write;
use std::process::exit;
use termion::event::Key;

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    quit: bool,
    terminal: Terminal,
    welcome_message: String,
    cursor_pos: Position,
}

impl Editor {
    pub fn run(&mut self) {
        let mut ok: bool = false;
        self.terminal.clear_screen();
        self.tilde();
        self.terminal.start();
        self.terminal.cursor_position(&Position { x: 2, y: 0 });
        loop {
            if ok == false {
                self.clear_message();
                ok = true;
                self.terminal.cursor_position(&Position { x: 2, y: 0 });
            }

            let pressed_key = self.process_keys();

            if self.quit == true {
                exit(1);
            } else {
                if let Some(_c) = pressed_key {
                    let pressed_key = match pressed_key {
                        Some(pressed_key) => pressed_key,
                        None => exit(1),
                    };
                    print!("{pressed_key}");
                }
                self.terminal.flush_output();
            }
        }
    }

    fn tilde(&mut self) {
        let size = self.terminal.size();
        for _ in 0..size.height {
            println!("~\r");
        }

        let display_h = size.height / 3;
        let display_w = size.width / 4;

        self.terminal.cursor_position(&Position {
            x: display_w as usize,
            y: display_h as usize,
        });
        let value = format!("{}", self.welcome_message);
        println!("{}", value.blue().bold());

        self.terminal.cursor_position(&Position { x: 2, y: 0 });
    }

    fn process_keys(&mut self) -> Option<char> {
        let key_pressed = Terminal::read_key();
        match key_pressed {
            termion::event::Key::Ctrl('q') => {
                self.quit = true;
                return None;
            }
            Key::Char(c) => return Some(c),
            _ => return Some('c'),
        }
    }

    fn clear_message(&mut self) {
        let size = self.terminal.size();
        let display_h = size.height / 3;
        let display_w = size.width / 4;
        self.welcome_message.clear();
        self.terminal.cursor_position(&Position {
            x: display_w as usize,
            y: display_h as usize,
        });
        write!(
            self.terminal.stdout,
            "                                                                  "
        )
        .unwrap();
    }

    pub fn default() -> Self {
        Self {
            quit: false,
            terminal: Terminal::default().unwrap(),
            welcome_message: String::from("Welcome to Linote -- Version 0.0.1"),
            cursor_pos: Position { x: 0, y: 0 },
        }
    }
}
