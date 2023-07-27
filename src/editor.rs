use colored::*;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use terminal_size::{terminal_size, Height, Width};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn quit() -> Key {
    Key::Ctrl('q')
}

fn enter() -> Key {
    Key::Char('\n')
}

fn up_arrow() -> Key {
    Key::Up
}

fn left_arrow() -> Key {
    Key::Left
}

fn right_arrow() -> Key {
    Key::Right
}

fn down_arrow() -> Key {
    Key::Down
}

fn backspace() -> Key {
    Key::Backspace
}

fn get_terminal_size() -> (u16, u16) {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        return (w, h);
    } else {
        println!("Unable to get terminal size");
        exit(2);
    }
}

fn tilde() {
    print!("{} {}", termion::clear::All, termion::cursor::Goto(1, 1));

    let (_w, h) = get_terminal_size();

    for _ in 0..h {
        println!("{t}\r", t = "~".yellow().bold());
    }
}

pub fn read_keys() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut j = 2;
    let (w, _h) = get_terminal_size();
    let i = w;
    tilde();
    print!("{}", termion::cursor::Goto(3, 1));
    let mut shadow_i = 3;

    for c in stdin().keys() {
        let c = c.unwrap_or_else(|err| {
            println!("Error parsin keys {}", err);
            exit(1);
        });

        let q = quit();
        let e = enter();
        let up_arrow = up_arrow();
        let down_arrow = down_arrow();
        let left_arrow = left_arrow();
        let right_arrow = right_arrow();
        let backspace = backspace();

        if c == q {
            exit(1);
        } else if c == e {
            write!(stdout, "{}", termion::cursor::Goto(3, j)).unwrap();
            j += 1;
            shadow_i = 4;
        } else if c == up_arrow {
            write!(stdout, "{}", termion::cursor::Up(1)).unwrap();
            j -= 1;
        } else if c == down_arrow {
            write!(stdout, "{}", termion::cursor::Down(1)).unwrap();
            j += 1;
        } else if c == left_arrow {
            write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
        } else if c == right_arrow {
            write!(stdout, "{}", termion::cursor::Right(1)).unwrap();
        } else if c == backspace {
            write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
            write!(stdout, " ").unwrap();
            write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
        } else {
            let c = match c {
                Key::Char(c) => c,
                _ => 'q',
            };
            if shadow_i <= i {
                write!(stdout, "{c}").unwrap();
                write!(stdout, "{}", termion::cursor::BlinkingUnderline).unwrap();
                shadow_i += 1;
            } else {
                write!(stdout, "{}", termion::cursor::Goto(3, j)).unwrap();
                write!(stdout, "{c}").unwrap();
                shadow_i = 4;
                j += 1;
            }
        }

        stdout.flush().unwrap();
    }
}
