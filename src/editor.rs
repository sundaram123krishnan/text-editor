use crate::terminal::Terminal;
use colored::Colorize;
use std::io::Write;
use std::process::exit;
use termion::event::Key;

pub struct Position {
    x: usize,
    y: usize,
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
        self.terminal.cursor_position(2, 0);
        let Position { x: _, y: _ } = self.cursor_pos;
        self.cursor_pos.x = 2;
        self.cursor_pos.y = 0;
        loop {
            if !ok {
                self.clear_message();
                ok = true;
                self.terminal.cursor_position(2, 0);
                self.cursor_pos.x = 2;
                self.cursor_pos.y = 1;
            } else {
                let pressed_key = self.process_keys();

                if self.quit {
                    exit(1);
                } else {
                    if let Some(_c) = pressed_key {
                        let pressed_key = match pressed_key {
                            Some(pressed_key) => pressed_key,
                            None => exit(1),
                        };
                        match pressed_key {
                            Key::Char(c) => {
                                print!("{c}");
                                self.print_char();
                            }

                            Key::Right => {
                                self.move_cursor_right();
                            }

                            Key::Left => {
                                self.move_cursor_left();
                            }

                            Key::Up => {
                                self.move_cursor_up();
                            }

                            Key::Down => {
                                self.move_cursor_down();
                            }
                            _ => todo!(),
                        }
                    }
                    self.terminal.flush_output();
                }
            }
        }
    }

    fn print_char(&mut self) {
        if self.cursor_pos.x == 89 {
            self.cursor_pos.x = 3;
            self.cursor_pos.y += 1;
            write!(
                self.terminal.stdout,
                "{}",
                termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16)
            )
            .unwrap();
        } else {
            self.cursor_pos.x += 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_pos.x == 89 {
            self.cursor_pos.x = 3;
            self.cursor_pos.y += 1;
            write!(
                self.terminal.stdout,
                "{}",
                termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16)
            )
            .unwrap();
        } else {
            self.cursor_pos.x += 1;
            write!(
                self.terminal.stdout,
                "{}",
                termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16)
            )
            .unwrap();
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_pos.x == 3 && self.cursor_pos.y == 1 {
            self.cursor_pos.x = 3;
            self.cursor_pos.y = 1;
        } else if self.cursor_pos.x == 3 {
            self.cursor_pos.y -= 1;
            self.cursor_pos.x = 89;
            write!(
                self.terminal.stdout,
                "{}",
                termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16)
            )
            .unwrap();
        } else {
            self.cursor_pos.x -= 1;
            if self.cursor_pos.x <= 2 {
                self.cursor_pos.x = 2;
            }
            write!(
                self.terminal.stdout,
                "{}",
                termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16)
            )
            .unwrap();
        }
    }

    fn move_cursor_down(&mut self) {
        self.cursor_pos.y += 1;
        write!(
            self.terminal.stdout,
            "{}",
            termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16)
        )
        .unwrap();
    }

    fn move_cursor_up(&mut self) {
        self.cursor_pos.y -= 1;
        if self.cursor_pos.y <= 1 {
            self.cursor_pos.y = 1;
        }
        write!(
            self.terminal.stdout,
            "{}",
            termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16)
        )
        .unwrap();
    }

    fn tilde(&mut self) {
        let size = self.terminal.size();
        for _ in 0..size.height {
            println!("~\r");
        }

        let display_h = size.height / 3;
        let display_w = size.width / 4;

        self.terminal
            .cursor_position(display_w as usize, display_h as usize);

        let value = format!("{}", self.welcome_message);
        println!("{}", value.blue().bold());

        self.terminal.cursor_position(2, 0);
    }

    fn process_keys(&mut self) -> Option<Key> {
        let key_pressed = Terminal::read_key();
        match key_pressed {
            termion::event::Key::Ctrl('q') => {
                self.quit = true;
                return None;
            }
            Key::Char(c) => return Some(Key::Char(c)),
            Key::Right => {
                return Some(Key::Right);
            }
            Key::Left => {
                return Some(Key::Left);
            }
            Key::Up => {
                return Some(Key::Up);
            }
            Key::Down => {
                return Some(Key::Down);
            }

            _ => return Some(Key::PageUp),
        }
    }

    fn clear_message(&mut self) {
        let size = self.terminal.size();
        let display_h = size.height / 3;
        let display_w = size.width / 4;
        self.welcome_message.clear();
        self.terminal
            .cursor_position(display_w as usize, display_h as usize);
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
