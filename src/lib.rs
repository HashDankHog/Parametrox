//this file implements constructive solid geometry trees
//  methods for converting b-rep to csg and vice versa can be found in /src/bin/CAD
//  
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scene::render::{Image, Color};
use solver;
use std::sync::{LazyLock, Mutex};

static SCREEN: LazyLock<Mutex<Vec<u8>>> = LazyLock::new(|| Mutex::new(vec![0]));
static EMPTY_SCREEN: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let mut a: Vec<u8> = vec![];
    for _ in 0..1000000 {
        a.push(0);
        a.push(0);
        a.push(0);
        a.push(255);
    }
    a
});
static HEIGHT: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(0));
static WIDTH: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(0));
#[tauri::command]
fn update_canvas() -> Vec<u8> {
    (*SCREEN.lock().unwrap()).clone()
}
#[tauri::command]
fn create_canvas(width: usize, height: usize) {
    *SCREEN.lock().unwrap() = vec![0; width*height*4];
    *WIDTH.lock().unwrap() = width;
    *HEIGHT.lock().unwrap() = height;
}

#[tauri::command]
fn clear_canvas() {
    *SCREEN.lock().unwrap() = (*EMPTY_SCREEN).clone();
}

#[tauri::command]
fn set_pixel(row: usize, colum: usize, c: [u8; 3]){
    
    let mut temp = SCREEN.lock().unwrap();
    let width = *WIDTH.lock().unwrap();
    (*temp)[4*(width*row+colum)] = c[0];
    (*temp)[4*(width*row+colum)+1] = c[1];
    (*temp)[4*(width*row+colum)+2] = c[2];
    (*temp)[4*(width*row+colum)+3] = 255;
}

#[tauri::command]
fn draw_rect(coord:(usize, usize), size: (usize, usize), color: [u8;3]){
    for row in (coord.1)..(coord.1+size.1) {
        for colum in (coord.0)..(coord.0+size.0) {
            set_pixel(row, colum, color);
        }
    }
}
pub fn run() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![update_canvas, create_canvas, clear_canvas, set_pixel, draw_rect])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
pub fn bench() {
    use std::time::Instant;
    let now = Instant::now();

    create_canvas(1000, 1000);
    update_canvas();
    for i in 100..200 {
        clear_canvas();
        for row in 0..100 {
            for colum in 0..100{
                set_pixel(i+row, i+colum, [255,255,255]);
            }
        }
        update_canvas();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}