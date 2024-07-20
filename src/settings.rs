use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Settings {
    update_site: bool,
    update_found_anime: bool,
}


impl Default for Settings {
    fn default() -> Self {
        Self::new(
            true,
            false,
        )
    }
}


impl Settings {
    pub fn new(update_site: bool, update_found_anime: bool) -> Self {
        Self {
            update_site,
            update_found_anime,
        }
    }
}
