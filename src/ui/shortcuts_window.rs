use gtk4::{prelude::*, ShortcutsWindow as GtkShortcutsWindow};

#[derive(Shrinkwrap)]
pub struct ShortcutsWindow {
    #[shrinkwrap(main_field)]
    window: GtkShortcutsWindow,
}

const GENERAL_SHORTCUTS: [(&str, &str); 3] = [
    ("Open Command Pallette", "<Primary><Shift>P"),
    ("Quit", "<Primary>Q"),
    ("Re-execute Last Script", "<Primary><Shift>B"),
];

const EDITOR_SHORTCUTS: [(&str, &str); 12] = [
    ("Undo", "<Primary>Z"),
    ("Redo", "<Primary><Shift>Z"),
    ("Move line up", "<Alt>Up"),
    ("Move line down", "<Alt>Down"),
    ("Move cursor backwards one word", "<Primary>Left"),
    ("Move cursor forward one word", "<Primary>Right"),
    ("Move cursor to beginning of previous line", "<Primary>Up"),
    ("Move cursor to end of next line", "<Primary>Down"),
    ("Move cursor to beginning of line", "<Primary>Page_Up"),
    ("Move cursor to end of line", "<Primary>Page_Down"),
    ("Move cursor to beginning of document", "<Primary>Home"),
    ("Move cursor to end of document", "<Primary>End"),
];

impl ShortcutsWindow {
    pub fn new() -> ShortcutsWindow {
        let window = gtk4::ShortcutsWindow::builder().build();

        let general_group = gtk4::ShortcutsGroup::builder().title("General").build();
        for (title, accelerator) in &GENERAL_SHORTCUTS {
            general_group.append(
                &gtk4::ShortcutsShortcut::builder()
                    .title(*title)
                    .accelerator(*accelerator)
                    .visible(true)
                    .build(),
            );
        }

        let editor_group = gtk4::ShortcutsGroup::builder().title("Editor").build();
        for (title, accelerator) in &EDITOR_SHORTCUTS {
            editor_group.append(
                &gtk4::ShortcutsShortcut::builder()
                    .title(*title)
                    .accelerator(*accelerator)
                    .visible(true)
                    .build(),
            );
        }

        let section = gtk4::ShortcutsSection::builder().build();
        section.append(&general_group);
        section.append(&editor_group);
        section.show();
        window.set_child(Some(&section));

        ShortcutsWindow { window }
    }
}
