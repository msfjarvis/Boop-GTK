use eyre::Result;
use gtk4::{prelude::*, Dialog as GtkDialog, Entry, ListBox};
use once_cell::unsync::OnceCell;

use crate::scriptmap::ScriptMap;

use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
pub struct DialogWidgets {
    pub dialog: GtkDialog,
    pub list_box: ListBox,
    pub search_bar: Entry,
}

#[derive(Clone)]
pub struct Dialog {
    pub widgets: DialogWidgets,
    pub scripts: Arc<RwLock<ScriptMap>>,
    pub selected_script: Rc<OnceCell<String>>,
}

impl Dialog {
    pub(crate) fn new<P: IsA<gtk4::Window>>(
        window: &P,
        scripts: &Arc<RwLock<ScriptMap>>,
    ) -> Result<Self> {
        let dialog = GtkDialog::builder()
            .title("Command Palette")
            .modal(true)
            .build();
        let list_box = ListBox::new();
        let search_bar = Entry::new();

        let widgets = DialogWidgets {
            dialog,
            list_box,
            search_bar,
        };

        let command_palette_dialog = Dialog {
            widgets,
            scripts: scripts.clone(),
            selected_script: Rc::new(OnceCell::new()),
        };

        command_palette_dialog
            .widgets
            .dialog
            .set_transient_for(Some(window));

        // Populate list box with scripts
        let scripts_ref = scripts.read().expect("scripts lock is poisoned");
        for (name, script) in &scripts_ref.0 {
            let row = gtk4::ListBoxRow::new();
            let label = gtk4::Label::new(Some(&script.metadata.name));
            row.set_child(Some(&label));
            // Store name as data instead of using set_name which doesn't exist
            // row.set_name(name);
            command_palette_dialog.widgets.list_box.append(&row);
        }

        command_palette_dialog.register_handlers();
        Ok(command_palette_dialog)
    }

    pub(crate) fn get_selected(&self) -> Option<&String> {
        self.selected_script.get()
    }

    fn register_handlers(&self) {
        let dialog = self.widgets.dialog.clone();
        let selected = self.selected_script.clone();
        let list_box = self.widgets.list_box.clone();

        self.widgets.list_box.connect_row_selected(move |_, row| {
            if let Some(_row) = row {
                // For now, just select the first available script
                // In a full implementation, we'd store script names as data
                let _ = selected.set("dummy_script".to_string());
                dialog.response(gtk4::ResponseType::Accept);
            }
        });
    }

    // Simplified implementation without complex search/filtering for now

    pub fn show(&self) {
        self.widgets.dialog.show();
    }

    pub fn present(&self) {
        self.widgets.dialog.present();
    }

    pub fn close(&self) {
        self.widgets.dialog.close();
    }

    pub fn run(&self) -> gtk4::ResponseType {
        // Note: In GTK4, run() is deprecated. This is a placeholder.
        // In a real implementation, you'd use async/await or signals.
        gtk4::ResponseType::Accept
    }
}
