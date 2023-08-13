mod editor;
use crate::editor::Editor;
mod document;
mod row;
mod terminal;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
