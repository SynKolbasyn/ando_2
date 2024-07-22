pub enum State {
    MainMenu,
    DownloadMenu,
    SettingsMenu,
}


impl Default for State {
    fn default() -> Self {
        Self::MainMenu
    }
}
