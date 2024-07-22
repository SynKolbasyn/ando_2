mod states;
mod main_menu;
mod settings_menu;
mod download_menu;


use std::io::stdin;

use anyhow::Result;

use crate::cache::Cache;
use crate::cli::download_menu::DownloadMenu;
use crate::cli::main_menu::MainMenu;
use crate::cli::settings_menu::SettingsMenu;
use crate::cli::states::State;

pub struct CLI {
    cache: Cache,
    state: State,
    main_menu: MainMenu,
    settings_menu: SettingsMenu,
    download_menu: DownloadMenu,
}


impl Default for CLI {
    fn default() -> Self {
        Self::new(
            Cache::default(),
            State::default(),
            MainMenu::default(),
            SettingsMenu::default(),
            DownloadMenu::default(),
        )
    }
}


impl CLI {
    pub fn new(cache: Cache, state: State, main_menu: MainMenu, settings_menu: SettingsMenu, download_menu: DownloadMenu) -> Self {
        Self {
            cache,
            state,
            main_menu,
            settings_menu,
            download_menu,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        self.cache.load()?;
        loop {
            self.show_actions()?;
            self.process_action().await?;
        }
    }
    
    fn show_actions(&mut self) -> Result<()> {
        match self.state {
            State::MainMenu => self.main_menu.show_actions(),
            State::SettingsMenu => self.settings_menu.show_actions(&self.cache.settings),
            State::DownloadMenu => self.download_menu.show_actions(&self.cache),
        }
    }

    async fn process_action(&mut self) -> Result<()> {
        let mut action: String = String::new();
        stdin().read_line(&mut action)?;
        action = action.trim().to_string();
        
        self.state = match self.state {
            State::MainMenu => self.main_menu.process_action(action, &mut self.cache).await?,
            State::SettingsMenu => self.settings_menu.process_action(action, &mut self.cache)?,
            State::DownloadMenu => self.download_menu.process_action(action, &mut self.cache).await?,
        };
        
        Ok(())
    }
}
