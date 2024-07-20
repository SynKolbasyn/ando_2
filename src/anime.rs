use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
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


#[derive(Serialize, Deserialize)]
pub struct Episode {
    pub name: String,
    pub url: String,
    pub quality: Quality
}


impl Default for Episode {
    fn default() -> Self {
        Self::new(
            String::default(),
            String::default(),
            Quality::default(),
        )
    }
}


impl Episode {
    pub fn new<Name: ToString, URL: ToString>(name: Name, url: URL, quality: Quality) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            quality,
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct Quality {
    pub url_360p: Option<String>,
    pub url_480p: Option<String>,
    pub url_720p: Option<String>,
    pub url_1080p: Option<String>,
}


impl Default for Quality {
    fn default() -> Self {
        Self::new(
            None,
            None,
            None,
            None,
        )
    }
}


impl Quality {
    pub fn new(
        url_360p: Option<String>,
        url_480p: Option<String>,
        url_720p: Option<String>,
        url_1080p: Option<String>
    ) -> Self {
        Self {
            url_360p,
            url_480p,
            url_720p,
            url_1080p,
        }
    }
}
