#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scene::render::{Image, Color};
use solver;
use std::sync::{LazyLock, Mutex};

static SCREEN: LazyLock<Mutex<Image>> = LazyLock::new(|| Mutex::new(Image::default()));
static EMPTY_SCREEN: LazyLock<Image> = LazyLock::new(|| Image::from((1000, 1000)));

#[tauri::command]
fn update_canvas() -> Vec<u8> {
    (*SCREEN.lock().unwrap()).to_canvas()
}
#[tauri::command]
fn create_canvas(width: usize, height: usize) {
    *SCREEN.lock().unwrap() = Image::from((width, height));
}

#[tauri::command]
fn clear_canvas() {
    *SCREEN.lock().unwrap() = (*EMPTY_SCREEN).clone();
}

#[tauri::command]
fn set_pixel(row: usize, colum: usize, c: [u8; 3]){
    let color = Color(c[0], c[1], c[2]);
    (*SCREEN.lock().unwrap()).canvas[row][colum] = color;
}
pub fn run() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![update_canvas, create_canvas, clear_canvas, set_pixel])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
