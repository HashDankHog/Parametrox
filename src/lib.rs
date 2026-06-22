//this file implements constructive solid geometry trees
//  methods for converting b-rep to csg and vice versa can be found in /src/bin/CAD
//  
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use solver::parameter::Parameter;
use solver::parse::{parse, tokenize};
use solver::{geometry::Profile};
use std::sync::{LazyLock, Mutex};
use tauri::ipc::Response;
use std::cell::{RefCell};
use std::rc::Rc;
use std::collections::HashMap;

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

//this function is a complete and utter shit show
//TODO: fix
#[tauri::command]
fn update_parameter(expressions: Vec<String>) -> Vec<f64> {
    let mnemonics = vec![(String::from("sin"), String::from("s")),
                                                (String::from("cos"), String::from("c")),
                                                (String::from("tan"), String::from("t")),
                                                (String::from("pi"), String::from("3.14159265359879")),];
    let operator_string = String::from("+-/c*t^s");
    let precidence = HashMap::from([
        ('+', 1),
        ('-', 1),
        ('/', 2),
        ('*', 2),
        ('^', -3),
        ('s', 4),
        ('c', 4),
        ('t', 4),
    ]);
    let mut profile = Profile::default();
    for expression in expressions {
        let tokens= tokenize(&expression, &operator_string, &mnemonics);

        let parsed_expression = parse(tokens, &precidence);
        profile.parameters.push(Rc::new(RefCell::new(Parameter{ expression: parsed_expression, value: 0.0})));
    }
    for _ in 0..2 {
        for parameter in &profile.parameters {
            parameter.borrow_mut().update_value(&profile.parameters);
        }
    }
    
    let mut values = Vec::new();

    for parameter in &profile.parameters {
            values.push(parameter.borrow().value);
        }

    values
}

#[tauri::command]
fn update_canvas() -> Response {
    tauri::ipc::Response::new((*SCREEN.lock().unwrap()).clone())
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
    .invoke_handler(tauri::generate_handler![update_canvas, create_canvas, clear_canvas, set_pixel, draw_rect, update_parameter])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}