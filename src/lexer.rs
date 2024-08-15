#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Each,
    Match,
    True,
    False,
    And,
    Or,
    Xor,
    Stdout,
    Join,

    // Symbols and Operators
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    Assign,         // =
    Arrow,          // =>
    ReverseArrow,   // <=
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    Comma,          // ,
    Semicolon,      // ;
    LeftParen,      // (
    RightParen,     // )

    // Literals
    Identifier(String),
    Number(i64),
    String(String),

    // End of input
    EOF,
}

pub struct Lexer {
    source: String,
    start: usize,
    current: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            start: 0,
            current: 0,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            let c = self.advance();
            match c {
                '+' => self.add_token(Token::Plus),
                '-' => self.add_token(Token::Minus),
                '*' => self.add_token(Token::Star),
                '/' => self.add_token(Token::Slash),
                '%' => self.add_token(Token::Percent),
                '=' => if self.match_char('>') {
                    self.add_token(Token::Arrow);
                } else {
                    self.add_token(Token::Assign);
                },
                '<' => if self.match_char('=') {
                    self.add_token(Token::ReverseArrow);
                },
                '{' => self.add_token(Token::LeftBrace),
                '}' => self.add_token(Token::RightBrace),
                '[' => self.add_token(Token::LeftBracket),
                ']' => self.add_token(Token::RightBracket),
                ',' => self.add_token(Token::Comma),
                ';' => self.add_token(Token::Semicolon),
                '(' => self.add_token(Token::LeftParen),
                ')' => self.add_token(Token::RightParen),
                '"' => self.string(),
                '0'..='9' => self.number(),
                'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
                _ => {}
            }
        }
        self.tokens.push(Token::EOF);
        self.tokens.clone()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }
        self.advance(); // Consume the closing quote
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(Token::String(value));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        let value = self.source[self.start..self.current].parse::<i64>().unwrap();
        self.add_token(Token::Number(value));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token = match text {
            "each" => Token::Each,
            "match" => Token::Match,
            "true" => Token::True,
            "false" => Token::False,
            "and" => Token::And,
            "or" => Token::Or,
            "xor" => Token::Xor,
            "stdout" => Token::Stdout,
            "join" => Token::Join,
            _ => Token::Identifier(text.to_string()),
        };
        self.add_token(token);
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }
}
