use std::collections::BTreeSet;

use anyhow::{Result, Context};

use select::{
    document::Document,
    predicate::{Class, Name},
};

use crate::anime::{Anime, Episode, Quality};


#[derive(Clone)]
pub struct Parser {

}


impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}


impl Parser {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn parse_anime_list(&self, anime_list_html: String) -> Result<Vec<Anime>> {
        let mut result: Vec<Anime> = Vec::new();
        
        let document: Document = Document::from(anime_list_html.as_str());
        for node in document.find(Class("all_anime_global")) {
            let name: String = node.text().trim().split("\n").next()
                .context("Error when searching for an anime link")?.to_string();
            let url: &str = node.first_child().context("Error when searching for an anime link")?
                .attr("href").context("Error when searching for an anime link")?;
            let anime: Anime = Anime::new(name, format!("https://jut.su{url}"), Vec::default());
            result.push(anime);
        }
        
        Ok(result)
    }

    pub fn parse_anime(&self, anime_html: String) -> Result<Anime> {
        let mut anime: Anime = Anime::default();

        let document: Document = Document::from(anime_html.as_str());
        for node in document.find(Class("short-btn")) {
            let name: String = node.text();
            let url: &str = node.attr("href").context("Error when searching for an anime link")?;
            let episode: Episode = Episode::new(name, format!("https://jut.su{url}"), BTreeSet::new());
            anime.episodes.push(episode);
        }

        Ok(anime)
    }
    
    pub fn parse_episode(&self, episode_html: String) -> Result<Episode> {
        let mut episode: Episode = Episode::default();

        let document: Document = Document::from(episode_html.as_str());
        for node in document.find(Name("source")) {
            episode.quality.insert(Quality::from(node)?);
        }
        
        Ok(episode)
    }
}
