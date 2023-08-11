mod editor;
use crate::editor::Editor;
mod terminal;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
