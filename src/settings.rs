use std::collections::HashMap;

use anyhow::{Result, Context};

use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub settings: HashMap<String, bool>,
}


impl Default for Settings {
    fn default() -> Self {
        let mut settings: HashMap<String, bool> = HashMap::default();
        for option in Options::arr() {
            settings.insert(option.val(), false);
        }
        Self::new(settings)
    }
}


impl Settings {
    pub fn new(settings: HashMap<String, bool>) -> Self {
        Self {
            settings,
        }
    }

    pub fn change_option(&mut self, option: String) -> Result<()> {
        self.check_settings();
        
        let state: bool = self.settings
            .get(&option)
            .context("Error when trying to change settings")? ^ true;
        self.settings.insert(option, state);
        Ok(())
    }
    
    fn check_settings(&mut self) {
        if self.settings.len() != Options::arr().len() {
            *self = Self::default();
            return;
        }

        for option in Options::arr() {
            if !self.settings.contains_key(&option.val()) {
                *self = Self::default();
                return;
            }
        }
    }
}


pub enum Options {
    UpdateFoundAnime(String),
}


impl Options {
    pub fn arr() -> [Self; 1] {
        [
            Self::UpdateFoundAnime(String::from("Update the anime that you have already searched for")),
        ]
    }
    
    pub fn val(&self) -> String {
        match self {
            Self::UpdateFoundAnime(text) => text,
        }.clone()
    }
}
