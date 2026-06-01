use std::collections::HashMap;

enum TokenState {
    Beginning,
    Constant,
    Parameter,
    Operator,
    Parantheses,
}

pub fn tokenize(
    raw_expression: &String,
    operators: &String,
    mnemonics: &Vec<(String, String)>,
) -> Vec<String> {
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
            }
            TokenState::Constant => match character {
                '0'..='9' | '.' => token.push(character),
                'p' => {
                    tokens.push(token);
                    tokens.push(String::from("*"));

                    token = String::from("p");
                    token_type = TokenState::Parameter;
                }
                character if operators.contains(character) => {
                    tokens.push(token);
                    token = String::from(character);
                    token_type = TokenState::Operator;
                }
                '(' => {
                    tokens.push(token);
                    tokens.push(String::from("*"));
                    token = String::from("(");
                    token_type = TokenState::Operator;
                }
                ')' => {
                    tokens.push(token);
                    token = String::from(")");
                }
                _ => panic!("invalid expression"),
            },

            TokenState::Parameter => match character {
                '0'..='9' => token.push(character),
                'p' => {
                    tokens.push(token);
                    tokens.push(String::from("*"));
                    token = String::from("p");
                }
                character if operators.contains(character) => {
                    tokens.push(token);
                    token = String::from(character);
                    token_type = TokenState::Operator;
                }
                '(' => {
                    tokens.push(token);
                    tokens.push(String::from("*"));
                    token = String::from("(");
                    token_type = TokenState::Operator;
                }
                ')' => {
                    tokens.push(token);
                    token = String::from(")");
                }
                _ => panic!("invalid expression"),
            },
            TokenState::Operator => {
                tokens.push(token);

                token = String::from(character);

                match character {
                    '0'..='9' => token_type = TokenState::Constant,
                    'p' => token_type = TokenState::Parameter,
                    '(' => token_type = TokenState::Parantheses,
                    _ => panic!("invalid expression"),
                }
            }
            TokenState::Parantheses => {
                tokens.push(token);
                token = String::new();
                token.push(character);
                match character {
                    '0'..='9' => token_type = TokenState::Constant,
                    'p' => token_type = TokenState::Parameter,
                    _ => panic!("invalid expression"),
                }
            }
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
    let mut temp: char = ' ';
    tokens.push(String::from(" "));
    for (operator, _precidence) in precidence {
        operators.push(*operator);
    }
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

            let stack_precidence = precidence
                .get(&operator_stack[stack_length - 1])
                .copied()
                .unwrap_or(0);
            let temp_precidence = precidence.get(&temp).copied().unwrap_or(i32::MAX);
            match temp {
                ')' if operator_stack[stack_length - 1] == '(' => {
                    operator_stack.pop();
                    temp = ' ';
                    break;
                }
                ')' => output.push(String::from(operator_stack.pop().unwrap())),
                _operator if stack_precidence.abs() > temp_precidence.abs() => {
                    output.push(String::from(operator_stack.pop().unwrap()))
                }
                _operator if stack_precidence.abs() == temp_precidence => {
                    output.push(String::from(operator_stack.pop().unwrap()))
                }
                ' ' => break,
                _ => {
                    operator_stack.push(temp);
                    temp = ' ';
                    break;
                }
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

pub fn interpret(
    expression: &Vec<String>,
    operators: &HashMap<char, Box<dyn Fn(f64, f64) -> f64>>,
) -> f64 {
    let mut output: Vec<String> = Vec::new();

    let mut symbols = String::new();
    for (symbol, _closure) in operators {
        symbols.push(*symbol);
    }
    let mut left_hand_side: f64;
    let mut right_hand_side: f64;
    for element in expression {
        match element.chars().next().unwrap_or('0') {
            operator if symbols.contains(operator) => {
                right_hand_side = output.pop().unwrap().parse().unwrap();
                left_hand_side = output.pop().unwrap().parse().unwrap();
                let value = (operators.get(&operator).unwrap())(left_hand_side, right_hand_side);
                output.push(value.to_string());
            }
            _ => output.push(element.clone()),
        }
    }
    output[0].parse().unwrap()
}

#[cfg(test)]
mod tests {
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

        operators: HashMap<char, Box<dyn Fn(f64, f64) -> f64>>,
        operator_string: String,
        precidence: HashMap<char, i32>,
        mnemonics: Vec<(String, String)>,
    }
    impl Default for TestExpressions {
        fn default() -> TestExpressions {
            TestExpressions {
                raw_expression_whitespace: String::from(" 1.2/7 - 6 + 3 . 0 *   10   "),
                raw_expression_parenthenses: String::from("(1-3)/(2+1)"),
                raw_expression_variable: String::from("2p3+p1/2+3.1(2+1)"),
                raw_expression_prefix: String::from("sin(3.14159) + 1 - p0"),

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
                    String::from("3.14159"),
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
                    String::from("3.14159"),
                    String::from("s"),
                    String::from("1"),
                    String::from("+"),
                    String::from("p0"),
                    String::from("-"),
                ],

                result_whitespace: 24.17142857142857,
                result_parenthenses: -00.6666666666666666,
                result_variable: 11.8,
                result_prefix: 1.0,

                operators: HashMap::from([
                    (
                        '+',
                        Box::new(|lhs: f64, rhs: f64| lhs + rhs) as Box<dyn Fn(f64, f64) -> f64>,
                    ),
                    ('-', Box::new(|lhs: f64, rhs: f64| lhs - rhs)),
                    ('/', Box::new(|lhs: f64, rhs: f64| lhs / rhs)),
                    ('*', Box::new(|lhs: f64, rhs: f64| lhs * rhs)),
                ]),
                operator_string: String::from("s^*/-+"),
                precidence: HashMap::from([
                    ('+', 1),
                    ('-', 1),
                    ('/', 2),
                    ('*', 2),
                    ('^', 3),
                    ('s', 4),
                ]),
                mnemonics: vec![(String::from("sin"), String::from("s"))],
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
                &test_expressions.operators
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
                &test_expressions.operators
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
                &test_expressions.operators
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
                &test_expressions.operators
            ),
            test_expressions.result_prefix
        )
    }
}
