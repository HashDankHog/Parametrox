use scene;
use solver;
#[tauri::command]
fn update_canvas() -> Vec<u8> {
    let screen = scene::render::Image::from((1000,1000));
    screen.to_canvas()
}