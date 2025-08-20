use eyre::Result;
// use gladis4::Gladis;
use gtk4::{prelude::*, ListBox, SearchEntry, Window as GtkWindow};

use crate::scriptmap::ScriptMap;

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, RwLock},
};

// #[derive(Gladis, Clone)]
#[derive(Clone)]
pub struct DialogWidgets {
    pub dialog: GtkWindow,
    pub list_box: ListBox,
    pub search_bar: SearchEntry,
}

#[derive(Clone)]
pub struct Dialog {
    pub widgets: DialogWidgets,
    pub scripts: Arc<RwLock<ScriptMap>>,
    pub selected_script: Rc<RefCell<Option<String>>>,
    pub script_names: Rc<RefCell<Vec<String>>>,
}

impl Dialog {
    pub(crate) fn new<P: IsA<gtk4::Window>>(
        window: &P,
        scripts: &Arc<RwLock<ScriptMap>>,
    ) -> Result<Self> {
        // Use GTK4 Builder pattern to load from Glade file
        let builder = gtk4::Builder::from_resource("/fyi/zoey/Boop-GTK/command-palette.glade");

        let dialog: GtkWindow = builder.object("dialog").expect("Failed to get dialog");
        let list_box = builder.object("list_box").expect("Failed to get list_box");

        // Create search entry programmatically since Builder can't find it
        let search_bar = gtk4::SearchEntry::new();
        search_bar.set_placeholder_text(Some("Search scripts..."));
        search_bar.set_hexpand(true);
        search_bar.set_margin_bottom(6);

        // Get the main box and add the search entry at the top
        let main_box: gtk4::Box = dialog
            .child()
            .unwrap()
            .downcast()
            .expect("Main child should be a Box");
        main_box.prepend(&search_bar);

        let widgets = DialogWidgets {
            dialog,
            list_box,
            search_bar,
        };

        let command_palette_dialog = Dialog {
            widgets,
            scripts: scripts.clone(),
            selected_script: Rc::new(RefCell::new(None)),
            script_names: Rc::new(RefCell::new(Vec::new())),
        };

        command_palette_dialog
            .widgets
            .dialog
            .set_transient_for(Some(window));

        // Populate list box with scripts
        let scripts_ref = scripts.read().expect("scripts lock is poisoned");
        let mut script_names_vec = Vec::new();

        for (name, script) in &scripts_ref.0 {
            let row = gtk4::ListBoxRow::new();
            let label = gtk4::Label::new(Some(&script.metadata.name));
            row.set_child(Some(&label));

            script_names_vec.push(name.clone());
            command_palette_dialog.widgets.list_box.append(&row);
        }

        *command_palette_dialog.script_names.borrow_mut() = script_names_vec;
        drop(scripts_ref);

        command_palette_dialog.register_handlers();
        Ok(command_palette_dialog)
    }

    pub(crate) fn get_selected(&self) -> Option<String> {
        self.selected_script.borrow().clone()
    }

    fn register_handlers(&self) {
        let dialog = self.widgets.dialog.clone();
        let selected = self.selected_script.clone();
        let script_names = self.script_names.clone();

        self.widgets
            .list_box
            .connect_row_selected(move |_list_box, row| {
                if let Some(row) = row {
                    // Get the script name by index
                    let index = row.index();
                    if index >= 0 {
                        let names = script_names.borrow();
                        if let Some(script_name) = names.get(index as usize) {
                            *selected.borrow_mut() = Some(script_name.clone());
                            dialog.close();
                        }
                    }
                }
            });

        // Handle search functionality
        let list_box = self.widgets.list_box.clone();
        self.widgets.search_bar.connect_changed(move |entry| {
            let search_text = entry.text().to_lowercase();
            let mut index = 0;

            while let Some(row) = list_box.row_at_index(index) {
                if let Some(child) = row.child() {
                    if let Ok(label) = child.downcast::<gtk4::Label>() {
                        let text = label.text();
                        if !text.is_empty() {
                            let visible = search_text.is_empty()
                                || text.to_lowercase().contains(&search_text);
                            row.set_visible(visible);
                        }
                    }
                }
                index += 1;
            }
        });
    }

    // Simplified implementation without complex search/filtering for now

    pub fn show(&self) {
        self.widgets.dialog.show();
    }

    pub fn present(&self) {
        self.widgets.dialog.present();
        // Set focus to search bar after presenting
        self.widgets.search_bar.grab_focus();
    }

    pub fn close(&self) {
        self.widgets.dialog.close();
    }

    pub fn run_async<F>(&self, callback: F)
    where
        F: Fn(Option<String>) + 'static,
    {
        let selected = self.selected_script.clone();
        let dialog = self.widgets.dialog.clone();

        dialog.connect_close_request(move |_| {
            let selected_script = selected.borrow().clone();
            callback(selected_script);
            gtk4::glib::Propagation::Proceed
        });

        self.present();
        // Ensure search bar has focus
        self.widgets.search_bar.grab_focus();
    }
}
