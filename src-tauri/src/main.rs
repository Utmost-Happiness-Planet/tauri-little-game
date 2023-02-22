#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::Write;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn save_log(log: &str) -> bool {
    let path = "log.txt";
    let mut output = match File::create(path) {
        Ok(a) => a,
        Err(_) => return false
    };
    match write!(output, "{}", String::from(log)){
        Ok(_) => return true,
        Err(_) => return false
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
