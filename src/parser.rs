/* Author: Florian Eßwein, esfl1011@h-ka.de */

mod ast;
use ast::{new_integer_expression, new_plus_expression, new_multiplication_expression};
pub use ast::Expression;

mod tokenizer;
use tokenizer::{Tokenizer, Token};


pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {

    pub fn new(input: &str) -> Parser {
        Parser {
            tokenizer: Tokenizer::new(input),
        }
    }
    
    pub fn parse(self: &mut Parser) -> Option<Expression> {
        return self.parse_e();
    }

    // E  := T E'
    fn parse_e(self: &mut Parser) -> Option<Expression> {
        let t = self.parse_t();
        return match t {
            Some(t) => self.parse_e2(t),
            None => t
        };
    }

    // E' := + T E' |
    fn parse_e2(self: &mut Parser, left: Expression) -> Option<Expression> {
        if self.tokenizer.current_token != Token::PLUS {
            return Some(left);
        }
        self.tokenizer.next_token();
        let right = self.parse_t();
        return match right {
            Some(right) => self.parse_e2(new_plus_expression(left, right)),
            None => right
        };
    }

    // T  := F T'
    fn parse_t(self: &mut Parser) -> Option<Expression> {
        let f = self.parse_f();
        return match f {
            Some(f) => self.parse_t2(f),
            None => f
        };
    }

    // T' := * F T' |
    fn parse_t2(self: &mut Parser, left: Expression) -> Option<Expression> {
        if self.tokenizer.current_token != Token::MULT {
            return Some(left);
        }
        self.tokenizer.next_token();
        let right = self.parse_f();
        return match right {
            Some(right) => self.parse_t2(new_multiplication_expression(left, right)),
            None => right
        }
    }

    // F := N | (E)
    fn parse_f(self: &mut Parser) -> Option<Expression> {
        match self.tokenizer.current_token {
            Token::ZERO => {
                self.tokenizer.next_token();
                return Some(new_integer_expression(0));
            },
            Token::ONE => {
                self.tokenizer.next_token();
                return Some(new_integer_expression(1));
            },
            Token::TWO => {
                self.tokenizer.next_token();
                return Some(new_integer_expression(2));
            },
            Token::THREE => {
                self.tokenizer.next_token();
                return Some(new_integer_expression(3));
            },
            Token::FOUR => {
                self.tokenizer.next_token();
                return Some(new_integer_expression(4));
            },
            Token::FIVE => {
                self.tokenizer.next_token();
                return Some(new_integer_expression(5));
            },
            Token::SIX => {
                self.tokenizer.next_token();
                return Some(new_integer_expression(6));
            },
            Token::SEVEN => {
                self.tokenizer.next_token();
                return Some(new_integer_expression(7));
            },
            Token::EIGHT => {
                self.tokenizer.next_token();
                return Some(new_integer_expression(8));
            },
            Token::NINE => {
                self.tokenizer.next_token();
                return Some(new_integer_expression(9));
            },
            Token::OPEN => {
                self.tokenizer.next_token();
                let expression = self.parse_e();
                match expression {
                    Some(expression) => {
                        if self.tokenizer.current_token != Token::CLOSE {
                            return None;
                        }
                        self.tokenizer.next_token();
                        return Some(expression);
                    },
                    None => {
                        return expression;
                    }
                }
            },
            _ => {
                return None;
            }
        }
    }
}

