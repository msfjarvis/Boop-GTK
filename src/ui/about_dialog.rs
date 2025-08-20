use std::sync::{Arc, RwLock};

use eyre::Result;
// use gladis4::Gladis;
use gtk4::prelude::*;

use crate::scriptmap::ScriptMap;

// #[derive(Gladis, Clone, Shrinkwrap)]
#[derive(Clone, Shrinkwrap)]
pub struct AboutDialog {
    #[shrinkwrap(main_field)]
    pub about_dialog: gtk4::AboutDialog,
}

impl AboutDialog {
    pub(crate) fn new(scripts: &Arc<RwLock<ScriptMap>>) -> Result<Self> {
        // Use GTK4 Builder pattern to load from Glade file
        let builder = gtk4::Builder::from_resource("/fyi/zoey/Boop-GTK/boop-gtk.glade");

        let dialog = AboutDialog {
            about_dialog: builder
                .object("about_dialog")
                .expect("Failed to get about_dialog"),
        };

        dialog
            .about_dialog
            .set_version(Some(env!("CARGO_PKG_VERSION")));

        for script in scripts.read().expect("Scripts lock is poisoned").0.values() {
            if let Some(author) = &script.metadata.author {
                dialog
                    .about_dialog
                    .add_credit_section(&format!("{} script", &script.metadata.name), &[author]);
            }
        }

        Ok(dialog)
    }

    pub fn present(&self) {
        self.about_dialog.present();
    }

    pub fn set_transient_for(&self, parent: Option<&impl IsA<gtk4::Window>>) {
        self.about_dialog.set_transient_for(parent);
    }
}
