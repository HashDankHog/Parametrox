use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Color(u8,u8,u8);

#[derive(Serialize, Deserialize)]
pub struct Image {
    canvas: Vec<Vec<Color>>,
    width: usize,
    height: usize,
}
impl From<(usize, usize)> for Image {
    fn from(size: (usize,usize)) -> Self {
        Image {
            canvas: vec![vec![Color(0,0,0); size.0]; size.1],
            width: size.0,
            height: size.1,
        }
    }
}
impl Image {
    pub fn draw_pixel(&mut self, position: (usize, usize), pixel: Color) {
        if position.0 > self.width || position.1 > self.height {
            panic!("cannot draw to pixel outside of sceen size");
        }
        self.canvas[position.0][position.1]=pixel;
    }
    pub fn to_canvas(&self) -> Vec<u8> {
        println!("hi");
        let mut image_data = Vec::new();
        for row in 0..self.height {
            for colum in 0..self.width {
                image_data.push(self.canvas[row][colum].0);
                image_data.push(self.canvas[row][colum].1);
                image_data.push(self.canvas[row][colum].2);
                image_data.push(255);
            }
        }
        image_data
    }
}

/* 
#[cfg(test)]
mod tests {

use tauri::image;

use super::*;

#[test]
fn test(){
    let point = mage
    let a = 
    println!()
}
}
*/