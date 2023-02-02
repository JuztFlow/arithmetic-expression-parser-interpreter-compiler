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
    
    pub fn parse(&mut self) -> Option<Expression> {
        self.parse_e()
    }

    // E  ::= T E'
    fn parse_e(&mut self) -> Option<Expression> {
        let t = self.parse_t();
        match t {
            Some(t) => self.parse_e2(t),
            None => t
        }
    }

    // E' ::= + T E' |
    fn parse_e2(&mut self, left: Expression) -> Option<Expression> {
        if self.tokenizer.current_token == Token::PLUS {
            self.tokenizer.next_token();
            let right = self.parse_t();
            return match right {
                Some(right) => self.parse_e2(new_plus_expression(left, right)),
                None => right
            };
        };
        Some(left)
    }

    // T  ::= F T'
    fn parse_t(&mut self) -> Option<Expression> {
        let f = self.parse_f();
        match f {
            Some(f) => self.parse_t2(f),
            None => f
        }
    }

    // T' ::= * F T' |
    fn parse_t2(&mut self, left: Expression) -> Option<Expression> {
        if self.tokenizer.current_token == Token::MULTIPLY {
            self.tokenizer.next_token();
            let right = self.parse_f();
            return match right {
                Some(right) => self.parse_t2(new_multiplication_expression(left, right)),
                None => right
            }
        }
        Some(left)
    }

    // F ::= N | (E)
    fn parse_f(&mut self) -> Option<Expression> {
        match self.tokenizer.current_token {
            Token::ZERO => {
                self.tokenizer.next_token();
                Some(new_integer_expression(0))
            },
            Token::ONE => {
                self.tokenizer.next_token();
                Some(new_integer_expression(1))
            },
            Token::TWO => {
                self.tokenizer.next_token();
                Some(new_integer_expression(2))
            },
            Token::OPEN => {
                self.tokenizer.next_token();
                let expression = self.parse_e();
                match expression {
                    Some(expression) => {
                        if self.tokenizer.current_token == Token::CLOSE {
                            self.tokenizer.next_token();
                            Some(expression)
                        } else {
                            None
                        }
                    },
                    None => expression
                }
            },
            _ => None
        }
    }
}
        