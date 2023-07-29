use colored::*;
use crossterm::cursor;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use terminal_size::{terminal_size, Height, Width};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

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
    write!(
        stdout(),
        "{} {}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    let (_w, h) = get_terminal_size();

    for _ in 0..h {
        println!("{t}\r", t = "~".yellow().bold());
    }
}

// pub fn print_file(file_name: &mut String) {
//     let mut stdout = stdout().into_raw_mode().unwrap();
//     tilde();
//     for i in file_name.lines() {
//         write!(stdout, "{} {i}", crossterm::cursor::MoveTo(2, col)).unwrap();
//         write!(stdout, "\r").unwrap();
//     }
//     read_keys();
//     stdout.flush().unwrap();
// }

pub fn read_keys() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (w, _h) = get_terminal_size();
    tilde();
    print!("{}", cursor::MoveTo(2, 0));

    for c in stdin().keys() {
        let (row, col) = cursor::position().unwrap();

        let c = c.unwrap_or_else(|err| {
            println!("Error parsin keys {}", err);
            exit(1);
        });

        match c {
            Key::Char('\n') => {
                write!(stdout, "{}", cursor::MoveTo(2, col + 1)).unwrap();
            }
            Key::Char(c) => {
                if row <= w - 2 {
                    write!(stdout, "{c}").unwrap();
                    write!(stdout, "{}", cursor::EnableBlinking).unwrap();
                } else {
                    write!(stdout, "{}", cursor::MoveTo(2, col + 1)).unwrap();
                    write!(stdout, "{c}").unwrap();
                }
            }
            Key::Up => {
                write!(stdout, "{}", cursor::MoveUp(1)).unwrap();
            }
            Key::Down => {
                write!(stdout, "{}", cursor::MoveDown(1)).unwrap();
            }
            Key::Left => {
                write!(stdout, "{}", cursor::MoveLeft(1)).unwrap();
            }
            Key::Right => {
                write!(stdout, "{}", cursor::MoveRight(1)).unwrap();
            }
            Key::Backspace => {
                write!(stdout, "{}", cursor::MoveLeft(1)).unwrap();
                write!(stdout, " ").unwrap();
                write!(stdout, "{}", cursor::MoveLeft(1)).unwrap();
            }
            Key::Ctrl('q') => {
                break;
            }

            _other => {
                write!(stdout, "other").unwrap();
            }
        }
        stdout.flush().unwrap();
    }
}
