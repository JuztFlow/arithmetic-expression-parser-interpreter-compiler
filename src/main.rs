/* Author: Florian Eßwein, esfl1011@h-ka.de */

mod parser;
use parser::{Parser, Expression};

mod vm;
use vm::{VM, Instruction};

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {

    test_parser();

    test_vm();
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn display_parser_result(expression: Option<Expression>) {
    match expression {
        Some(expression) => println!("> {:18} = {}", expression.to_string_pretty(), expression.evaluate()),
        None => println!("> nothing"),
    }
}

fn test_parser() {

    println!("\nParser (SYNTAX):");

    display_parser_result(Parser::new("1").parse());

    display_parser_result(Parser::new("1 + 0 ").parse());

    display_parser_result(Parser::new("1 + (0) ").parse());

    display_parser_result(Parser::new("1 + 2 * 0 ").parse());

    display_parser_result(Parser::new("1 * 2 + 0 ").parse());

    display_parser_result(Parser::new("(1 + 2) * 0 ").parse());

    display_parser_result(Parser::new("(1 + 2) * 0 + 2").parse());

    display_parser_result(Parser::new("3 * 2 + 1").parse());

    display_parser_result(Parser::new("(5 + 3) * 2").parse());

    display_parser_result(Parser::new("(5 + 3) * ((2 + 8) + 5)").parse());
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn display_vm_result(result: Option<i64>) {
    match result {
        Some(result) => println!("> VM stack (top): {}", result),
        None => println!("> VM stack (top): empty"),
    }
}

fn test_vm() {

    println!("\nVM (SEMANTIK):");
    
    let instructions = vec![
        Instruction::new_push(1),
        Instruction::new_push(2),
        Instruction::new_push(3),
        Instruction::new_mult(),
        Instruction::new_plus(),
    ];
    display_vm_result(VM::new(instructions).run());

    let instructions = vec![
        Instruction::new_push(2),
        Instruction::new_push(3),
        Instruction::new_push(5),
        Instruction::new_plus(),
        Instruction::new_mult(),
    ];
    display_vm_result(VM::new(instructions).run());

    /*   
     *   Teilaufgabe 3: SEMANTIK: If your solution is correct, the following code should produce the same result as the parser does above in `test_parser()`!
     */

    let instructions = expression_to_instructions(Parser::new("(5 + 3) * ((2 + 8) + 5)").parse());
    display_vm_result(VM::new(instructions).run());
    println!("{:18} ^ 120 would be correct here!", " ");
}

#[allow(dead_code, unused_variables)]
fn expression_to_instructions(expression: Option<Expression>) -> Vec<Instruction> {

    /*   
     *   TODO: Teilaufgabe 3: SEMANTIK: Implement this method
     *
     *   Hints:
     *      - Use `match expression { ... }` to match the expression, and return the corresponding instruction(s) per case
     *      - Again, this function needs to be recursive, because the expression can be arbitrarily nested
     *      - Note that the order of instructions is important here, because the corresponding operations follow after their operands
     *      - You can use the `new_push()`, `new_plus()` and `new_mult()` methods of the `Instruction` struct to create the necessary instructions
     */

    Vec::<Instruction>::new() // <- Replace this with you solution!

    /*   
     *   Possible Solution:
     *
     *   match expression {
     *       Some(expression) => match expression {
     *           Expression::Integer(value) => vec![Instruction::new_push(value)],
     *           Expression::Plus(left, right) => {
     *               let mut instructions = expression_to_instructions(Some(*left));
     *               instructions.append(&mut expression_to_instructions(Some(*right)));
     *               instructions.push(Instruction::new_plus());
     *               instructions
     *           },
     *           Expression::Multiply(left, right) => {
     *               let mut instructions = expression_to_instructions(Some(*left));
     *               instructions.append(&mut expression_to_instructions(Some(*right)));
     *               instructions.push(Instruction::new_mult());
     *               instructions
     *           },
     *       },
     *       None => Vec::<Instruction>::new(),
     *   }
     */
}

