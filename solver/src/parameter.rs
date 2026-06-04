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
                    ('+', Box::new(|lhs: f64, rhs: f64| vec![(lhs + rhs).to_string()]) as Box<dyn Fn(f64, f64) -> Vec<String>>),
                    ('-', Box::new(|lhs: f64, rhs: f64| vec![(lhs - rhs).to_string()])),
                    ('/', Box::new(|lhs: f64, rhs: f64| vec![(lhs / rhs).to_string()])),
                    ('*', Box::new(|lhs: f64, rhs: f64| vec![(lhs * rhs).to_string()])),
                    ('^', Box::new(|lhs: f64, rhs: f64| vec![(lhs.powf(rhs)).to_string()])),
                    ('s', Box::new(|lhs: f64, rhs: f64| vec![lhs.to_string(), rhs.sin().to_string()])),
                    ('c', Box::new(|lhs: f64, rhs: f64| vec![lhs.to_string(), rhs.cos().to_string()])),
                    ('t', Box::new(|lhs: f64, rhs: f64| vec![lhs.to_string(), rhs.tan().to_string()])),
                ]);
        self.value = interpret(&self.expression, &operators, parameters,1)
        
    }
}
impl Default for Parameter {
    fn default() -> Self {
        Parameter { expression: vec![String::from("0")], value: 0.0 }
    }
}