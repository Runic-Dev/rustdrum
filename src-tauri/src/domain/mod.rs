use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ArtistsResponse {
    pub items: Vec<ArtistInfo>,
}

impl ArtistsResponse {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ArtistInfo {
    spotify_id: String,
    name: String,
    images: Vec<Image>,
    followers: u32,
    genres: Vec<String>,
}

impl ArtistInfo {
    pub fn new(
        spotify_id: &str,
        name: &str,
        images: Vec<Image>,
        followers: u32,
        genres: Vec<String>,
    ) -> Self {
        Self {
            spotify_id: spotify_id.to_string(),
            name: name.to_string(),
            images,
            followers,
            genres,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    url: String,
    height: Option<u32>,
    width: Option<u32>,
}

impl Image {
    pub fn new(url: String, height: Option<u32>, width: Option<u32>) -> Self {
        Self { url, height, width }
    }
}
