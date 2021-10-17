//! The Parser module

use crate::lexer::{Lexer, Token, TokenType};
use std::collections::HashSet;

pub struct Parser {
    lexer: Lexer,
    curtoken: Token,
    symbols: HashSet<String>,
    declared_labels: HashSet<String>,
    gotoed_labels: HashSet<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let curtoken = lexer.get_token();

        Parser {
            lexer: lexer,
            curtoken: curtoken,
            symbols: HashSet::new(),
            declared_labels: HashSet::new(),
            gotoed_labels: HashSet::new(),
        }
    }

    fn check_token(&self, kind: TokenType) -> bool {
        self.curtoken.kind == kind
    }

    fn next_token(&mut self) {
        self.curtoken = self.lexer.get_token();
    }

    fn match_token(&mut self, kind: TokenType) {
        if !self.check_token(kind) {
            self.abort(&format!(
                "expected token of kind {:?}, but found token of kind {:?}",
                kind, self.curtoken.kind
            ));
        }
        self.next_token();
    }

    fn abort(&self, message: &str) {
        panic!("Parser error: {}", message);
    }

    /// NL ::= "\n"+
    fn parse_newline(&mut self) {
        println!("NEWLINE");

        self.match_token(TokenType::Newline);
        while self.check_token(TokenType::Newline) {
            self.next_token();
        }
    }

    /// primary ::= number | ident
    fn parse_primary(&mut self) {
        println!("PRIMARY ({:?})", self.curtoken.spelling);

        if self.check_token(TokenType::Number) {
            self.next_token();
        } else if self.check_token(TokenType::Ident) {
            if !self.symbols.contains(&self.curtoken.spelling) {
                self.abort(&format!(
                    "Undeclared variable: {:?}",
                    self.curtoken.spelling
                ));
            }
        }

        if self.check_token(TokenType::Number) || self.check_token(TokenType::Ident) {
            self.next_token();
        }
    }

    /// unary ::= ["+" | "-"] primary
    fn parse_unary(&mut self) {
        println!("UNARY");

        if self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.next_token();
        }
        self.parse_primary();
    }

    /// term ::= unary { ("*" | "/") unary }
    fn parse_term(&mut self) {
        println!("TERM");

        self.parse_unary();

        while self.check_token(TokenType::Asterisk) || self.check_token(TokenType::Slash) {
            self.next_token();
            self.parse_unary();
        }
    }

    /// expression ::= term { ("+" | "-) term }
    fn parse_expression(&mut self) {
        println!("EXPRESSION");

        self.parse_term();

        while self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.next_token();
            self.parse_term();
        }
    }

    fn is_comparison_operator(&self, kind: TokenType) -> bool {
        match kind {
            TokenType::EqEq
            | TokenType::NotEq
            | TokenType::Lt
            | TokenType::Lte
            | TokenType::Gt
            | TokenType::Gte => true,
            _ => false,
        }
    }

    /// comparison ::= expression ( ("==" | "!=" | "<" | "<=" | ">" | ">=") expression)+
    fn parse_comparison(&mut self) {
        println!("COMPARISON");

        self.parse_expression();
        if self.is_comparison_operator(self.curtoken.kind) {
            self.next_token();
            self.parse_expression();
        } else {
            self.abort(&format!(
                "Expected comparison operator, but got {:?}",
                self.curtoken.kind
            ));
        }

        while self.is_comparison_operator(self.curtoken.kind) {
            self.next_token();
            self.parse_expression();
        }
    }

    /// statement ::= "PRINT" (expression | string) NL
    ///             | "IF" comparison "THEN" NL { statement } "ENDIF" NL
    ///             | "WHILE" comparison "REPEAT" NL { statement } "ENDWHILE" NL
    ///             | "LABEL" ident NL
    ///             | "GOTO" ident NL
    ///             | "LET" ident "=" expression NL
    ///             | "INPUT" ident NL
    fn parse_statement(&mut self) {
        match self.curtoken.kind {
            TokenType::Print => {
                println!("STATEMENT-PRINT");
                self.match_token(TokenType::Print);

                if self.check_token(TokenType::String) {
                    self.match_token(TokenType::String);
                } else {
                    self.parse_expression();
                }
            }

            TokenType::If => {
                println!("STATEMENT-IF");
                self.match_token(TokenType::If);
                self.parse_comparison();
                self.match_token(TokenType::Then);
                self.parse_newline();

                while !self.check_token(TokenType::Endif) {
                    self.parse_statement();
                }
                self.match_token(TokenType::Endif);
            }

            TokenType::While => {
                println!("STATEMENT-WHILE");
                self.match_token(TokenType::While);
                self.parse_comparison();
                self.match_token(TokenType::Repeat);
                self.parse_newline();

                while !self.check_token(TokenType::Endwhile) {
                    self.parse_statement();
                }
                self.match_token(TokenType::Endwhile);
            }

            TokenType::Label => {
                println!("STATEMENT-LABEL");
                self.match_token(TokenType::Label);

                if self.declared_labels.contains(&self.curtoken.spelling) {
                    self.abort(&format!("Duplicate label: {:?}", &self.curtoken.spelling));
                }
                self.declared_labels.insert(self.curtoken.spelling.clone());
                self.match_token(TokenType::Ident);
            }

            TokenType::Goto => {
                println!("STATEMENT-GOTO");
                self.match_token(TokenType::Goto);
                self.gotoed_labels.insert(self.curtoken.spelling.clone());
                self.match_token(TokenType::Ident);
            }

            TokenType::Let => {
                println!("STATEMENT-LET");
                self.match_token(TokenType::Let);
                self.symbols.insert(self.curtoken.spelling.clone());

                if !self.symbols.contains(&self.curtoken.spelling) {
                    self.symbols.insert(self.curtoken.spelling.clone());
                }
                self.match_token(TokenType::Ident);
                self.match_token(TokenType::Eq);
                self.parse_expression();
            }

            TokenType::Input => {
                println!("STATEMENT-INPUT");
                self.match_token(TokenType::Input);

                if !self.symbols.contains(&self.curtoken.spelling) {
                    self.symbols.insert(self.curtoken.spelling.clone());
                }
                self.match_token(TokenType::Ident);
            }

            _ => self.abort(&format!("Invalid statement at {:?}", self.curtoken)),
        }

        self.parse_newline();
    }

    /// program ::= { statement }
    fn parse_program(&mut self) {
        println!("PROGRAM");

        while !self.check_token(TokenType::Eof) {
            self.parse_statement();
        }
    }

    pub fn parse(&mut self) {
        while self.check_token(TokenType::Newline) {
            self.next_token();
        }
        self.parse_program();

        for label in &self.gotoed_labels {
            if !self.declared_labels.contains(label) {
                self.abort(&format!("Goto's label is undefined: {:?}", label));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn read_source(infile: &str) -> String {
        use std::fs::File;
        use std::io::{BufReader, Read};

        let mut reader = BufReader::new(File::open(infile).unwrap());
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();
        buffer
    }

    #[test]
    fn test_parse_label_loop() {
        let input = "LABEL loop\nPRINT \"hello, world\"\nGOTO loop";
        let mut parser = Parser::new(Lexer::new(input));
        parser.parse();
    }

    #[test]
    #[should_panic]
    fn test_parse_let() {
        let input = "LET foo = bar * 3 + 2";
        let mut parser = Parser::new(Lexer::new(input));
        parser.parse();
    }

    #[test]
    #[should_panic]
    fn test_parse_let_if() {
        let input = "LET foo = bar * 3 + 2\nIF foo > 0 THEN\nPRINT \"yes!\"\nENDIF\n";
        let mut parser = Parser::new(Lexer::new(input));
        parser.parse();
    }

    #[test]
    #[should_panic]
    fn test_parse_nested_if() {
        let input = "LET foo = bar * 3 + 2\nIF foo > 0 THEN\nIF 10 * 10 < 100 THEN\nPRINT bar\nENDIF\nENDIF";
        let mut parser = Parser::new(Lexer::new(input));
        parser.parse();
    }

    #[test]
    #[should_panic]
    fn test_invalid_variable_and_label() {
        let input = "PRINT index\nGOTO main\n";
        let mut parser = Parser::new(Lexer::new(input));
        parser.parse();
    }

    #[test]
    fn test_parse_average() {
        let mut parser = Parser::new(Lexer::new(&read_source("samples/average.teeny")));
        parser.parse();
    }

    #[test]
    fn test_parse_factorial() {
        let mut parser = Parser::new(Lexer::new(&read_source("samples/factorial.teeny")));
        parser.parse();
    }

    #[test]
    fn test_parse_hello() {
        let mut parser = Parser::new(Lexer::new(&read_source("samples/hello.teeny")));
        parser.parse();
    }

    #[test]
    fn test_parse_statements() {
        let mut parser = Parser::new(Lexer::new(&read_source("samples/statements.teeny")));
        parser.parse();
    }

    #[test]
    fn test_parse_expressions() {
        let mut parser = Parser::new(Lexer::new(&read_source("samples/expression.teeny")));
        parser.parse();
    }

    #[test]
    fn test_parse_fib() {
        let mut parser = Parser::new(Lexer::new(&read_source("samples/fib.teeny")));
        parser.parse();
    }

    #[test]
    fn test_parse_minmax() {
        let mut parser = Parser::new(Lexer::new(&read_source("samples/minmax.teeny")));
        parser.parse();
    }

    #[test]
    fn test_parse_vector() {
        let mut parser = Parser::new(Lexer::new(&read_source("samples/vector.teeny")));
        parser.parse();
    }
}
