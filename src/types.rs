use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Torrent {
    pub name: String,
    pub size: String,
    pub uploader: String,
    pub age: String,
    pub seed: String,
    pub leech: String,
    pub link: String,
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
