pub fn tokenize(expression: &String, ) -> Vec<String> {
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
            0 => { // beginning of expression
                if character != ' ' {
                    token.push(character);
                }
                match character {
                    ' '       => token_type = 0, 
                    '0'..='9' => token_type = 1,
                    'p' | 'P' => token_type = 2,
                    '('       => token_type = 4,
                     _        => panic!("invalid expression"),
                }
            },
            1 => { // constant
                match character {
                    ' '             => token_type = 1,
                    '0'..='9' | '.' => token.push(character),
                    'p' | 'P'       => {
                        tokens.push(token);
                        tokens.push(String::from("*"));

                        token = String::from("p");
                        token_type = 2;
                    },
                    '+' | '-' | '/' | '*' => { // should be operations[i] but okay compilar pop off girly pop give me that error you go girl
                        tokens.push(token);
                        token = String::from(character);
                        token_type = 3;
                    },
                    _                => panic!("invalid expression"), // son im crine why is this unreachable. girly pop you lied
                }
            },
            2 => {
                match character {
                    ' '       => token_type = 2,
                    '0'..='9' => token.push(character),
                    'p' | 'P' => {
                        tokens.push(token);
                        tokens.push(String::from("*"));
                        token = String::from("p");
                    },
                    '+' | '-' | '/' | '*' => { // should be operations[i] but okay compilar pop off girly pop give me that error you go girl
                        tokens.push(token);
                        token = String::from(character);
                        token_type = 3;
                    },
                    _ => panic!("invalid expression"), // son im crine why is this unreachable. girly pop you lied
                }
            }
            3 => {
                tokens.push(token);
                // this if else block is fugly and needs to be fixed
                if character != ' ' {
                    token = String::from(character);
                } else {
                    token = String::new();
                }
                match character {
                    ' '       => token_type = 0,
                    '0'..='9' => token_type = 1,
                    'p' | 'P' => token_type = 2,
                    '('       => token_type = 4,
                     _        => panic!("invalid expression"),
                }
            },
            _ => unreachable!()
        }
        token.make_ascii_lowercase(); //inefficent to make this call every loop 

    }
    tokens.push(token); // pushs last token on stack
    tokens
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
        assert_eq!(tokens, vec![String::from("1.5"), String::from("+"), String::from("3.5"), String::from("/"), String::from("2"), String::from("-"), String::from("100")])
    }

    #[test]
    fn lexer_test_prefix() {
        let test_expression = String::from("4p");
        let tokens = tokenize(&test_expression);
        assert_eq!(tokens, vec![String::from("4"),String::from("*"),String::from("p")])
    }
    #[test]
    fn lexer_test_whitespace(){
        let test_expression = String::from(" P 1 + 3 . 0 *   10   ");
        let tokens = tokenize(&test_expression);
        assert_eq!(tokens, vec![String::from("p1"),String::from("+"), String::from("3.0"), String::from("*"), String::from("10")])
    }
}
