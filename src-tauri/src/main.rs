// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod git_utilities;

use crate::git_utilities::{branches::local_branches, is_git_available, is_it_repository};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            is_git_available,
            is_it_repository,
            local_branches
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
