use std::io::stdin;
use std::process::exit;
use termion::{event::Key, input::TermRead};

pub fn check_input() {
    for c in stdin().keys() {
        let character = c.unwrap_or_else(|err| {
            println!("ERROR: error reading character {}", err);
            exit(1);
        });

        match character {
            Key::Ctrl('q') => break,
            Key::Char(c) => {
                println!("{}\r", c);
            }
            Key::Ctrl(c) => {
                println!("Ctrl-{}\r", c);
            }
            Key::Alt(c) => println!("Alt-{}\r", c),
            Key::Left => println!("<left>\r"),
            Key::Right => println!("<right>\r"),
            Key::Up => println!("<up>\r"),
            Key::Down => println!("<down>\r"),
            Key::Backspace => println!("<backspace>\r"),
            _ => {
                println!("other");
            }
        }
    }
}
