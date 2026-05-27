pub enum Value<T: Fn() -> f64> {
    Constant(f64),
    Expression(T),
}
impl<T: Fn() -> f64> Value<T> {
    pub fn value(&self) -> f64 {
        match self {
            Value::Constant(constant) => *constant,
            Value::Expression(expression) => expression(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn value_test_constant() {
        let a = Value::<fn() -> f64>::Constant(0.0);
        let a_value = a.value();

        let expected_value = 0.0;
        assert_eq!(expected_value, a_value);
    }

    #[test]
    fn value_test_expression() {
        let value = Rc::new(RefCell::new(5.0));
        let a = Value::Expression(|| *value.borrow());
        let a_value1 = a.value();

        let expected_value1 = 5.0;

        *value.borrow_mut() += 1.0;

        let a_value2 = a.value();
        let expected_value2: f64 = 6.0;
        assert_eq!(expected_value1, a_value1);
        assert_eq!(expected_value2, a_value2);
    }
}
