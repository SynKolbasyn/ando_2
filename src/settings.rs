use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Settings {
    update_site: bool,
}


impl Default for Settings {
    fn default() -> Self {
        Self::new(
            true,
        )
    }
}


impl Settings {
    pub fn new(update_site: bool) -> Self {
        Self {
            update_site,
        }
    }
}
