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

fn tilde() {
    let size = terminal_size();
    print!("{} {}", termion::clear::All, termion::cursor::Goto(1, 1));

    if let Some((Width(_w), Height(h))) = size {
        for _ in 0..h {
            println!("~\r");
        }
    } else {
        println!("Unable to get terminal size");
        exit(2);
    }
}

pub fn read_keys() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let size = terminal_size();
    tilde();
    let mut i = 0;
    let mut j = 2;
    let mut enter_line = 1;
    print!("{}", termion::cursor::Goto(2, 1));

    let mut width = 0;

    if let Some((Width(w), Height(_h))) = size {
        width = w;
    }

    for c in stdin().keys() {
        let c = c.unwrap_or_else(|_err| {
            print!("{} {}", termion::clear::All, termion::cursor::Goto(1, 1));
            exit(1);
        });

        if i < width / 2 {
            write!(stdout, "{}", termion::cursor::Right(1)).unwrap();
        } else {
            write!(stdout, "{}", termion::cursor::Goto(2, j)).unwrap();
            i = 0;
            j += 1;
        }

        let q = quit();
        let e = enter();

        if c == e {
            write!(stdout, "{}", termion::cursor::Goto(2, enter_line)).unwrap();
            enter_line += 1;
            j = enter_line + 1;
        }
        if c == q {
            exit(1);
        } else {
            let c = match c {
                Key::Char(c) => c,
                _ => 'q',
            };
            print!("{c}");
            i += 1;
        }

        stdout.flush().unwrap();
    }
}
