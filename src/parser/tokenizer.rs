
#[derive(PartialEq, Clone)]
pub enum Token {
    EOS,
    ZERO,
    ONE,
    TWO,
    OPEN,
    CLOSE,
    PLUS,
    MULTIPLY,
}

#[allow(dead_code)]
pub fn show_token(token: Token) -> String {
    match token {
        Token::EOS      => "EOS".to_string(),
        Token::ZERO     => "ZERO".to_string(),
        Token::ONE      => "ONE".to_string(),
        Token::TWO      => "TWO".to_string(),
        Token::OPEN     => "OPEN".to_string(),
        Token::CLOSE    => "CLOSE".to_string(),
        Token::PLUS     => "PLUS".to_string(),
        Token::MULTIPLY => "MULTIPLY".to_string(),
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
            tokenize: tokenize,
            current_token: current_token,
        }
    }

    pub fn next_token(&mut self) {
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

    fn next(&mut self) -> Token {
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
                    return Token::MULTIPLY;
                }
                _ => {
                    self.position += 1;
                }
            }
        }
    }

    fn scan(&mut self) -> Vec<Token> {
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

    fn show(&mut self) -> String {
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

