use crate::parse::interpret;
use std::{cell::RefCell, collections::HashMap};
use std::rc::Rc;



pub struct Parameter {
    pub expression: Vec<String>,
    pub value: f64,
}
impl Parameter {
    pub fn update_value(&mut self, parameters: &Vec<Rc<RefCell<Parameter>>>) { 
        let operators = HashMap::from([
            ('+', Box::new(|lhs: f64, rhs: f64| lhs + rhs) as Box<dyn Fn(f64, f64) -> f64>),
            ('-', Box::new(|lhs: f64, rhs: f64| lhs - rhs)),
            ('/', Box::new(|lhs: f64, rhs: f64| lhs / rhs)),
            ('*', Box::new(|lhs: f64, rhs: f64| lhs * rhs)),
            ('^', Box::new(|lhs: f64, rhs: f64| lhs.powf(rhs))),
            ('s', Box::new(|_lhs: f64, rhs: f64| rhs.sin())),
        ]);
        self.value = interpret(&self.expression, &operators, parameters,1)
        
    }
}
impl Default for Parameter {
    fn default() -> Self {
        Parameter { expression: vec![String::from("0")], value: 0.0 }
    }
}