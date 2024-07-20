use anyhow::Result;

use crate::cache::Cache;


pub struct CLI {
    cache: Cache
}


impl Default for CLI {
    fn default() -> Self {
        Self::new(Cache::default())
    }
}


impl CLI {
    pub fn new(cache: Cache) -> Self {
        Self {
            cache,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        self.cache.load()?;
        self.cache.update().await?;
        Ok(())
        // loop {
        //     self.show_actions();
        //     self.process_action();
        // }
    }

    fn show_actions(&self) {

    }

    fn process_action(&self) {

    }
}
