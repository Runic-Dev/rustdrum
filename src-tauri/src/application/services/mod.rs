use std::sync::Arc;

use crate::{
    architecture::{ArtistsSearchError, SpotifyServiceAdapter, TokenRequestError},
    domain::ArtistsResponse,
};

pub struct SpotifyService<T>
where
    T: SpotifyServiceAdapter,
{
    adapter: Arc<T>,
}

impl<T: SpotifyServiceAdapter> SpotifyService<T> {
    pub fn new(adapter: Arc<T>) -> Self {
        Self { adapter }
    }

    pub async fn request_token(&self) -> Result<(), TokenRequestError> {
        self.adapter.request_token().await
    }

    pub async fn search_artists(
        &self,
        artist_name: &str,
    ) -> Result<ArtistsResponse, ArtistsSearchError> {
        self.adapter.get_artists_by_name(artist_name).await
    }
}
