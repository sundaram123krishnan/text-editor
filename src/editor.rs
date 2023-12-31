use crate::document::Document;
use crate::row::Row;
use crate::terminal::{self, Terminal};
use colored::Colorize;
use std::env;
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
    document: Document,
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
                                print!("{}", c);
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

    fn move_cursor_right(&mut self) {
        if self.cursor_pos.x == 112 {
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
            self.cursor_pos.x = 112;
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
        print!("{}", termion::scroll::Up(1));
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
        print!("{}", termion::scroll::Down(1));
    }

    fn write_single_row(&self, row: &Row) {
        for i in row.string.lines() {
            println!("{i}\r");
        }
    }

    fn tilde(&mut self) {
        // for _ in 0.. {
        //     println!("~\r");
        // }
        print!("{}", termion::cursor::Goto(3, 0));

        for i in &self.document.rows {
            self.write_single_row(i);
        }
        //
        // let display_h = 42 / 3;
        // let display_w = 90 / 4;
        //
        // self.terminal
        //     .cursor_position(display_w as usize, display_h as usize);
        //
        // let value = self.welcome_message.to_string();
        // println!("{}", value.blue().bold());

        self.terminal.cursor_position(3, 0);
    }

    fn process_keys(&mut self) -> Option<Key> {
        let key_pressed = Terminal::read_key();
        match key_pressed {
            termion::event::Key::Ctrl('q') => {
                self.quit = true;
                None
            }
            Key::Char(c) => Some(Key::Char(c)),
            Key::Right => Some(Key::Right),
            Key::Left => Some(Key::Left),
            Key::Up => Some(Key::Up),
            Key::Down => Some(Key::Down),
            _ => None,
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
        let args: Vec<String> = env::args().collect();
        let filename = args.get(1);
        let filename = match filename {
            Some(f) => &f[..],
            None => exit(1),
        };

        Self {
            quit: false,
            terminal: Terminal::default().unwrap(),
            welcome_message: String::from("Welcome to Linote -- Version 0.0.1"),
            document: Document::open(filename),
            cursor_pos: Position { x: 0, y: 0 },
        }
    }
}
