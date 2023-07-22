use std::io::{stdin, stdout};
use std::process::exit;
use terminal_size::{terminal_size, Height, Width};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn quit() -> Key {
    Key::Ctrl('q')
}

fn tilde() {
    print!("{} {}", termion::clear::All, termion::cursor::Goto(1, 1));
    let size = terminal_size();

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
    let _stdout = stdout().into_raw_mode().unwrap();
    tilde();
    let mut i = 1;
    for c in stdin().keys() {
        let c = c.unwrap_or_else(|_err| {
            print!("{} {}", termion::clear::All, termion::cursor::Goto(1, 1));
            exit(1);
        });
        let q = quit();
        if c == q {
            println!("Goodbye!");
            exit(1);
        } else {
            println!("{cursor} {:?}\r", c, cursor = termion::cursor::Goto(2, i));
            i += 1;
        }
    }
}
