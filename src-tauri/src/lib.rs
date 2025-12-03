use std::sync::Arc;
mod spotify_client;

use tauri::{
    async_runtime::{block_on, Mutex},
    State,
};

use crate::spotify_client::RSpotifyService;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn search_artist(
    spotify_service: State<'_, Arc<Mutex<RSpotifyService>>>,
    artist_name: &str,
) -> Result<String, String> {
    let locked_service = spotify_service.lock().await;
    let result = locked_service.search_artist(&artist_name).await;
    if let Ok(url_str) = result {
        return Ok(url_str);
    }
    Ok(String::new())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let spotify_service = RSpotifyService::new();
    if let Err(_) = block_on(spotify_service.request_token()) {
        todo!("This should be a UI error");
    }

    tauri::Builder::default()
        .manage(Arc::from(Mutex::new(spotify_service)))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![search_artist])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
