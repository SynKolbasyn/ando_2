use std::{
    collections::BTreeSet,
    hash::Hash,
};

use anyhow::{Result, bail, Context};

use select::node::Node;

use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct Anime {
    pub name: String,
    pub url: String,
    pub episodes: Vec<Episode>
}


impl Default for Anime {
    fn default() -> Self {
        Self::new(
            String::default(),
            String::default(),
            Vec::default(),
        )
    }
}


impl Anime {
    pub fn new<Name: ToString, URL: ToString>(name: Name, url: URL, episodes: Vec<Episode>) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            episodes,
        }
    }
}


#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Episode {
    pub name: String,
    pub url: String,
    pub quality: BTreeSet<Quality>
}


impl Default for Episode {
    fn default() -> Self {
        Self::new(
            String::default(),
            String::default(),
            BTreeSet::default(),
        )
    }
}


impl Episode {
    pub fn new<Name: ToString, URL: ToString>(
        name: Name,
        url: URL,
        quality: BTreeSet<Quality>
    ) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            quality,
        }
    }
}


#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Quality {
    Q360P(String),
    Q480P(String),
    Q720P(String),
    Q1080P(String),
}


impl Default for Quality {
    fn default() -> Self {
        Self::arr()[0].clone()
    }
}


impl Quality {
    pub fn from(node: Node) -> Result<Self> {
        let quality: &str = node
            .attr("res")
            .context("Error when parsing anime quality")?;
        
        let url: String = node
            .attr("src")
            .context("")?
            .to_string();
        
        Ok(match quality {
            "360" => Self::Q360P(url),
            "480" => Self::Q480P(url),
            "720" => Self::Q720P(url),
            "1080" => Self::Q1080P(url),
            _ => bail!("An error in correlating anime quality and available quality"),
        })
    }

    pub fn arr() -> [Self; 4] {
        [
            Self::Q360P(String::from("360p")),
            Self::Q480P(String::from("480p")),
            Self::Q720P(String::from("720p")),
            Self::Q1080P(String::from("1080p")),
        ]
    }
    
    pub fn equal(&self, quality: &Quality) -> bool {
        self.empty() == quality.empty()
    }
    
    pub fn empty(&self) -> Self {
        match self {
            Self::Q360P(_) => Self::Q360P(String::new()),
            Self::Q480P(_) => Self::Q480P(String::new()),
            Self::Q720P(_) => Self::Q720P(String::new()),
            Self::Q1080P(_) => Self::Q1080P(String::new()),
        }
    }
    
    pub fn val(&self) -> String {
        match self {
            Self::Q360P(url) => url,
            Self::Q480P(url) => url,
            Self::Q720P(url) => url,
            Self::Q1080P(url) => url,
        }.clone()
    }
}
