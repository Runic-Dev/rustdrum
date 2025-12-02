use core::panic;

use rspotify::{
    model::AlbumId, prelude::*, scopes, AuthCodeSpotify, ClientCredsSpotify, Credentials, OAuth,
};
use tauri::async_runtime::block_on;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let creds = Credentials::from_env().expect("Problem sourcing credentials env");

    let spotify = ClientCredsSpotify::new(creds);

    match block_on(spotify.request_token()) {
        Ok(_) => {
            dbg!("Token request was successful!");
        }
        Err(err) => panic!("Oh shit! : {}", err),
    }

    let birdy_uri = AlbumId::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj")
        .expect("Failed to create album id");
    match block_on(spotify.album(birdy_uri, None)) {
        Ok(album) => {
            dbg!("Well would you look at that!");
            dbg!(album);
        }
        Err(err) => {
            panic!("Oh shit! : {}", err);
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
