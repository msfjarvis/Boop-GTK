use std::sync::{Arc, RwLock};

use eyre::{Context, Result};
// use gladis4::Gladis;
use glib::SignalHandlerId;
use gtk4::{prelude::*, Dialog as GtkDialog, Switch};
use sourceview5::{prelude::*, StyleScheme};

use crate::config::Config;

// #[derive(Gladis, Clone, Shrinkwrap)]
#[derive(Clone)]
pub struct Widgets {
    // #[shrinkwrap(main_field)]
    pub preference_dialog: GtkDialog, // TODO: change to preferences_dialog

    pub color_scheme_button: gtk4::Label,
    pub shortcut_switch: Switch,
}

// #[derive(Clone, Shrinkwrap)]
#[derive(Clone)]
pub struct Dialog {
    // #[shrinkwrap(main_field)]
    pub widgets: Widgets,
    pub config: Arc<RwLock<Config>>,
}

impl Dialog {
    pub(crate) fn new(config: &Arc<RwLock<Config>>) -> Result<Self> {
        let preference_dialog = GtkDialog::builder()
            .title("Preferences")
            .modal(true)
            .build();
        let color_scheme_button = gtk4::Label::new(Some("Color Scheme"));
        let shortcut_switch = Switch::new();

        let widgets = Widgets {
            preference_dialog,
            color_scheme_button,
            shortcut_switch,
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
        // Temporarily disabled - would need proper StyleSchemeChooser
        // self.widgets.color_scheme_button.set_style_scheme(&scheme);

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
        _f: F,
    ) -> gtk4::glib::SignalHandlerId {
        // Temporarily return a dummy signal handler ID using a dummy button
        // In a real implementation, this would connect to the proper signal
        let dummy_button = gtk4::Button::new();
        dummy_button.connect_clicked(|_| {})
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

    pub fn set_transient_for(&self, parent: Option<&impl IsA<gtk4::Window>>) {
        self.widgets.preference_dialog.set_transient_for(parent);
    }
}
