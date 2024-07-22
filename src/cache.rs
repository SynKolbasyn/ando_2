use std::{
    fs::{File, create_dir_all},
    path::Path,
};

use anyhow::{Result, Context};

use indicatif::ProgressBar;

use serde::{Deserialize, Serialize};

use crate::anime::{Anime, Episode, Quality};
use crate::net::Net;
use crate::parser::Parser;
use crate::settings::Settings;


#[derive(Clone, Serialize, Deserialize)]
pub struct Cache {
    #[serde(skip_serializing, skip_deserializing)]
    net: Net,
    #[serde(skip_serializing, skip_deserializing)]
    parser: Parser,
    path: String,
    pub settings: Settings,
    pages: u64,
    pub anime: Vec<Anime>,
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
        )
    }
}


impl Cache {
    pub fn new<P: ToString>(
        net: Net,
        parser: Parser,
        path: P,
        settings: Settings,
        pages: u64,
        anime: Vec<Anime>,
    ) -> Self {
        Self {
            net,
            parser,
            path: path.to_string(),
            settings,
            pages,
            anime,
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

    pub fn update(&mut self) -> Result<()> {
        if !self.folder()?.exists() {
            create_dir_all(self.folder()?)?;
        }
        if !self.file().exists() {
            File::create(self.file())?;
        }
        let file: File = File::options().write(true).truncate(true).open(self.file())?;
        serde_json::to_writer_pretty(file, self)?;
        
        Ok(())
    }
    
    pub async fn full_update(&mut self) -> Result<()> {
        let pages: u64 = self.pages;
        self.pages = 2;
        let site: String = self.net.get_anime_list_html(&mut self.pages, pages).await?;
        self.anime = self.parser.parse_anime_list(site)?;
        self.update()?;
        Ok(())
    }
    
    pub fn get_anime_list(&self) -> Vec<Anime> {
        self.anime.clone()
    }
    
    pub async fn get_anime(&self, id: usize) -> Result<Anime> {
        let anime: Anime = self.anime
            .get(id)
            .context("Error when trying to select an anime")?
            .clone();
        let anime_html: String = self.net.get_anime_html(anime.url).await?;
        self.parser.parse_anime(anime_html)
    }
    
    pub async fn download_episode(&self, mut episode: Episode, quality: Quality, pb: &ProgressBar) -> Result<()> {
        let episode_html: String = self.net.get_episode_html(episode.clone().url).await?;
        let episode_urls: Episode = self.parser.parse_episode(episode_html)?;
        episode.quality = episode_urls.quality;
        self.net.download_episode(episode, quality, pb).await?;
        Ok(())
    }
    
    pub async fn download_episodes(&self, episodes: Vec<Episode>, quality: Quality, pbs: &Vec<ProgressBar>) -> Result<()> {
        for i in 0..episodes.len() {
            self.download_episode(episodes[i].clone(), quality.clone(), &pbs[i]).await?;
        }
        Ok(())
    }
    
    fn folder(&self) -> Result<&Path> {
        self.file().parent().context("Error receiving the cache folder")
    }
    
    fn file(&self) -> &Path {
        Path::new(&self.path)
    }
}
