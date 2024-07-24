use std::{
    io::{stdin, stdout, Write},
    collections::HashSet,
};

use anyhow::{bail, Context, Result};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use rayon::prelude::*;

use tokio::task::JoinHandle;

use crate::anime::{Anime, Episode, Quality};
use crate::cache::Cache;
use crate::cli::states::State;


pub struct DownloadMenu {
    menu: String,
    download_state: DownloadState,
    selected_anime_list: Vec<Anime>,
    selected_anime: Anime,
    download_type: DownloadType,
    selected_episodes: HashSet<Episode>,
    selected_quality: Quality,
    thread_count: usize,
}


impl Default for DownloadMenu {
    fn default() -> Self {
        Self::new(
            String::default(),
            DownloadState::default(),
            Vec::default(),
            Anime::default(),
            DownloadType::default(),
            HashSet::default(),
            Quality::default(),
            1
        )
    }
}


impl DownloadMenu {
    fn new(
        menu: String,
        download_state: DownloadState,
        selected_anime_list: Vec<Anime>,
        selected_anime: Anime,
        download_type: DownloadType,
        selected_episodes: HashSet<Episode>,
        selected_quality: Quality,
        thread_count: usize,
    ) -> Self {
        Self {
            menu,
            download_state,
            selected_anime_list,
            selected_anime,
            download_type,
            selected_episodes,
            selected_quality,
            thread_count,
        }
    }
    
    pub fn show_actions(&mut self, cache: &Cache) -> Result<()> {
        self.generate_menu(cache);
        print!("{}", self.menu);
        stdout().flush()?;
        Ok(())
    }
    
    pub async fn process_action(&mut self, action: String, cache: &mut Cache) -> Result<State> {
        match self.download_state {
            DownloadState::SelectAnime => self.select_anime(action, &cache).await?,
            DownloadState::SelectDownloadType => self.select_download_type(action)?,
            DownloadState::SelectEpisode => self.select_episode(action)?,
            DownloadState::SelectQuality => self.select_quality(action)?,
            DownloadState::SelectThreadCount => self.select_thread_count(action)?,
            DownloadState::Download => {
                self.start_downloading(action, &cache).await?;
                self.selected_anime_list = cache.anime.clone();
                return Ok(State::MainMenu);
            },
        }
        Ok(State::DownloadMenu)
    }
    
    fn generate_menu(&mut self, cache: &Cache) {
        match self.download_state {
            DownloadState::SelectAnime => self.generate_select_anime_list_menu(cache),
            DownloadState::SelectDownloadType => self.generate_select_download_type_menu(),
            DownloadState::SelectEpisode => self.generate_select_episode_menu(),
            DownloadState::SelectQuality => self.generate_select_quality_menu(),
            DownloadState::SelectThreadCount => self.generate_select_thread_count_menu(),
            DownloadState::Download => self.generate_download_menu(),
        }
    }
    
    fn generate_select_anime_list_menu(&mut self, cache: &Cache) {
        let mut menu: String = String::new();

        if self.selected_anime_list.is_empty() {
            self.selected_anime_list = cache.anime.clone();
        }
        
        for (idx, anime) in self.selected_anime_list.iter().enumerate() {
            menu += format!("[{}] -> {} ({})\n", idx + 1, anime.name, anime.url).as_str();
        }
        self.menu = menu + "~$ ";
    }
    
    fn generate_select_download_type_menu(&mut self) {
        let mut menu: String = String::new();
        for (idx, t) in DownloadType::arr().iter().enumerate() {
            menu += format!("[{}] -> {}\n", idx + 1, t.val()).as_str();
        }
        self.menu = menu + "~$ ";
    }
    
    fn generate_select_episode_menu(&mut self) {
        match self.download_type {
            DownloadType::OneEpisode(_) => self.generate_select_one_episode_menu(),
            DownloadType::SomeEpisodes(_) => self.generate_select_some_episodes_menu(),
            DownloadType::RangeEpisodes(_) => self.generate_select_range_episodes_menu(),
            DownloadType::AllEpisodes(_) => self.generate_select_all_episodes_menu(),
        }
    }
    
    fn generate_select_one_episode_menu(&mut self) {
        let mut menu: String = String::new();
        for (idx, episode) in self.selected_anime.episodes.iter().enumerate() {
            menu += format!("[{}] -> {}\n", idx + 1, episode.name).as_str();
        }
        self.menu = menu + "~$ ";
    }

    fn generate_select_some_episodes_menu(&mut self) {
        let mut menu: String = String::new();
        for (idx, episode) in self.selected_anime.episodes.iter().enumerate() {
            let star: String = if self.selected_episodes.contains(episode) {
                String::from("*")
            }
            else {
                String::from(" ")
            };
            menu += format!("[{}] [{}] -> {}\n", idx + 1, star, episode.name).as_str();
        }
        menu += format!("[{}] -> Done\n", self.selected_anime.episodes.len() + 1).as_str();
        self.menu = menu + "~$ ";
    }
    
    fn generate_select_range_episodes_menu(&mut self) {
        self.menu = String::from("Select the episode including which range will start: ");
    }

    fn generate_select_all_episodes_menu(&mut self) {
        self.menu = String::from("Selecting all episodes...\n");
    }

    fn generate_select_quality_menu(&mut self) {
        // TODO: If download quality error, not drop program, print info about failed episode and quality
        let mut menu: String = String::new();
        for (idx, quality) in Quality::arr().iter().enumerate() {
            menu += format!("[{}] -> {}\n", idx + 1, quality.val()).as_str();
        }
        self.menu = menu + "~$ ";
    }

    fn generate_select_thread_count_menu(&mut self) {
        self.menu = String::from(
            "Select the number of episodes that will be downloaded at the same time\n"
        );
        self.menu += "~$ "
    }

    fn generate_download_menu(&mut self) {
        let mut menu: String = String::new();
        
        menu += format!("Selected anime: {}\n", self.selected_anime.name).as_str();
        menu += format!("Selected episodes: {:#?}\n", self.selected_episodes.par_iter().map(|e| e.name.clone()).collect::<Vec<String>>()).as_str();
        menu += format!("Selected quality: {}\n", self.selected_quality.val()).as_str();
        menu += format!("Selected thread count: {}\n", self.thread_count).as_str();
        
        self.menu = menu + "Start download? [Y/n]: ";
    }
    
    async fn select_anime(&mut self, action: String, cache: &Cache) -> Result<()> {
        match self.parse_action(action.clone()) {
            Ok(index) => {
                self.selected_anime = self.get_anime(index, cache).await?;
                self.download_state = DownloadState::SelectDownloadType;
            },
            Err(_) => {
                self.selected_anime_list = cache.get_anime_name(action)?;
            }
        }
        
        Ok(())
    }
    
    async fn get_anime(&self, id: usize, cache: &Cache) -> Result<Anime> {
        let anime: Anime = self.selected_anime_list
            .get(id)
            .context("Error when trying to select an anime")?
            .clone();
        Ok(cache.get_anime_self(anime).await?)
    }
    
    fn select_download_type(&mut self, action: String) -> Result<()> {
        let index: usize = self.parse_action(action)?;
        self.download_type = DownloadType::arr()
            .get(index)
            .context("Error when choosing the download type")?
            .clone();
        self.download_state = DownloadState::SelectEpisode;
        Ok(())
    }
    
    fn select_episode(&mut self, action: String) -> Result<()> {
        match self.download_type {
            DownloadType::OneEpisode(_) => self.select_one_episode(action)?,
            DownloadType::SomeEpisodes(_) => self.select_some_episodes(action)?,
            DownloadType::RangeEpisodes(_) => self.select_range_episodes(action)?,
            DownloadType::AllEpisodes(_) => self.select_all_episodes(),
        }
        Ok(())
    }
    
    fn select_one_episode(&mut self, action: String) -> Result<()> {
        let index: usize = self.parse_action(action)?;
        self.selected_episodes = HashSet::from([
            self.selected_anime.episodes
                .get(index)
                .context("Error when selecting an episode")?
                .clone()
        ]);
        
        self.download_state = DownloadState::SelectQuality;
        
        Ok(())
    }
    
    fn select_some_episodes(&mut self, action: String) -> Result<()> {
        let index: usize = self.parse_action(action)?;

        if index == self.selected_anime.episodes.len() {
            self.download_state = DownloadState::SelectQuality;
            return Ok(());
        }
        
        let episode: Episode = self.selected_anime.episodes
            .get(index)
            .context("Error when selecting an episode")?
            .clone();
        
        if self.selected_episodes.contains(&episode) {
            self.selected_episodes.remove(&episode);
        }
        else {
            self.selected_episodes.insert(episode);
        }
        
        Ok(())
    }
    
    fn select_range_episodes(&mut self, action: String) -> Result<()> {
        let start_range: usize = self.parse_action(action.clone())?;
        
        if start_range >= self.selected_anime.episodes.len() {
            bail!("Error when entering the initial number of the range");
        }
        
        let mut end_range: String = String::new();
        print!(
            "Select the episode including which the range will end [{}..{}]: ",
            start_range + 2,
            self.selected_anime.episodes.len()
        );
        stdout().flush()?;
        stdin().read_line(&mut end_range)?;
        let end_range: usize = self.parse_action(end_range)?;
        
        self.selected_episodes = self.selected_anime.episodes
            .get(start_range..=end_range)
            .context("Error when selecting a range of episodes")?
            .par_iter().map(|e| e.clone()).collect();
        
        self.download_state = DownloadState::SelectQuality;
        
        Ok(())
    }
    
    fn select_all_episodes(&mut self) {
        self.selected_episodes = self.selected_anime.episodes
            .par_iter()
            .map(|e| e.clone())
            .collect();
        
        self.download_state = DownloadState::SelectQuality;
    }
    
    fn select_quality(&mut self, action: String) -> Result<()> {
        let index: usize = self.parse_action(action)?;
        self.selected_quality = Quality::arr()
            .get(index)
            .context("Error during quality selection")?
            .clone();

        self.download_state = DownloadState::SelectThreadCount;
        
        if self.download_type.equal(&DownloadType::default()) {
            self.thread_count = 1;
            self.download_state = DownloadState::Download;
        }
        
        Ok(())
    }
    
    fn select_thread_count(&mut self, action: String) -> Result<()> {
        self.thread_count = action.parse()?;
        
        if self.thread_count < 1 {
            self.thread_count = 1;
            
        }
        
        if self.thread_count > self.selected_episodes.len() {
            self.thread_count = self.selected_episodes.len()
        }
        
        self.download_state = DownloadState::Download;
        
        Ok(())
    }
    
    async fn start_downloading(&self, action: String, cache: &Cache) -> Result<()> {
        // TODO: Rewrite to queue and threads, that take episodes from this queue
        if (action.to_lowercase() != "y") && (action.to_lowercase() != "yes") {
            println!("Download canceled");
            return Ok(());
        }
        
        let multi_pb: MultiProgress = MultiProgress::new();
        
        let episodes: Vec<Episode> = self.selected_episodes.par_iter().map(|e| e.clone()).collect();
        let chunks: Vec<&[Episode]> = episodes.par_chunks(episodes.len().div_ceil(self.thread_count)).collect();
        let mut handles: Vec<JoinHandle<Result<()>>> = Vec::new();
        
        for chunk in chunks {
            let mut pbs: Vec<ProgressBar> = Vec::new();
            for pb in chunk
                .par_iter()
                .map(|_| -> Result<ProgressBar> {
                    let pb: ProgressBar = multi_pb.add(ProgressBar::new(0));
                    pb.set_style(
                        ProgressStyle::default_bar()
                            .template(
                                "{msg} -> {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})"
                            )?
                    );
                    Ok(pb)
                }).collect::<Vec<Result<ProgressBar>>>() {
                pbs.push(pb?);
            }
            
            let cache: Cache = cache.clone();
            let eps = chunk.to_vec();
            let quality: Quality = self.selected_quality.clone();
            
            let handle: JoinHandle<Result<()>> = tokio::task::spawn(async move {
                cache.download_episodes(eps, quality, &pbs).await?;
                Ok(())
            });
            
            handles.push(handle);
        }

        for handle in handles {
            handle.await??;
        }
        
        Ok(())
    }
    
    fn parse_action(&self, action: String) -> Result<usize> {
        action.parse::<usize>()?
            .checked_sub(1)
            .context("Error during user input conversion")
    }
}


enum DownloadState {
    SelectAnime,
    SelectDownloadType,
    SelectEpisode,
    SelectQuality,
    SelectThreadCount,
    Download,
}


impl Default for DownloadState {
    fn default() -> Self {
        Self::SelectAnime
    }
}


#[derive(Clone, PartialEq)]
enum DownloadType {
    OneEpisode(String),
    SomeEpisodes(String),
    RangeEpisodes(String),
    AllEpisodes(String),
}


impl Default for DownloadType {
    fn default() -> Self {
        Self::arr()[0].clone()
    }
}


impl DownloadType {
    fn arr() -> [DownloadType; 4] {
        [
            Self::OneEpisode(String::from("Download one episode")),
            Self::SomeEpisodes(String::from("Download some episodes")),
            Self::RangeEpisodes(String::from("Download range episodes")),
            Self::AllEpisodes(String::from("Download all episodes")),
        ]
    }

    pub fn equal(&self, rhs: &Self) -> bool {
        self.empty() == rhs.empty()
    }

    pub fn empty(&self) -> Self {
        match self {
            Self::OneEpisode(_) => Self::OneEpisode(String::new()),
            Self::SomeEpisodes(_) => Self::SomeEpisodes(String::new()),
            Self::RangeEpisodes(_) => Self::RangeEpisodes(String::new()),
            Self::AllEpisodes(_) => Self::AllEpisodes(String::new()),
        }
    }
    
    fn val(&self) -> String {
        match self {
            Self::OneEpisode(text) => text,
            Self::SomeEpisodes(text) => text,
            Self::RangeEpisodes(text) => text,
            Self::AllEpisodes(text) => text,
        }.clone()
    }
}
