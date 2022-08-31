use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Torrent {
    pub id: String,
    pub name: String,
    pub size: String,
    pub uploader: String,
    pub age: String,
    pub seed: String,
    pub leech: String,
    #[tabled(skip)]
    pub link: String,
    #[tabled(skip)]
    pub category: String,
}

pub struct SearchOptions {
    pub search_in_tv: bool,
    pub search_in_movies: bool,
    pub search_in_music: bool,
    pub search_in_documentaries: bool,
    pub search_in_anime: bool,
    pub search_in_xxx: bool,
    pub search_in_games: bool,
    pub search_in_apps: bool,
    pub search_in_other: bool,
}

pub struct PopularOptions {
    pub popular_in_tv: bool,
    pub popular_in_movies: bool,
    pub popular_in_music: bool,
    pub popular_in_documentaries: bool,
    pub popular_in_anime: bool,
    pub popular_in_xxx: bool,
    pub popular_in_games: bool,
    pub popular_in_apps: bool,
    pub popular_in_other: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TorrentDetails {
    pub id: String,
    pub name: String,
    pub files: Vec<String>,
    pub magnet: String,
    pub seeders: String,
    pub leechers: String,
    pub size: String,
    pub trackers: Vec<String>,
}
