use core::panic;

use rspotify::{prelude::BaseClient, ClientCredsSpotify, Credentials};
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
    match block_on(spotify.search(
        "Kvelertak",
        rspotify::model::SearchType::Artist,
        None,
        None,
        None,
        None,
    )) {
        Ok(search_result) => {
            println!("Well well!");
            dbg!(search_result);
        }
        Err(err) => {
            panic!("Oh shit! : {}", err);
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
