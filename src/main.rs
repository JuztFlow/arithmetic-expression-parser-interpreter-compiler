mod parser;
use parser::Parser;
use parser::Expression;

fn display(expression: Option<Expression>) {
    match expression {
        Some(expression) => println!("{}", expression.to_string_pretty()),
        None => println!("nothing"),
    }
}

fn test_parser() {
    display(Parser::new("1").parse());

    display(Parser::new("1 + 0 ").parse());

    display(Parser::new("1 + (0) ").parse());

    display(Parser::new("1 + 2 * 0 ").parse());

    display(Parser::new("1 * 2 + 0 ").parse());

    display(Parser::new("(1 + 2) * 0 ").parse());

    display(Parser::new("(1 + 2) * 0 + 2").parse());
}

fn main() {

    test_parser();

    //test_vm();

}

