// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod project;
mod plugins;
mod audio;

use std::path::PathBuf;
use std::str::FromStr;
use crate::plugins::{find_plugins, Plugin};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
fn file_exists(file: &str) -> bool {
    PathBuf::from_str(file).is_ok_and(|f| f.is_file())
}

#[tauri::command]
fn plugins() -> Vec<Plugin> {
    find_plugins(None)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, file_exists, plugins])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
