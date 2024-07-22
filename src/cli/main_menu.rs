use std::{
    io::{stdout, Write},
    process::exit
};

use anyhow::{Context, Result};
use crate::cache::Cache;
use crate::cli::states::State;


pub struct MainMenu {
    menu: String,
}


impl Default for MainMenu {
    fn default() -> Self {
        let mut menu: String = String::default();
        for (idx, action) in Action::arr().iter().enumerate() {
            menu += format!("[{}] -> {}\n", idx + 1, action.text()).as_str();
        }
        Self::new(menu + "~$ ")
    }
}


impl MainMenu {
    pub fn new(menu: String) -> Self {
        Self {
            menu,
        }
    }

    pub fn show_actions(&self) -> Result<()> {
        print!("{}", self.menu);
        stdout().flush()?;
        Ok(())
    }

    pub async fn process_action(&mut self, action: String, cache: &mut Cache) -> Result<State> {
        let index: usize = action.parse::<usize>()?
            .checked_sub(1)
            .context("Error during user input conversion")?;

        Ok(match Action::arr().get(index).context("Error while using user input")? {
            Action::DownloadAnime(_) => State::DownloadMenu,
            Action::Settings(_) => State::SettingsMenu,
            Action::UpdateCache(_) => Self::update_cache(cache).await?,
            Action::Exit(_) => exit(0),
        })
    }
    
    pub async fn update_cache(cache: &mut Cache) -> Result<State> {
        cache.full_update().await?;
        Ok(State::MainMenu)
    }
}


enum Action {
    DownloadAnime(String),
    Settings(String),
    UpdateCache(String),
    Exit(String),
}


impl Action {
    fn arr() -> [Action; 4] {
        [
            Action::DownloadAnime(String::from("Download anime")),
            Action::Settings(String::from("Settings")),
            Action::UpdateCache(String::from("Update cache")),
            Action::Exit(String::from("Exit")),
        ]
    }
    
    fn text(&self) -> String {
        match self {
            Self::DownloadAnime(text) => text,
            Self::Settings(text) => text,
            Self::UpdateCache(text) => text,
            Self::Exit(text) => text,
        }.clone()
    }
}
