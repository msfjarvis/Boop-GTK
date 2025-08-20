use crate::{
    config::Config,
    executor::{self},
    script::Script,
    scriptmap::ScriptMap,
    ui::command_palette::Dialog as CommandDialog,
    ui::preferences_dialog::Dialog as PreferencesDialog,
    util::SourceViewExt,
    util::StringExt,
};
use eyre::{Context, Result};

// use gladis4::Gladis;
use glib::SourceId;
use gtk4::{prelude::*, Label, Revealer};
use sourceview5::{prelude::*, Language};

use executor::{ExecutorError, TextReplacement};
use gtk4::{ApplicationWindow, Button};
use std::sync::{Arc, RwLock};

use super::about_dialog::AboutDialog;

pub const NOTIFICATION_LONG_DELAY: u32 = 5000;

// #[derive(Gladis, Clone, Shrinkwrap)]
#[derive(Clone, Shrinkwrap)]
pub struct Widgets {
    #[shrinkwrap(main_field)]
    pub window: ApplicationWindow,

    pub header_button: Button,
    pub source_view: sourceview5::View,
    // status_bar: Statusbar,
    pub notification_revealer: Revealer,
    pub notification_label: Label,
    pub notification_button: Button,
}

#[derive(Clone, Shrinkwrap)]
pub struct App {
    #[shrinkwrap(main_field)]
    pub widgets: Widgets,
    pub preferences_dialog: PreferencesDialog,
    pub about_dialog: AboutDialog,

    pub scripts: Arc<RwLock<ScriptMap>>,
    pub notification_source_id: Arc<RwLock<Option<SourceId>>>,
    pub last_script_executed: Arc<RwLock<Option<String>>>,
    pub config: Arc<RwLock<Config>>,
}

impl App {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn new(
        boop_language: &Language,
        scripts: Arc<RwLock<ScriptMap>>,
        config: Arc<RwLock<Config>>,
    ) -> Result<Self> {
        // Use GTK4 Builder pattern to load from Glade file
        let builder = gtk4::Builder::from_resource("/fyi/zoey/Boop-GTK/boop-gtk.glade");

        let widgets = Widgets {
            window: builder.object("window").expect("Failed to get window"),
            header_button: builder
                .object("header_button")
                .expect("Failed to get header_button"),
            source_view: builder
                .object("source_view")
                .expect("Failed to get source_view"),
            notification_revealer: builder
                .object("notification_revealer")
                .expect("Failed to get notification_revealer"),
            notification_label: builder
                .object("notification_label")
                .expect("Failed to get notification_label"),
            notification_button: builder
                .object("notification_button")
                .expect("Failed to get notification_button"),
        };

        let app = App {
            widgets,
            preferences_dialog: PreferencesDialog::new(&config)?,
            about_dialog: AboutDialog::new(&scripts)?,
            scripts,
            notification_source_id: Arc::new(RwLock::new(None)),
            last_script_executed: Arc::new(RwLock::new(None)),
            config,
        };

        app.configure(boop_language)?;
        app.update_state_from_config()?;

        // close notification on dismiss
        {
            let notification_revealer = app.widgets.notification_revealer.clone();
            let gesture = gtk4::GestureClick::new();
            gesture.connect_pressed(move |_gesture, _n_press, _x, _y| {
                notification_revealer.set_reveal_child(false);
            });
            app.widgets.notification_button.add_controller(gesture);
        }

        {
            let source_view: sourceview5::View = app.widgets.source_view.clone();
            app.preferences_dialog
                .connect_config_style_scheme_notify(move |scheme| {
                    source_view
                        .get_sourceview_buffer()
                        .expect("Failed to get sourceview buffer")
                        .set_style_scheme(scheme.as_ref());
                });
        }

        // Connect the popover menu to the menu button
        {
            let main_menu: gtk4::PopoverMenu = builder
                .object("main_menu")
                .expect("Failed to get main_menu");
            let menu_button: gtk4::MenuButton = builder
                .object("menu_button")
                .expect("Failed to get menu_button");
            menu_button.set_popover(Some(&main_menu));
        }

        {
            let app_ = app.clone();
            app.widgets.header_button.connect_clicked(move |_| {
                app_.run_command_palette()
                    .expect("Failed to run command palette");
            });
        }

        Ok(app)
    }

    fn configure(&self, boop_language: &Language) -> Result<()> {
        self.preferences_dialog
            .set_transient_for(Some(&self.widgets.window));

        // update source_view syntax highlighting
        let buffer = self.widgets.source_view.get_sourceview_buffer()?;
        buffer.set_highlight_syntax(true);
        buffer.set_language(Some(boop_language));

        Ok(())
    }

    fn update_state_from_config(&self) -> Result<()> {
        let config = self
            .config
            .read()
            .map_err(|e| eyre!("Config lock poisoned: {}", e))?;

        // update source_view style scheme
        let scheme_id = &config.editor.colour_scheme_id;
        let scheme = sourceview5::StyleSchemeManager::default().scheme(scheme_id);
        self.widgets
            .source_view
            .get_sourceview_buffer()?
            .set_style_scheme(scheme.as_ref());

        Ok(())
    }

    fn post_notification(&self, text: &str, delay: u32) {
        let notification_source_id = self.notification_source_id.clone();
        let notification_revealer = self.widgets.notification_revealer.clone();
        let notification_label = self.widgets.notification_label.clone();

        {
            notification_label.set_markup(text);
            notification_revealer.set_reveal_child(true);

            let mut source_id = notification_source_id.write().unwrap();

            // cancel old notification timeout
            if source_id.is_some() {
                glib::SourceId::remove(source_id.take().unwrap());
            }

            // dismiss after 3000ms
            let new_source_id = {
                let notification_source_id = notification_source_id.clone();
                glib::timeout_add_local(std::time::Duration::from_millis(delay as u64), move || {
                    notification_revealer.set_reveal_child(false);
                    *notification_source_id.write().unwrap() = None;
                    glib::ControlFlow::Break
                })
            };

            source_id.replace(new_source_id);
        }
    }

    pub fn post_notification_error(&self, text: &str, delay: u32) {
        self.post_notification(
            &format!(r#"<span foreground="red" weight="bold">ERROR:</span> {text}"#),
            delay,
        );
    }

    pub fn run_command_palette(&self) -> Result<()> {
        let dialog = CommandDialog::new(&self.widgets.window, &self.scripts)?;
        let app_clone = self.clone();

        dialog.run_async(move |selected_script| {
            if let Some(selected) = selected_script {
                *app_clone.last_script_executed.write().unwrap() = Some(selected.clone());
                if let Err(e) = app_clone.execute_script(&selected) {
                    error!("Failed to execute script: {}", e);
                    app_clone.post_notification_error(
                        &format!("Failed to execute script: {}", e),
                        NOTIFICATION_LONG_DELAY,
                    );
                }
            }
        });

        Ok(())
    }

    pub fn re_execute(&self) -> Result<()> {
        if let Some(script_key) = &*self.last_script_executed.read().unwrap() {
            self.execute_script(script_key)
                .wrap_err("Failed to execute script")
        } else {
            warn!("no last script");
            Ok(())
        }
    }

    fn execute_script(&self, script_key: &str) -> Result<()> {
        let mut script_map = self.scripts.write().expect("Scripts lock is poisoned");
        let script: &mut Script = script_map
            .0
            .get_mut(script_key)
            .ok_or_else(|| eyre!("Script not in map"))?;

        info!("executing {}", script.metadata.name);

        let buffer = &self.widgets.source_view.buffer();

        let buffer_text = buffer
            .text(&buffer.start_iter(), &buffer.end_iter(), false)
            .to_string();

        let selection_text = buffer
            .selection_bounds()
            .map(|(start, end)| buffer.text(&start, &end, false).to_string());

        let status_result = script.execute(buffer_text.as_str(), selection_text.as_deref());

        match status_result {
            Ok(status) => {
                // TODO: how to handle multiple messages?
                if let Some(error) = status.error() {
                    self.post_notification(
                        &format!(r#"<span foreground="red" weight="bold">ERROR:</span> {error}"#),
                        NOTIFICATION_LONG_DELAY,
                    );
                } else if let Some(info) = status.info() {
                    self.post_notification(info, NOTIFICATION_LONG_DELAY);
                }
                self.do_replacement(status.clone().into_replacement())
                    .wrap_err_with(|| format!("Failed to make replacement: {status:?}"))?;
            }
            Err(err) => {
                let executor_err = err.downcast::<ExecutorError>().unwrap(); // can't recover from other errors

                error!("Exception: {executor_err:?}");
                self.post_notification_error(
                    &executor_err.into_notification_string(),
                    NOTIFICATION_LONG_DELAY,
                );
            }
        }

        Ok(())
    }

    fn do_replacement(&self, replacement: TextReplacement) -> Result<()> {
        let buffer = &self.widgets.source_view.buffer();

        match replacement {
            TextReplacement::Full(text) => {
                info!("replacing full text");

                let safe_text = text
                    .remove_null_bytes()
                    .wrap_err("Failed to remove null bytes from text")?;

                buffer.set_text(&safe_text);
            }
            TextReplacement::Selection(text) => {
                info!("replacing selection");

                let safe_text = text
                    .remove_null_bytes()
                    .wrap_err("Failed to remove null bytes from text")?;

                match buffer.selection_bounds() {
                    Some((mut start, mut end)) => {
                        buffer.delete(&mut start, &mut end);
                        buffer.insert(&mut start, &safe_text);
                    }
                    None => {
                        error!("tried to do a selection replacement, but no text is selected!");
                    }
                }
            }
            TextReplacement::Insert(insertions) => {
                let insert_text = insertions.join("");
                info!("inserting {} bytes", insert_text.len());

                let safe_text = insert_text
                    .remove_null_bytes()
                    .wrap_err("Failed to remove null bytes from text")?;

                if let Some((mut start, mut end)) = buffer.selection_bounds() {
                    buffer.delete(&mut start, &mut end);
                    buffer.insert(&mut start, &safe_text);
                } else {
                    let mut insert_point = buffer.iter_at_offset(buffer.cursor_position());
                    buffer.insert(&mut insert_point, &safe_text);
                }
            }
            TextReplacement::None => {
                info!("no text to replace");
            }
        }

        self.widgets.source_view.grab_focus();

        Ok(())
    }
}
