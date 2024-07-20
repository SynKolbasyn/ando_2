use std::time::Duration;

use anyhow::{Result, bail};

use indicatif::{ProgressBar, ProgressStyle};

use reqwest::{Client, IntoUrl, Response};
use tokio::time::sleep;


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

        Ok(result)
    }
    
    pub async fn get_anime_html<URL: IntoUrl>(&self, url: URL) -> Result<String> {
        Ok(self.client.get(url).send().await?.text().await?)
    }
}
