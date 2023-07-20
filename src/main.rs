mod editor;
use editor::check_input;
use std::io::stdout;
use termion::raw::IntoRawMode;
fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    check_input();
}
