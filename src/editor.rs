use crate::terminal::Terminal;
use std::process::exit;
use termion::event::Key;

pub struct Editor {
    quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        let mut ok: bool = false;
        self.terminal.clear_screen();
        self.tilde();
        self.terminal.start();
        self.terminal.cursor_position(2, 0);
        loop {
            if ok == false {
                let size = self.terminal.size();
                let display_h = size.height / 3;
                let display_w = size.width / 3;
                self.terminal.cursor_position(display_w, display_h);
                print!("{}", termion::clear::CurrentLine);
                ok = true;
                self.terminal.cursor_position(2, 0);
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

        self.terminal.cursor_position(display_w, display_h);
        println!("Welcome to LinNote editor -- Version 0.0.1. Create, Edit and Delete");
        self.terminal.cursor_position(2, 0);
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

    pub fn default() -> Self {
        Self {
            quit: false,
            terminal: Terminal::default().unwrap(),
        }
    }
}
