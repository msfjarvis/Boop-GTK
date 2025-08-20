use std::sync::{Arc, RwLock};

use eyre::Result;
// use gladis4::Gladis;

use gtk4::{prelude::*, Switch, Window as GtkWindow};
use sourceview5::StyleScheme;

use crate::config::Config;

// #[derive(Gladis, Clone, Shrinkwrap)]
#[derive(Clone, Shrinkwrap)]
pub struct Widgets {
    #[shrinkwrap(main_field)]
    pub preference_dialog: GtkWindow,

    pub color_scheme_button: gtk4::Button,
    pub shortcut_switch: Switch,
    pub preference_cancel_button: gtk4::Button,
    pub preference_ok_button: gtk4::Button,
}

// #[derive(Clone, Shrinkwrap)]
#[derive(Clone, Shrinkwrap)]
pub struct Dialog {
    #[shrinkwrap(main_field)]
    pub widgets: Widgets,
    pub config: Arc<RwLock<Config>>,
}

impl Dialog {
    pub(crate) fn new(config: &Arc<RwLock<Config>>) -> Result<Self> {
        // Use GTK4 Builder pattern to load from Glade file
        let builder = gtk4::Builder::from_resource("/fyi/zoey/Boop-GTK/boop-gtk.glade");

        let widgets = Widgets {
            preference_dialog: builder
                .object("preference_dialog")
                .expect("Failed to get preference_dialog"),
            color_scheme_button: builder
                .object("color_scheme_button")
                .expect("Failed to get color_scheme_button"),
            shortcut_switch: builder
                .object("shortcut_switch")
                .expect("Failed to get shortcut_switch"),
            preference_cancel_button: builder
                .object("preference_cancel_button")
                .expect("Failed to get preference_cancel_button"),
            preference_ok_button: builder
                .object("preference_ok_button")
                .expect("Failed to get preference_ok_button"),
        };

        let mut dialog = Dialog {
            widgets,
            config: config.clone(),
        };

        dialog.update_state_from_config()?;
        dialog.connect_config_style_scheme_notify(Dialog::on_config_style_scheme_notify(
            config.clone(),
        ));
        dialog.connect_config_open_shortcuts_on_startup_notify(
            Dialog::on_config_open_shortcuts_on_startup_notify(config.clone()),
        );
        dialog.connect_buttons();

        Ok(dialog)
    }

    // update the controls with values from config
    pub fn update_state_from_config(&mut self) -> Result<()> {
        let config = self
            .config
            .read()
            .map_err(|e| eyre!("Config lock poisoned: {}", e))?;

        // update color_scheme_button
        let scheme_id = &config.editor.colour_scheme_id;
        let scheme = sourceview5::StyleSchemeManager::default()
            .scheme(scheme_id)
            .ok_or_else(|| eyre!("StyleSchemeManager could not find scheme '{}'", scheme_id))?;
        // Update button label to show current scheme
        let scheme_name = scheme.name().to_string();
        self.widgets
            .color_scheme_button
            .set_label(&format!("Color Scheme: {}", scheme_name));

        // update shortcut_switch
        self.widgets
            .shortcut_switch
            .set_state(config.show_shortcuts_on_open);

        Ok(())
    }

    fn on_config_style_scheme_notify(config: Arc<RwLock<Config>>) -> impl Fn(Option<StyleScheme>) {
        move |scheme: Option<StyleScheme>| {
            if let Some(scheme) = scheme {
                let scheme_id = scheme.id();
                let mut config = config.write().expect("Config lock poisoned");
                config.editor.set_colour_scheme_id(&scheme_id);
                config.save().expect("Failed to save config");
            }
        }
    }

    fn on_config_open_shortcuts_on_startup_notify(config: Arc<RwLock<Config>>) -> impl Fn(bool) {
        move |enabled| {
            let mut config = config.write().expect("Config lock poisoned");
            config.set_show_shortcuts_on_open(enabled);
            config.save().expect("Failed to save config");
        }
    }

    pub fn connect_config_style_scheme_notify<F: Fn(Option<StyleScheme>) + 'static>(
        &self,
        f: F,
    ) -> gtk4::glib::SignalHandlerId {
        // In GTK4, we would typically use a StyleSchemeChooser widget
        // For now, we'll connect to a button click that could open a chooser dialog
        self.widgets.color_scheme_button.connect_clicked(move |_| {
            // This would open a style scheme chooser and call f() with the result
            // For demonstration, we call with None
            f(None);
        })
    }

    pub fn connect_config_open_shortcuts_on_startup_notify<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> gtk4::glib::SignalHandlerId {
        self.widgets
            .shortcut_switch
            .connect_state_notify(move |switch| f(switch.state()))
    }

    pub fn present(&self) {
        self.widgets.preference_dialog.present();
    }

    pub fn close(&self) {
        self.widgets.preference_dialog.close();
    }

    pub fn set_transient_for(&self, parent: Option<&impl IsA<gtk4::Window>>) {
        self.widgets.preference_dialog.set_transient_for(parent);
    }

    fn connect_buttons(&self) {
        // Connect cancel button
        let dialog_clone = self.widgets.preference_dialog.clone();
        self.widgets
            .preference_cancel_button
            .connect_clicked(move |_| {
                dialog_clone.close();
            });

        // Connect OK button
        let dialog_clone2 = self.widgets.preference_dialog.clone();
        self.widgets.preference_ok_button.connect_clicked(move |_| {
            // Settings are saved automatically when changed, so just close
            dialog_clone2.close();
        });
    }
}
