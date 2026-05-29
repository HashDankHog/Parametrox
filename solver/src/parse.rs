use std::collections::HashMap;

pub fn tokenize(expression_raw: &String) -> Vec<String> {
    let mut expression = expression_raw.clone(); // clone might make this more inneficient than my original solution 

    expression.make_ascii_lowercase();
    expression = expression.replace(" ", "");
    //tokinizes the input expression
    let mut tokens: Vec<String> = Vec::new();
    let mut token = String::new();
    // I should probably create an enum for this instead of using magic numbers but there might be a performance hit
    let mut token_type = 0;

    /* tokenizing rules
    each token can be either an operation
    a parantheses
    the letter p followed by an integer
    or an integer
    */
    /* token_type
    0: begining of string
    1: number
    2: variable
    3: operation
    4: prefix op
    5: postfix op
     */

    for character in expression.chars() {
        match token_type {
            0 => {
                // beginning of expression

                token.push(character);
                match character {
                    '0'..='9' => token_type = 1,
                    'p' => token_type = 2,
                    '(' => token_type = 4,
                    _ => panic!("invalid expression"),
                }
            }
            1 => {
                // constant
                match character {
                    '0'..='9' | '.' => token.push(character),
                    'p' => {
                        tokens.push(token);
                        tokens.push(String::from("*"));

                        token = String::from("p");
                        token_type = 2;
                    }
                    '+' | '-' | '/' | '*' => {
                        tokens.push(token);
                        token = String::from(character);
                        token_type = 3;
                    }
                    _ => panic!("invalid expression"),
                }
            }
            2 => match character {
                '0'..='9' => token.push(character),
                'p' => {
                    tokens.push(token);
                    tokens.push(String::from("*"));
                    token = String::from("p");
                }
                '+' | '-' | '/' | '*' => {
                    tokens.push(token);
                    token = String::from(character);
                    token_type = 3;
                }
                _ => panic!("invalid expression"),
            },
            3 => {
                tokens.push(token);

                token = String::from(character);

                match character {
                    ' ' => token_type = 0,
                    '0'..='9' => token_type = 1,
                    'p' => token_type = 2,
                    '(' => token_type = 4,
                    _ => panic!("invalid expression"),
                }
            }
            _ => unreachable!(),
        }
    }
    tokens.push(token); // pushs last token on stack
    tokens
}

//shunting yard algorithm
//autism reference
pub fn parse(tokens: Vec<String>) -> Vec<String> {
    let mut precidence = HashMap::new();
    precidence.insert(String::from("+"), 1);
    precidence.insert(String::from("-"), 1);
    precidence.insert(String::from("*"), 2);
    precidence.insert(String::from("/"), 2);
    let mut output = Vec::new();
    let mut operation_stack = Vec::new();
    operation_stack.push(String::new());
    for token in tokens {
        let value = precidence.get(&token).copied().unwrap_or(0);
        match value {
            0 => output.push(token),
            _ => {
                if value > precidence.get(&operation_stack[0]).copied().unwrap_or(3) {
                    output.push(token);
                } else {
                    operation_stack.push(token);
                }
            }
        }
    }
    operation_stack.reverse();
    for operator in operation_stack {
        output.push(operator);
    }
    output.pop();
    output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_test_constant() {
        let test_expression = String::from("3.1415926");
        let tokens = tokenize(&test_expression);
        assert_eq!(tokens, vec![String::from("3.1415926")])
    }

    #[test]
    fn lexer_test_variable() {
        let test_expression = String::from("P123");
        let tokens = tokenize(&test_expression);
        assert_eq!(tokens, vec![String::from("p123")])
    }

    #[test]
    fn lexer_test_operation() {
        let test_expression = String::from("1.5+3.5/2-100");
        let tokens = tokenize(&test_expression);
        assert_eq!(
            tokens,
            vec![
                String::from("1.5"),
                String::from("+"),
                String::from("3.5"),
                String::from("/"),
                String::from("2"),
                String::from("-"),
                String::from("100")
            ]
        )
    }

    #[test]
    fn lexer_test_prefix() {
        let test_expression = String::from("4p");
        let tokens = tokenize(&test_expression);
        assert_eq!(
            tokens,
            vec![String::from("4"), String::from("*"), String::from("p")]
        )
    }
    #[test]
    fn lexer_test_whitespace() {
        let test_expression = String::from(" P 1 + 3 . 0 *   10   ");
        let tokens = tokenize(&test_expression);
        assert_eq!(
            tokens,
            vec![
                String::from("p1"),
                String::from("+"),
                String::from("3.0"),
                String::from("*"),
                String::from("10")
            ]
        )
    }

    #[test]
    fn parser_test_operation() {
        let tokens = vec![
            String::from("1"),
            String::from("+"),
            String::from("2"),
            String::from("-"),
            String::from("1"),
            String::from("*"),
            String::from("3"),
        ];
        let parsed_expression = parse(tokens);
        assert_eq!(
            parsed_expression,
            vec![
                String::from("1"),
                String::from("2"),
                String::from("1"),
                String::from("3"),
                String::from("*"),
                String::from("-"),
                String::from("+")
            ]
        )
    }
}
