// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    println!("Starting chat-with-geogebra Tauri application...");
    chat_with_geogebra_lib::run()
}
