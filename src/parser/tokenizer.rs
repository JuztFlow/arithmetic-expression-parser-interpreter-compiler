/* Author: Florian Eßwein, esfl1011@h-ka.de */

#[derive(PartialEq, Clone)]
pub enum Token {
    EOS,
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    OPEN,
    CLOSE,
    PLUS,
    MULT,
}

#[allow(dead_code)]
pub fn show_token(token: Token) -> String {
    match token {
        Token::EOS      => "EOS".to_string(),
        Token::ZERO     => "ZERO".to_string(),
        Token::ONE      => "ONE".to_string(),
        Token::TWO      => "TWO".to_string(),
        Token::THREE    => "THREE".to_string(),
        Token::FOUR     => "FOUR".to_string(),
        Token::FIVE     => "FIVE".to_string(),
        Token::SIX      => "SIX".to_string(),
        Token::SEVEN    => "SEVEN".to_string(),
        Token::EIGHT    => "EIGHT".to_string(),
        Token::NINE     => "NINE".to_string(),
        Token::OPEN     => "OPEN".to_string(),
        Token::CLOSE    => "CLOSE".to_string(),
        Token::PLUS     => "PLUS".to_string(),
        Token::MULT     => "MULT".to_string(),
    }
}

pub struct Tokenizer {
    tokenize: Tokenize,
    pub current_token: Token,
}

impl Tokenizer {
    
    pub fn new(input: &str) -> Tokenizer {
        let mut tokenize = Tokenize::new(input);
        let current_token = tokenize.next();
        Tokenizer {
            tokenize,
            current_token,
        }
    }

    pub fn next_token(self: &mut Tokenizer) {
        self.current_token = self.tokenize.next();
    }

    #[allow(dead_code)]
    pub fn show(&mut self) -> String {
        self.tokenize.show()
    }
}

struct Tokenize {
    input: String,
    position: usize,
}

impl Tokenize {

    fn new(input: &str) -> Tokenize {
        Tokenize {
            input: input.to_string(),
            position: 0,
        }
    }

    fn next(self: &mut Tokenize) -> Token {
        loop {
            if self.input.len() <= self.position {
                return Token::EOS;
            }
            let current_character = self.input.chars().nth(self.position).unwrap();
            match current_character {
                '0' => {
                    self.position += 1;
                    return Token::ZERO;
                }
                '1' => {
                    self.position += 1;
                    return Token::ONE;
                }
                '2' => {
                    self.position += 1;
                    return Token::TWO;
                }
                '3' => {
                    self.position += 1;
                    return Token::THREE;
                }
                '4' => {
                    self.position += 1;
                    return Token::FOUR;
                }
                '5' => {
                    self.position += 1;
                    return Token::FIVE;
                }
                '6' => {
                    self.position += 1;
                    return Token::SIX;
                }
                '7' => {
                    self.position += 1;
                    return Token::SEVEN;
                }
                '8' => {
                    self.position += 1;
                    return Token::EIGHT;
                }
                '9' => {
                    self.position += 1;
                    return Token::NINE;
                }  
                '(' => {
                    self.position += 1;
                    return Token::OPEN;
                }
                ')' => {
                    self.position += 1;
                    return Token::CLOSE;
                }
                '+' => {
                    self.position += 1;
                    return Token::PLUS;
                }
                '*' => {
                    self.position += 1;
                    return Token::MULT;
                }
                _ => {
                    self.position += 1;
                }
            }
        }
    }

    fn scan(self: &mut Tokenize) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next();
            tokens.push(token.clone());
            if token == Token::EOS {
                break;
            }
        }
        tokens
    }

    fn show(self: &mut Tokenize) -> String {
        let tokens = self.scan();
        let mut string = String::new();
        for i in 0..tokens.len() {
            string.push_str(&show_token(tokens[i].clone()));
            if i < tokens.len() - 1 {
                string.push_str(";");
            }
        }
        string
    }
}

