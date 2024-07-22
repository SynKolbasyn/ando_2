use std::time::Duration;

use anyhow::{Result, bail, Context};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use rayon::prelude::*;

use reqwest::{Client, IntoUrl, Response};
use tokio::fs::{create_dir_all, File};
use futures::StreamExt;
use tokio::time::sleep;
use crate::anime::{Episode, Quality};


#[derive(Clone)]
pub struct Net {
    client: Client,
}


impl Default for Net {
    fn default() -> Self {
        Self::new(Client::default())
    }
}


impl Net {
    pub fn new(client: Client) -> Self {
        Self {
            client,
        }
    }

    pub async fn get_anime_list_html(&self, from_page: &mut u64, pages: u64) -> Result<String> {
        let spinner_style: ProgressStyle = ProgressStyle::with_template(
            "{prefix:.bold.dim} {spinner} {wide_msg}"
        )?.tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
        let pb: ProgressBar = ProgressBar::new(pages);
        pb.set_style(spinner_style);
        pb.set_message("Fetching anime...");
        if pages == 0 {
            pb.set_prefix(format!("[{from_page}/?]"));
        }
        else {
            pb.set_prefix(format!("[{from_page}/{pages}]"));
        }
        
        let mut result: String = self.client.get("https://jut.su/anime/").send().await?.text().await?;

        loop {
            // TODO: Add all headers
            let response: Response = self.client.post("https://jut.su/anime/")
                .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
                .body(format!("ajax_load=yes&start_from_page={from_page}&show_search=&anime_of_user="))
                .send().await?;

            if response.status() != 200 {
                bail!("Status code not 200");
            }

            let body: String = response.text().await?;

            if body == "empty" {
                break;
            }

            result += body.as_str();
            
            pb.set_position(*from_page);
            if pages == 0 {
                pb.set_prefix(format!("[{from_page}/?]"));
            }
            else {
                pb.set_prefix(format!("[{from_page}/{pages}]"));
            }

            *from_page += 1;

            sleep(Duration::from_millis(250)).await;
        }
        *from_page -= 1;
        
        pb.finish_with_message("Fetching done!");

        Ok(result)
    }
    
    pub async fn get_anime_html<URL: IntoUrl>(&self, anime_url: URL) -> Result<String> {
        self.get_html(anime_url).await
    }

    pub async fn get_episode_html<URL: IntoUrl>(&self, episode_url: URL) -> Result<String> {
        self.get_html(episode_url).await
    }

    async fn get_html<URL: IntoUrl>(&self, url: URL) -> Result<String> {
        // TODO: Catch request code
        Ok(self.client.get(url).send().await?.text().await?)
    }

    pub async fn download_episode(&self, episode: Episode, quality: Quality, pb: &ProgressBar) -> Result<()> {
        let url: String = episode.quality
            .par_iter()
            .find_any(|&q| q.equal(&quality))
            .context("Error while searching for the selected quality")?
            .val();
        let response: Response = self.client.get(url).send().await?;
        // TODO: Check response
        let size: u64 = response.content_length().context("Error while getting the episode size")?;
        
        pb.set_length(size);
        pb.set_message(episode.name.clone());

        create_dir_all("./data/anime/").await?;
        let mut file: File = File::create(format!("./data/anime/{}.mp4", episode.name)).await?;
        let mut stream = response.bytes_stream();
        while let Some(bytes) = stream.next().await {
            let bytes = bytes?;
            tokio::io::copy(&mut bytes.clone().as_ref(), &mut file).await?;
            pb.inc(bytes.len() as u64);
        }
        pb.finish_with_message(format!("Downloading complete: {}", episode.name));
        Ok(())
    }
}
