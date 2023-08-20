mod editor;
use crate::editor::Editor;
mod document;
mod row;
mod terminal;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

fn main() -> io::Result<()> {
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut editor = Editor::default();
    editor.run();
    execute!(io::stdout(), LeaveAlternateScreen)
}
