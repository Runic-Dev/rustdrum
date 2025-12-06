use crate::domain::ArtistInfo;
use crate::domain::ArtistsResponse;
use std::fmt::Display;

use rspotify::ClientCredsSpotify;
use rspotify::Credentials;
use rspotify::{
    model::{Id, SearchResult},
    prelude::BaseClient,
};

use crate::domain::Image;

pub struct RSpotifyAdapter {
    library: rspotify::ClientCredsSpotify,
}

impl SpotifyServiceAdapter for RSpotifyAdapter {
    async fn get_artists_by_name(
        &self,
        artist_name: &str,
    ) -> Result<ArtistsResponse, ArtistsSearchError> {
        let res = self
            .library
            .search(
                artist_name,
                rspotify::model::SearchType::Artist,
                None,
                None,
                None,
                None,
            )
            .await
            .map_err(|err| ArtistsSearchError {
                error: err.to_string(),
            })?;

        match res {
            SearchResult::Artists(page) => {
                let artists_responses = page.items.into_iter().fold(
                    Vec::<ArtistInfo>::new(),
                    |mut res_vec, full_artist| {
                        res_vec.push(ArtistInfo::new(
                            full_artist.id.id(),
                            &full_artist.name,
                            Self::map_images(full_artist.images),
                            full_artist.followers.total,
                            full_artist.genres,
                        ));
                        res_vec
                    },
                );
                Ok(ArtistsResponse {
                    items: artists_responses,
                })
            }
            _ => Err(ArtistsSearchError::new("Expected artist response")),
        }
    }

    async fn request_token(&self) -> Result<(), TokenRequestError> {
        self.library
            .request_token()
            .await
            .map_err(|original_err| TokenRequestError::new(original_err.to_string()))
    }
}

#[derive(Debug)]
pub struct TokenRequestError {
    error: String,
}

impl TokenRequestError {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

impl Display for TokenRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error requesting token: {}", self.error)
    }
}

impl std::error::Error for TokenRequestError {}

impl RSpotifyAdapter {
    fn map_images(original: Vec<rspotify::model::Image>) -> Vec<Image> {
        original
            .into_iter()
            .map(|oi| Image::new(oi.url, oi.height, oi.width))
            .collect()
    }

    pub fn new() -> Self {
        let creds = Credentials::from_env().expect("Problem sourcing credentials");
        Self {
            library: ClientCredsSpotify::new(creds),
        }
    }
}

pub trait SpotifyServiceAdapter {
    async fn get_artists_by_name(
        &self,
        artist_name: &str,
    ) -> Result<ArtistsResponse, ArtistsSearchError>;

    async fn request_token(&self) -> Result<(), TokenRequestError>;
}

#[derive(Debug)]
pub struct ArtistsSearchError {
    error: String,
}

impl ArtistsSearchError {
    pub fn new(error: &str) -> Self {
        Self {
            error: String::from(error),
        }
    }
}

impl Display for ArtistsSearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error searching for artists: {}", self.error)
    }
}

impl std::error::Error for ArtistsSearchError {}
