#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scene;
use solver;


#[tauri::command]
fn update_canvas() -> Vec<u8> {
    let screen = scene::render::Image::from((1000,1000));
    screen.to_canvas()
}
pub fn run() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![update_canvas])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
