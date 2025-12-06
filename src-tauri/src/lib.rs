use std::sync::Arc;
mod application;
mod architecture;
mod domain;

use crate::domain::ArtistsResponse;
use crate::{application::services::SpotifyService, architecture::RSpotifyAdapter};

use serde::Deserialize;
use serde::Serialize;
use tauri::{
    async_runtime::{block_on, Mutex},
    State,
};

struct AppState {
    spotify_service: SpotifyService<RSpotifyAdapter>,
}

#[derive(Serialize)]
struct SuccessResponse<T: Serialize> {
    content: T,
    success: bool,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn new(content: T) -> Self {
        Self {
            content,
            success: true,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct FailureResponse {
    error: String,
    success: bool,
}

impl FailureResponse {
    pub fn new(error: String) -> Self {
        Self {
            error: error,
            success: false,
        }
    }
}

#[tauri::command]
async fn search_artists(
    app_state: State<'_, Arc<Mutex<AppState>>>,
    artist_name: String,
) -> Result<SuccessResponse<ArtistsResponse>, FailureResponse> {
    let app_state = app_state.lock().await;
    let spotify_service = &app_state.spotify_service;
    spotify_service
        .search_artists(&artist_name)
        .await
        .map(|res| SuccessResponse::new(res))
        .map_err(|oe| FailureResponse::new(oe.to_string()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let adapter = Arc::from(RSpotifyAdapter::new());
    let spotify_service = SpotifyService::new(adapter);
    let token_res = block_on(spotify_service.request_token());
    if token_res.is_err() {
        panic!("The token request failed!");
    }

    let app_state = AppState { spotify_service };

    tauri::Builder::default()
        .manage(Arc::from(Mutex::new(app_state)))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![search_artists])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
