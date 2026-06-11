/*
TODO:
    - improve tokenizer error handling
    - improve test cases
    - add enum for operators("(", ")", operator, and function)?
    - clean code even more(FAHHHH)
    - other performance shi tbh
    - expression simplification(?)
*/

use std::{cell::RefCell, collections::HashMap};
use crate::parameter;
use std::rc::Rc;
enum TokenState {
    Beginning,
    Constant,
    Parameter,
    Operator,
    Parantheses,
}

pub fn tokenize(raw_expression: &String, operators: &String, mnemonics: &Vec<(String, String)>) -> Vec<String> {
    let mut expression = raw_expression.clone(); // clone might make this more inneficient than my original solution 
    
    for (from, to) in mnemonics {
        expression = expression.replace(from, to);
    }
    expression.make_ascii_lowercase();
    expression = expression.replace(" ", "");

    let mut tokens: Vec<String> = Vec::new();
    let mut token = String::new();
    let mut token_type = TokenState::Beginning;

    for character in expression.chars() {
        match token_type {
            TokenState::Beginning => {
                token.push(character);
                match character {
                    '0'..='9' => token_type = TokenState::Constant,
                    'p' => token_type = TokenState::Parameter,
                    '(' => token_type = TokenState::Parantheses,
                    character if operators.contains(character) => {
                        //inelegant way to handle prefix expressions because it will accept non prefix ones too
                        token = String::from(character);
                        token_type = TokenState::Operator;
                    }
                    _ => panic!("invalid expression"),
                }
            },

            TokenState::Constant => match character {
                '0'..='9' | '.' => token.push(character),
                'p' => {
                    tokens.push(token);
                    tokens.push(String::from("*"));

                    token = String::from("p");
                    token_type = TokenState::Parameter;
                },

                character if operators.contains(character) => {
                    tokens.push(token);
                    token = String::from(character);

                    token_type = TokenState::Operator;
                },

                '(' => {
                    tokens.push(token);
                    tokens.push(String::from("*"));

                    token = String::from("(");
                    token_type = TokenState::Operator;
                },

                ')' => {
                    tokens.push(token);
                    token = String::from(")");
                },

                _ => panic!("invalid expression"),
            },

            TokenState::Parameter => match character {
                '0'..='9' => token.push(character),
                'p' => {
                    tokens.push(token);
                    tokens.push(String::from("*"));
                    token = String::from("p");
                },

                character if operators.contains(character) => {
                    tokens.push(token);
                    token = String::from(character);

                    token_type = TokenState::Operator;
                },

                '(' => {
                    tokens.push(token);
                    tokens.push(String::from("*"));

                    token = String::from("(");
                    token_type = TokenState::Operator;
                },

                ')' => {
                    tokens.push(token);
                    token = String::from(")");
                },

                _ => panic!("invalid expression"),
            },

            TokenState::Operator => {
                tokens.push(token);
                token = String::from(character);

                match character {
                    '0'..='9' => token_type = TokenState::Constant,
                    'p' => token_type = TokenState::Parameter,
                    '(' => token_type = TokenState::Parantheses,
                    character if operators.contains(character) => token_type = TokenState::Operator, //buns ass logic that will allow for so many errors
                    _ => panic!("invalid expression"),
                }
            },

            TokenState::Parantheses => {
                tokens.push(token);
                token = String::new();
                token.push(character);

                match character {
                    '0'..='9' => token_type = TokenState::Constant,
                    'p' => token_type = TokenState::Parameter,
                    character if operators.contains(character) => token_type = TokenState::Operator, //buns ass logic that will allow for so many errors
                    _ => panic!("invalid expression"),
                }
            },
        }
    }
    tokens.push(token); // pushs last token on stack
    tokens
}

//shunting yard algorithm
//autism reference
//needs to be refactored again because ts buns
pub fn parse(mut tokens: Vec<String>, precidence: &HashMap<char, i32>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut operator_stack: Vec<char> = Vec::new();
    let mut operators = String::new();
    
    for (operator, _precidence) in precidence {
        operators.push(*operator);
    }

    let mut temp: char = ' ';
    tokens.push(String::from(" "));
    for token in tokens {
        loop {
            let stack_length = operator_stack.len();
            if stack_length == 0 {
                if temp != ' ' {
                    operator_stack.push(temp);
                    temp = ' ';
                }
                break;
            }

            let stack_precidence = precidence.get(&operator_stack[stack_length - 1]).copied().unwrap_or(0);
            let temp_precidence = precidence.get(&temp).copied().unwrap_or(i32::MAX);
            match temp {
                ')' if operator_stack[stack_length - 1] == '(' => {
                    operator_stack.pop();
                    temp = ' ';
                    break;
                },

                ')' => output.push(String::from(operator_stack.pop().unwrap())),
                _operator if stack_precidence.abs() > temp_precidence.abs() => {
                    output.push(String::from(operator_stack.pop().unwrap()))
                },

                _operator if stack_precidence.abs() == temp_precidence => {
                    output.push(String::from(operator_stack.pop().unwrap()))
                },

                ' ' => break,

                _ => {
                    operator_stack.push(temp);
                    temp = ' ';
                    break;
                },
            }
        }

        match token.chars().next().unwrap_or('0') {
            operator if operators.contains(operator) => temp = operator,
            parenthenses @ ('(' | ')') => temp = parenthenses,
            _ => output.push(token),
        }
    }

    output.pop();

    operator_stack.reverse();
    for operator in operator_stack {
        output.push(String::from(operator));
    }

    output
}

pub fn interpret(expression: &Vec<String>,parameters: &Vec<Rc<RefCell<parameter::Parameter>>>, depth: u8) -> f64 {
    let operators= HashMap::from([
                    ('+', Box::new(|lhs: f64, rhs: f64| vec![(lhs + rhs).to_string()]) as Box<dyn Fn(f64, f64) -> Vec<String>>),
                    ('-', Box::new(|lhs: f64, rhs: f64| vec![(lhs - rhs).to_string()])),
                    ('/', Box::new(|lhs: f64, rhs: f64| vec![(lhs / rhs).to_string()])),
                    ('*', Box::new(|lhs: f64, rhs: f64| vec![(lhs * rhs).to_string()])),
                    ('^', Box::new(|lhs: f64, rhs: f64| vec![(lhs.powf(rhs)).to_string()])),
                    ('s', Box::new(|lhs: f64, rhs: f64| match lhs {
                        0.0 => vec![rhs.sin().to_string()],
                        _   => vec![lhs.to_string(), rhs.sin().to_string()],
                    })),
                    ('c', Box::new(|lhs: f64, rhs: f64| match lhs {
                        0.0 => vec![rhs.cos().to_string()],
                        _   => vec![lhs.to_string(), rhs.cos().to_string()],
                    })),
                    ('t', Box::new(|lhs: f64, rhs: f64| match lhs {
                        0.0 => vec![rhs.tan().to_string()],
                        _   => vec![lhs.to_string(), rhs.tan().to_string()],
                    })),
                ]);
    
    let mut output: Vec<String> = Vec::new();
    
    let mut symbols = String::new();
    for (symbol, _closure) in &operators {
        symbols.push(*symbol);
    }
    let mut left_hand_side: f64;
    let mut right_hand_side: f64;

    for element in expression {
        match element.chars().next().unwrap_or('0') {
            operator if symbols.contains(operator) && element.len() == 1 => {
                right_hand_side = output.pop().unwrap_or(String::from("0")).parse().unwrap_or(0.0);
                left_hand_side = output.pop().unwrap_or(String::from("0")).parse().unwrap_or(0.0);
                
                let value = (operators.get(&operator).unwrap())(left_hand_side, right_hand_side);
                output.extend(value);
            },

            'p' => {
                let index: usize = element[1..].parse().unwrap_or(0);
                if depth == 0 {
                    parameters[index].borrow_mut().update_value(parameters);
                }
                output.push(parameters[index].borrow().value.to_string());
            },
            _ => output.push(element.clone()),
        }
    }
    
    output[0].parse().unwrap()
}

pub fn simplify(expression: &Vec<String>) -> Vec<String> {
    let operators= HashMap::from([
                    ('+', Box::new(|lhs: f64, rhs: f64| vec![(lhs + rhs).to_string()]) as Box<dyn Fn(f64, f64) -> Vec<String>>),
                    ('-', Box::new(|lhs: f64, rhs: f64| vec![(lhs - rhs).to_string()])),
                    ('/', Box::new(|lhs: f64, rhs: f64| vec![(lhs / rhs).to_string()])),
                    ('*', Box::new(|lhs: f64, rhs: f64| vec![(lhs * rhs).to_string()])),
                    ('^', Box::new(|lhs: f64, rhs: f64| vec![(lhs.powf(rhs)).to_string()])),
                    ('s', Box::new(|lhs: f64, rhs: f64| match lhs {
                        0.0 => vec![rhs.sin().to_string()],
                        _   => vec![lhs.to_string(), rhs.sin().to_string()],
                    })),
                    ('c', Box::new(|lhs: f64, rhs: f64| match lhs {
                        0.0 => vec![rhs.cos().to_string()],
                        _   => vec![lhs.to_string(), rhs.cos().to_string()],
                    })),
                    ('t', Box::new(|lhs: f64, rhs: f64| match lhs {
                        0.0 => vec![rhs.tan().to_string()],
                        _   => vec![lhs.to_string(), rhs.tan().to_string()],
                    })),
                ]);
    let mut output: Vec<String> = Vec::new();

    let mut symbols = String::new();
    for (symbol, _closure) in &operators {
        symbols.push(*symbol);
    }
    symbols.push('p');
    let mut left_hand_side_token: String;
    let mut right_hand_side_token: String;

    for element in expression {
        
        match element.chars().next().unwrap_or('0') {
            operator if symbols.contains(operator) && element.len() == 1 => {
                right_hand_side_token = output.pop().unwrap_or(String::from("0"));
                let right_char = right_hand_side_token.chars().next().unwrap_or('0');
                left_hand_side_token = output.pop().unwrap_or(String::from("0"));
                let left_char = left_hand_side_token.chars().next().unwrap_or('0');
                if symbols.contains(right_char) && right_hand_side_token.len() == 1
                 ||  symbols.contains(left_char) && left_hand_side_token.len() == 1 {
                    output.push(left_hand_side_token);
                    output.push(right_hand_side_token);
                    output.push(element.clone());
                } else if right_char == 'p' || left_char == 'p' {
                    output.push(left_hand_side_token);
                    output.push(right_hand_side_token);
                    output.push(element.clone());
                } else {
                    let left_hand_side = left_hand_side_token.parse().unwrap_or(0.0);
                    let right_hand_side = right_hand_side_token.parse().unwrap_or(0.0);
                    let value = (operators.get(&operator).unwrap())(left_hand_side, right_hand_side);
                    output.extend(value);
                }
            },
            _ => output.push(element.clone()),
        }
    }
    output
}

#[cfg(test)]
mod tests {
use crate::parameter::Parameter;

use super::*;
    struct TestExpressions {
        raw_expression_whitespace: String,
        raw_expression_parenthenses: String,
        raw_expression_variable: String,
        raw_expression_prefix: String,

        tokens_whitespace: Vec<String>,
        tokens_parenthenses: Vec<String>,
        tokens_variable: Vec<String>,
        tokens_prefix: Vec<String>,

        expression_whitespace: Vec<String>,
        expression_parenthenses: Vec<String>,
        expression_variable: Vec<String>,
        expression_prefix: Vec<String>,

        result_whitespace: f64,
        result_parenthenses: f64,
        result_variable: f64,
        result_prefix: f64,
 
        operator_string: String,
        precidence: HashMap<char, i32>,
        mnemonics: Vec<(String, String)>,
        parameters: Vec<Rc<RefCell<parameter::Parameter>>>, //the worlds ugliest concrete type
    }
    impl Default for TestExpressions {
        fn default() -> TestExpressions {
            TestExpressions {
                raw_expression_whitespace: String::from(" 1.2/7 - 6 + 3 . 0 *   10   "),
                raw_expression_parenthenses: String::from("(1-3)/(2+1)"),
                raw_expression_variable: String::from("2p3+p1/2+3.1(2+1)"),
                raw_expression_prefix: String::from("sin(3.14159265358979) + 1 - p0"),

                tokens_whitespace: vec![
                    String::from("1.2"),
                    String::from("/"),
                    String::from("7"),
                    String::from("-"),
                    String::from("6"),
                    String::from("+"),
                    String::from("3.0"),
                    String::from("*"),
                    String::from("10"),
                ],
                tokens_parenthenses: vec![
                    String::from("("),
                    String::from("1"),
                    String::from("-"),
                    String::from("3"),
                    String::from(")"),
                    String::from("/"),
                    String::from("("),
                    String::from("2"),
                    String::from("+"),
                    String::from("1"),
                    String::from(")"),
                ],
                tokens_variable: vec![
                    String::from("2"),
                    String::from("*"),
                    String::from("p3"),
                    String::from("+"),
                    String::from("p1"),
                    String::from("/"),
                    String::from("2"),
                    String::from("+"),
                    String::from("3.1"),
                    String::from("*"),
                    String::from("("),
                    String::from("2"),
                    String::from("+"),
                    String::from("1"),
                    String::from(")"),
                ],
                tokens_prefix: vec![
                    String::from("s"),
                    String::from("("),
                    String::from("3.14159265358979"),
                    String::from(")"),
                    String::from("+"),
                    String::from("1"),
                    String::from("-"),
                    String::from("p0"),
                ],

                expression_whitespace: vec![
                    String::from("1.2"),
                    String::from("7"),
                    String::from("/"),
                    String::from("6"),
                    String::from("-"),
                    String::from("3.0"),
                    String::from("10"),
                    String::from("*"),
                    String::from("+"),
                ],
                expression_parenthenses: vec![
                    String::from("1"),
                    String::from("3"),
                    String::from("-"),
                    String::from("2"),
                    String::from("1"),
                    String::from("+"),
                    String::from("/"),
                ],
                expression_variable: vec![
                    String::from("2"),
                    String::from("p3"),
                    String::from("*"),
                    String::from("p1"),
                    String::from("2"),
                    String::from("/"),
                    String::from("+"),
                    String::from("3.1"),
                    String::from("2"),
                    String::from("1"),
                    String::from("+"),
                    String::from("*"),
                    String::from("+"),
                ],
                expression_prefix: vec![
                    String::from("3.14159265358979"),
                    String::from("s"),
                    String::from("1"),
                    String::from("+"),
                    String::from("p0"),
                    String::from("-"),
                ],

                result_whitespace: 24.17142857142857,
                result_parenthenses: -0.6666666666666666,
                result_variable: 25.3,
                result_prefix: -6.9999999999999964, //aproaches -7 with more digits of pi

                operator_string: String::from("s^*/-+"),
                precidence: HashMap::from([
                    ('+', 1),
                    ('-', 1),
                    ('/', 2),
                    ('*', 2),
                    ('^', -3),
                    ('s', 4),
                ]),
                mnemonics: vec![(String::from("sin"), String::from("s"))],
                parameters: vec![
                    Rc::new(RefCell::new(Parameter {
                        expression: vec![String::from("8")],
                        value: 8.0,
                    })),
                    Rc::new(RefCell::new(Parameter {
                        expression: vec![String::from("0")],
                        value: 0.0,
                    })),
                    Rc::new(RefCell::new(Parameter {
                        expression: vec![String::from("0")],
                        value: 0.0,
                    })),
                    Rc::new(RefCell::new(Parameter {
                        expression: vec![String::from("8")],
                        value: 8.0,
                    })),
                    Rc::new(RefCell::new(Parameter {
                        expression: vec![String::from("5")],
                        value: 5.0,
                    })),
                ],
            }
        }
    }

    #[test]
    fn tokenize_whitespace_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            tokenize(
                &test_expressions.raw_expression_whitespace,
                &test_expressions.operator_string,
                &test_expressions.mnemonics
            ),
            test_expressions.tokens_whitespace
        )
    }

    #[test]
    fn tokenize_parenthenses_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            tokenize(
                &test_expressions.raw_expression_parenthenses,
                &test_expressions.operator_string,
                &test_expressions.mnemonics
            ),
            test_expressions.tokens_parenthenses
        )
    }

    #[test]
    fn tokenize_variable_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            tokenize(
                &test_expressions.raw_expression_variable,
                &test_expressions.operator_string,
                &test_expressions.mnemonics
            ),
            test_expressions.tokens_variable
        )
    }
    #[test]
    fn tokenize_prefix_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            tokenize(
                &test_expressions.raw_expression_prefix,
                &test_expressions.operator_string,
                &test_expressions.mnemonics
            ),
            test_expressions.tokens_prefix
        )
    }

    #[test]
    fn parse_whitespace_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            parse(
                test_expressions.tokens_whitespace.clone(),
                &test_expressions.precidence
            ),
            test_expressions.expression_whitespace
        )
    }

    #[test]
    fn parse_parenthenses_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            parse(
                test_expressions.tokens_parenthenses.clone(),
                &test_expressions.precidence
            ),
            test_expressions.expression_parenthenses
        )
    }

    #[test]
    fn parse_variable_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            parse(
                test_expressions.tokens_variable.clone(),
                &test_expressions.precidence
            ),
            test_expressions.expression_variable
        )
    }

    #[test]
    fn parse_prefix_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            parse(
                test_expressions.tokens_prefix.clone(),
                &test_expressions.precidence
            ),
            test_expressions.expression_prefix
        )
    }

    #[test]
    fn interpret_whitespace_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            interpret(
                &test_expressions.expression_whitespace,                
                &test_expressions.parameters,
                0
            ),
            test_expressions.result_whitespace
        )
    }

    #[test]
    fn interpret_parenthenses_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            interpret(
                &test_expressions.expression_parenthenses,                
                &test_expressions.parameters,
                0
            ),
            test_expressions.result_parenthenses
        )
    }

    #[test]
    fn interpret_variable_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            interpret(
                &test_expressions.expression_variable,               
                &test_expressions.parameters,
                0
            ),
            test_expressions.result_variable
        )
    }

    #[test]
    fn interpret_prefix_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            interpret(
                &test_expressions.expression_prefix,
                &test_expressions.parameters,
                0
            ),
            test_expressions.result_prefix
        )
    }

    #[test]
    fn simplify_test() {
        let test_expressions = TestExpressions::default();
        let expected_result = vec![
                    String::from("2"),
                    String::from("p3"),
                    String::from("*"),
                    String::from("p1"),
                    String::from("2"),
                    String::from("/"),
                    String::from("+"),
                    String::from("9.3"),
                    String::from("+"),
                ];
        assert_eq!(expected_result, simplify(&test_expressions.expression_variable))
    }
}
