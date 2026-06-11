use std::{cell::RefCell, collections::HashMap, io, rc::Rc};
use solver::{parameter::Parameter, parse};
use std::io::Write;

fn main() {
    let mut parameters: Vec<Rc<RefCell<Parameter>>> = Vec::new();
    let size: usize = 128;
    let operator_string = String::from("+-/c*t^s");
    for _i in 0..size {
        parameters.push(Rc::new(RefCell::new(Parameter::default())));
    }
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
    let mnemonics = vec![(String::from("sin"), String::from("s")),
                                                (String::from("cos"), String::from("c")),
                                                (String::from("tan"), String::from("t")),
                                                (String::from("pi"), String::from("3.14159265359879")),];


    
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
            io::stdin()
        .read_line(&mut input).expect("Failed to read line");
        input = String::from(input.trim());
        if input.trim() == "exit" {
            break
        } else if input.trim() == "clear" {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!();
        } else if input.contains('='){
            let parameter = String::from(input.split('=').collect::<Vec<_>>()[0]);
            let expression_raw = String::from(input.split('=').collect::<Vec<_>>()[1]);

            let expression_tokens = parse::tokenize(&expression_raw, &operator_string, &mnemonics);
            let parsed_expression = parse::parse(expression_tokens, &precidence);
            
            let value = parse::interpret(&parsed_expression, &parameters,0);

            let index: usize = parameter[1..].parse().unwrap_or(0);
            parameters[index].borrow_mut().expression = parsed_expression;
            parameters[index].borrow_mut().value = value;
        } else {
            let expression_tokens = parse::tokenize(&input, &operator_string, &mnemonics);
            let parsed_expression = parse::parse(expression_tokens, &precidence);
            let value = parse::interpret(&parsed_expression, &parameters,0);
            println!("{value}");
        }

    }
}