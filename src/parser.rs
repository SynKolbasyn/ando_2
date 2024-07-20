use anyhow::{Result, Context};

use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use crate::anime::{Anime, Episode, Quality};

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

    pub fn parse_anime_list(&self, anime_list_html: String) -> Result<Vec<String>> {
        let mut result: Vec<String> = Vec::new();
        
        let document: Document = Document::from(anime_list_html.as_str());
        for node in document.find(Class("all_anime_global")) {
            let url: String = node.first_child().context("Error when searching for an anime link")?
                .attr("href").context("Error when searching for an anime link")?.to_string();
            let name: String = node.text().trim().split("\n").next()
                .context("Error when searching for an anime link")?.to_string();
            result.push(format!("{}: https://jut.su{}", name, url));
        }
        
        Ok(result)
    }

    pub fn parse_anime(&self, anime_html: String) -> Result<Anime> {
        let mut anime: Anime = Anime::default();

        let document: Document = Document::from(anime_html.as_str());
        for node in document.find(Class("short-btn")) {
            let name: String = node.text();
            let url: &str = node.attr("href").context("Error when searching for an anime link")?;
            let episode: Episode = Episode::new(name, url, Quality::default());
            anime.episodes.push(episode);
        }

        Ok(anime)
    }
}
