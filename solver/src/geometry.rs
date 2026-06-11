//this file might be deleted at a later date, but currently this file might contain structures and methods related to 
//fundemental geometry operations
//AKA planes, spheres, distances, segments/lines, intersections, and other things ig idk
use crate::parameter::Parameter;
use std::{rc::Rc, cell::RefCell};

pub struct Profile {
    pub parameters: Vec<Rc<RefCell<Parameter>>>,
    pub indices: Vec<usize>,
}
pub struct Segment(Parameter, Parameter);
impl Default for Profile {
    fn default() -> Self {
        Profile {
            parameters: Vec::new(),
            indices: Vec::new(),
        }
    }
}

impl Profile {
    pub fn add_segment(&mut self, segment: Segment) {
        let index = self.parameters.len();
        self.indices.push(index);
        self.parameters.push(Rc::new(RefCell::new(segment.0))); //now that I am looking at this, I am pretty sure I dont need rc<refcell<>>
        self.parameters.push(Rc::new(RefCell::new(segment.1)));
    }

    pub fn plot(&mut self, t: f64) -> (f64, f64){
        *(self.parameters[0].borrow_mut()) = Parameter{expression: vec![t.to_string()], value: t};
        let index = usize::from(t.abs().trunc() as u16); //trunc and abs might be redundent but idgaf to test it rn
        let parameter = self.indices[index];

        self.parameters[parameter].borrow_mut().update_value(&self.parameters);
        let x = self.parameters[parameter].borrow().value;

        self.parameters[parameter + 1].borrow_mut().update_value(&self.parameters);
        let y = self.parameters[parameter + 1].borrow().value;

        (x,y)
    }
}

