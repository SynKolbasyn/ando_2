use std::io::{stdout, Write};

use anyhow::{Context, Result};

use crate::cache::Cache;
use crate::cli::states::State;
use crate::settings::{Options, Settings};


pub struct SettingsMenu {
    menu: String,
}


impl Default for SettingsMenu {
    fn default() -> Self {
        Self::new(String::default())
    }
}


impl SettingsMenu {
    pub fn new(menu: String) -> Self {
        Self {
            menu,
        }
    }
    
    pub fn show_actions(&mut self, settings: &Settings) -> Result<()> {
        self.generate_menu(settings);
        print!("{}", self.menu);
        stdout().flush()?;
        Ok(())
    }
    
    pub fn process_action(&self, action: String, cache: &mut Cache) -> Result<State> {
        let index: usize = action.parse::<usize>()?
            .checked_sub(1)
            .context("Error during user input conversion")?;
        
        if index == Options::arr().len() {
            return Ok(State::MainMenu);
        }
        
        let option: String = Options::arr()
            .get(index)
            .context("Error while using user input")?
            .val();
        
        cache.settings.change_option(option)?;
        cache.update()?;
        
        Ok(State::SettingsMenu)
    }
    
    fn generate_menu(&mut self, settings: &Settings) {
        let mut menu: String = String::new();
        for (idx, (setting, state)) in settings.settings.iter().enumerate() {
            let star: String = if *state { String::from("*") } else { String::from(" ") };
            menu += format!("[{}] [{star}] -> {setting}\n", idx + 1).as_str();
        }
        menu += format!("[{}] -> Back\n", Options::arr().len() + 1).as_str();
        self.menu = menu + "~$ ";
    }
}
