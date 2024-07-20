use std::{
    fs::{File, create_dir_all},
    path::Path,
};

use anyhow::{Result, Context};

use serde::{Deserialize, Serialize};

use crate::anime::Anime;
use crate::net::Net;
use crate::parser::Parser;
use crate::settings::Settings;


#[derive(Serialize, Deserialize)]
pub struct Cache {
    #[serde(skip_serializing, skip_deserializing)]
    net: Net,
    #[serde(skip_serializing, skip_deserializing)]
    parser: Parser,
    path: String,
    settings: Settings,
    pages: u64,
    found_anime: Vec<Anime>,
    site: String,
}


impl Default for Cache {
    fn default() -> Self {
        Self::new(
            Net::default(),
            Parser::default(),
            "./data/cache.json",
            Settings::default(),
            0,
            Vec::default(),
            "",
        )
    }
}


impl Cache {
    pub fn new<P: ToString, S: ToString>(
        net: Net,
        parser: Parser,
        path: P,
        settings: Settings,
        pages: u64,
        found_anime: Vec<Anime>,
        site: S,
    ) -> Self {
        Self {
            net,
            parser,
            path: path.to_string(),
            settings,
            pages,
            found_anime,
            site: site.to_string(),
        }
    }
    
    pub fn load(&mut self) -> Result<()> {
        if !self.folder()?.exists() {
            create_dir_all(self.folder()?)?;
            *self = Self::default();
            return Ok(());
        }

        if !self.file().exists() {
            File::create(self.file())?;
            *self = Self::default();
            return Ok(());
        }

        let file: File = File::open(self.file())?;
        
        *self = match serde_json::from_reader::<File, Self>(file) {
            Ok(mut cache) => {
                cache.net = Net::default();
                cache.parser = Parser::default();
                cache
            },
            Err(_) => Self::default(),
        };
        
        Ok(())
    }

    pub async fn update(&mut self) -> Result<()> {
        let pages: u64 = self.pages;
        self.pages = 2;
        self.site = self.net.get_anime_html(&mut self.pages, pages).await?;

        if !self.folder()?.exists() {
            create_dir_all(self.folder()?)?;
        }
        if !self.file().exists() {
            File::create(self.file())?;
        }
        let file: File = File::options().write(true).open(self.file())?;
        serde_json::to_writer_pretty(file, self)?;
        
        Ok(())
    }
    
    fn folder(&self) -> Result<&Path> {
        self.file().parent().context("Error receiving the cache folder")
    }
    
    fn file(&self) -> &Path {
        Path::new(&self.path)
    }
}
