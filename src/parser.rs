use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Literal(Token),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Identifier(String),
    Call(String, Vec<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Assignment(String, Expr),
    Block(Vec<Stmt>),
    Match(Expr, Vec<(Expr, Stmt)>),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement());
        }
        statements
    }

    fn statement(&mut self) -> Stmt {
        if self.match_token(&[Token::Identifier("stdout".into())]) {
            self.stdout_statement()
        } else if self.match_token(&[Token::Identifier(_)]) {
            self.assignment_or_call()
        } else {
            Stmt::Expression(self.expression())
        }
    }

    fn stdout_statement(&mut self) -> Stmt {
        self.consume(Token::Assign, "Expect '=' after 'stdout'.");
        let exprs = self.expression_list();
        self.consume(Token::Semicolon, "Expect ';' after 'stdout' statement.");
        Stmt::Print(Expr::Call("stdout".into(), exprs))
    }

    fn assignment_or_call(&mut self) -> Stmt {
        let identifier = if let Token::Identifier(name) = self.previous().clone() {
            name
        } else {
            panic!("Expected identifier.");
        };

        if self.match_token(&[Token::Assign]) {
            let value = self.expression();
            self.consume(Token::Semicolon, "Expect ';' after assignment.");
            Stmt::Assignment(identifier, value)
        } else if self.match_token(&[Token::LeftParen]) {
            let args = self.expression_list();
            self.consume(Token::RightParen, "Expect ')' after function arguments.");
            Stmt::Expression(Expr::Call(identifier, args))
        } else {
            panic!("Expected '=' or '('.")
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(&[Token::And, Token::Or, Token::Xor]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token(&[Token::Match]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token(&[Token::Plus, Token::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(&[Token::Star, Token::Slash, Token::Percent]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(&[Token::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(operator, Box::new(right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(&[Token::True, Token::False]) {
            return Expr::Literal(self.previous().clone());
        }

        if let Some(literal) = self.match_literal() {
            return Expr::Literal(literal);
        }

        if self.match_token(&[Token::Identifier("".into())]) {
            return Expr::Identifier(self.previous().to_string());
        }

        if self.match_token(&[Token::LeftParen]) {
            let expr = self.expression();
            self.consume(Token::RightParen, "Expect ')' after expression.");
            return expr;
        }

        panic!("Expected expression.")
    }

    fn match_literal(&mut self) -> Option<Token> {
        if let Token::Number(_) | Token::String(_) = self.peek().clone() {
            self.advance();
            Some(self.previous().clone())
        } else {
            None
        }
    }

    fn expression_list(&mut self) -> Vec<Expr> {
        let mut exprs = Vec::new();
        if !self.check(Token::RightParen) {
            exprs.push(self.expression());
            while self.match_token(&[Token::Comma]) {
                exprs.push(self.expression());
            }
        }
        exprs
    }

    fn match_token(&mut self, types: &[Token]) -> bool {
        for token in types {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.tokens[self.current] == token
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token: Token, message: &str) {
        if self.check(&token) {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.peek() == &Token::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
}
