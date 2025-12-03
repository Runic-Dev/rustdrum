use rspotify::model::{FullArtist, Page, SearchResult, SearchType};
use rspotify::ClientResult;
use rspotify::{prelude::BaseClient, ClientCredsSpotify, Credentials};
use tauri::async_runtime::block_on;

pub struct RSpotifyService {
    spotify: ClientCredsSpotify,
}

impl RSpotifyService {
    pub fn new() -> Self {
        let creds = Credentials::from_env().expect("Problem sourcing credentials env");

        let spotify = ClientCredsSpotify::new(creds);

        match block_on(spotify.request_token()) {
            Ok(_) => {
                dbg!("Token request was successful!");
            }
            Err(err) => panic!("Unable to get token : {}", err),
        }
        Self { spotify }
    }

    pub async fn request_token(&self) -> ClientResult<()> {
        self.spotify.request_token().await
    }

    pub async fn search_artist(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        let query = self
            .spotify
            .search(query, SearchType::Artist, None, None, None, None);

        let res = query.await?;

        if let SearchResult::Artists(page_artists) = res {
            match Self::get_top_artist(&page_artists) {
                Some(url) => Ok(url),
                None => Ok(String::new()),
            }
        } else {
            Ok(String::new())
        }
    }

    fn get_top_artist(full_artists_page: &Page<FullArtist>) -> Option<String> {
        Some(full_artists_page.items.first()?.images.first()?.url.clone())
    }
}
