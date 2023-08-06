use crate::editor::Position;
use std::io::{self, stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    pub stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        let (x, y) = size;
        Ok(Self {
            size: Size {
                width: x,
                height: y,
            },
            stdout: stdout().into_raw_mode().unwrap(),
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn read_key() -> Key {
        loop {
            let key = stdin().keys().next().unwrap();
            return key.unwrap();
        }
    }

    pub fn start(&mut self) {
        writeln!(self.stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
    }

    pub fn clear_screen(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
    }

    pub fn cursor_position(&mut self, pos: &Position) {
        let mut w = pos.x;
        let mut h = pos.y;
        w = w.saturating_add(1);
        h = w.saturating_add(1);
        write!(self.stdout, "{}", termion::cursor::Goto(w as u16, h as u16)).unwrap();
    }
    pub fn flush_output(&mut self) {
        self.stdout.flush().unwrap();
    }
}
