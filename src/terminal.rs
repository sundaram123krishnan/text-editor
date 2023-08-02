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
    stdout: RawTerminal<io::Stdout>,
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
    pub fn clear_screen(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
    }

    pub fn cursor_position(&mut self, w: u16, h: u16) {
        let w = w.saturating_add(1);
        let h = h.saturating_add(1);
        write!(self.stdout, "{}", termion::cursor::Goto(w, h)).unwrap();
    }
    pub fn flush_output(&mut self) {
        self.stdout.flush().unwrap();
    }
}
