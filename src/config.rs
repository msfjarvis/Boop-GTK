use std::{fs::File, io::Write};

use crate::XDG_DIRS;
use eyre::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub show_shortcuts_on_open: bool,
    pub editor: Editor,
}

#[derive(Serialize, Deserialize)]
pub struct Editor {
    pub colour_scheme_id: String,
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            colour_scheme_id: String::from("classic"),
        }
    }
}

impl Config {
    pub fn load() -> Result<(Config, bool)> {
        let mut config_file_created = false;

        let config_path = XDG_DIRS
            .place_config_file("config.toml")
            .wrap_err("Failed to place config file")?;

        if !config_path.exists() {
            info!("creating config.toml");
            config_file_created = true;

            // no config file, write default
            let mut file = File::create(&config_path).wrap_err("Failed to create config file")?;
            file.write_all(
                toml::to_string_pretty(&Config::default())
                    .unwrap()
                    .as_bytes(),
            )
            .wrap_err("Failed to write to config file")?;
        }

        let settings = config::Config::builder().add_source(config::File::from(config_path));

        let config = settings
            .build()
            .wrap_err("Failed to covert settings into Config")?
            .try_deserialize()?;

        Ok((config, config_file_created)) // TODO: handle results
    }

    pub fn save(&self) -> Result<()> {
        let config_path = XDG_DIRS
            .place_config_file("config.toml")
            .wrap_err("Failed to place config file")?;

        File::create(config_path)
            .wrap_err("Failed to create config file")?
            .write_all(toml::to_string_pretty(self).unwrap().as_bytes())
            .wrap_err("Failed to write to config file")
    }

    pub fn set_show_shortcuts_on_open(&mut self, enable: bool) {
        self.show_shortcuts_on_open = enable;
    }
}

impl Editor {
    pub fn set_colour_scheme_id(&mut self, id: &str) {
        self.colour_scheme_id = String::from(id);
    }
}
