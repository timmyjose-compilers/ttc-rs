//! The Parser module

use crate::emitter::Emitter;
use crate::lexer::{Lexer, Token, TokenType};
use std::collections::HashSet;

pub struct Parser<'a> {
    lexer: Lexer,
    emitter: &'a mut Emitter,
    curtoken: Token,
    symbols: HashSet<String>,
    declared_labels: HashSet<String>,
    gotoed_labels: HashSet<String>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer, emitter: &'a mut Emitter) -> Self {
        let curtoken = lexer.get_token();

        Parser {
            lexer: lexer,
            emitter: emitter,
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
        self.match_token(TokenType::Newline);
        while self.check_token(TokenType::Newline) {
            self.next_token();
        }
    }

    /// primary ::= number | ident
    fn parse_primary(&mut self) {
        if self.check_token(TokenType::Number) {
            self.emitter.emit(&self.curtoken.spelling);
            self.next_token();
        } else if self.check_token(TokenType::Ident) {
            if !self.symbols.contains(&self.curtoken.spelling) {
                self.abort(&format!(
                    "Undeclared variable: {:?}",
                    self.curtoken.spelling
                ));
            }

            self.emitter.emit(&self.curtoken.spelling);
            self.next_token();
        } else {
            self.abort(&format!("Unexpected token: {:?}", self.curtoken.spelling));
        }
    }

    /// unary ::= ["+" | "-"] primary
    fn parse_unary(&mut self) {
        if self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.emitter.emit(&self.curtoken.spelling);
            self.next_token();
        }
        self.parse_primary();
    }

    /// term ::= unary { ("*" | "/") unary }
    fn parse_term(&mut self) {
        self.parse_unary();

        while self.check_token(TokenType::Asterisk) || self.check_token(TokenType::Slash) {
            self.emitter.emit(&self.curtoken.spelling);
            self.next_token();
            self.parse_unary();
        }
    }

    /// expression ::= term { ("+" | "-) term }
    fn parse_expression(&mut self) {
        self.parse_term();

        while self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.emitter.emit(&self.curtoken.spelling);
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
        self.parse_expression();
        if self.is_comparison_operator(self.curtoken.kind) {
            self.emitter.emit(&self.curtoken.spelling);
            self.next_token();
            self.parse_expression();
        } else {
            self.abort(&format!(
                "Expected comparison operator, but got {:?}",
                self.curtoken.kind
            ));
        }

        while self.is_comparison_operator(self.curtoken.kind) {
            self.emitter.emit(&self.curtoken.spelling);
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
                self.match_token(TokenType::Print);

                if self.check_token(TokenType::String) {
                    self.emitter
                        .emit_line(&format!("printf(\"{}\\n\");", self.curtoken.spelling));
                    self.match_token(TokenType::String);
                } else {
                    self.emitter
                        .emit(&format!("printf(\"{}\\n\", (float)(", "%.2f"));
                    self.parse_expression();
                    self.emitter.emit_line("));");
                }
            }

            TokenType::If => {
                self.match_token(TokenType::If);
                self.emitter.emit("if (");
                self.parse_comparison();
                self.match_token(TokenType::Then);
                self.parse_newline();
                self.emitter.emit_line(") {");

                while !self.check_token(TokenType::Endif) {
                    self.parse_statement();
                }
                self.match_token(TokenType::Endif);
                self.emitter.emit_line("}");
            }

            TokenType::While => {
                self.match_token(TokenType::While);
                self.emitter.emit("while (");
                self.parse_comparison();
                self.match_token(TokenType::Repeat);
                self.parse_newline();
                self.emitter.emit_line(") {");

                while !self.check_token(TokenType::Endwhile) {
                    self.parse_statement();
                }
                self.match_token(TokenType::Endwhile);
                self.emitter.emit_line("}");
            }

            TokenType::Label => {
                self.match_token(TokenType::Label);

                if self.declared_labels.contains(&self.curtoken.spelling) {
                    self.abort(&format!("Duplicate label: {:?}", &self.curtoken.spelling));
                }
                self.declared_labels.insert(self.curtoken.spelling.clone());
                self.emitter
                    .emit_line(&format!("{}:", self.curtoken.spelling));
                self.match_token(TokenType::Ident);
            }

            TokenType::Goto => {
                self.match_token(TokenType::Goto);
                self.gotoed_labels.insert(self.curtoken.spelling.clone());
                self.emitter
                    .emit_line(&format!("goto {};", self.curtoken.spelling));
                self.match_token(TokenType::Ident);
            }

            TokenType::Let => {
                self.match_token(TokenType::Let);

                if !self.symbols.contains(&self.curtoken.spelling) {
                    self.symbols.insert(self.curtoken.spelling.clone());
                    self.emitter
                        .header_line(&format!("float {};", self.curtoken.spelling));
                }

                self.emitter.emit(&format!("{} = ", self.curtoken.spelling));
                self.match_token(TokenType::Ident);
                self.match_token(TokenType::Eq);
                self.parse_expression();
                self.emitter.emit_line(";");
            }

            TokenType::Input => {
                self.match_token(TokenType::Input);

                if !self.symbols.contains(&self.curtoken.spelling) {
                    self.symbols.insert(self.curtoken.spelling.clone());
                    self.emitter
                        .header_line(&format!("float {};", self.curtoken.spelling));
                }
                self.emitter.emit_line(&format!(
                    "if (0 == scanf(\"{}\", &{})) {{",
                    "%f", self.curtoken.spelling
                ));
                self.emitter
                    .emit_line(&format!("{} = 0;", self.curtoken.spelling));
                self.emitter.emit("scanf(\"%");
                self.emitter.emit_line("*s\");");
                self.emitter.emit_line("}");
                self.match_token(TokenType::Ident);
            }

            _ => self.abort(&format!("Invalid statement at {:?}", self.curtoken)),
        }

        self.parse_newline();
    }

    /// program ::= { statement }
    fn parse_program(&mut self) {
        self.emitter.header_line("#include <stdio.h>");
        self.emitter
            .header_line("int main(int argc, char *argv[]) {");

        while !self.check_token(TokenType::Eof) {
            self.parse_statement();
        }

        self.emitter.emit_line("return 0;");
        self.emitter.emit_line("}");
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
    use crate::emitter::Emitter;
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
        let emitter = Emitter::new("dummy.c");
        let mut parser = Parser::new(Lexer::new(input), &emitter);
        parser.parse();
    }

    #[test]
    #[should_panic]
    fn test_parse_let() {
        let input = "LET foo = bar * 3 + 2";
        let emitter = Emitter::new("dummy.c");
        let mut parser = Parser::new(Lexer::new(input), &emitter);
        parser.parse();
    }

    #[test]
    #[should_panic]
    fn test_parse_let_if() {
        let input = "LET foo = bar * 3 + 2\nIF foo > 0 THEN\nPRINT \"yes!\"\nENDIF\n";
        let emitter = Emitter::new("dummy.c");
        let mut parser = Parser::new(Lexer::new(input), &emitter);
        parser.parse();
    }

    #[test]
    #[should_panic]
    fn test_parse_nested_if() {
        let input = "LET foo = bar * 3 + 2\nIF foo > 0 THEN\nIF 10 * 10 < 100 THEN\nPRINT bar\nENDIF\nENDIF";
        let emitter = Emitter::new("dummy.c");
        let mut parser = Parser::new(Lexer::new(input), &emitter);
        parser.parse();
    }

    #[test]
    #[should_panic]
    fn test_invalid_variable_and_label() {
        let input = "PRINT index\nGOTO main\n";
        let emitter = Emitter::new("dummy.c");
        let mut parser = Parser::new(Lexer::new(input), &emitter);
        parser.parse();
    }

    #[test]
    fn test_parse_average() {
        let emitter = Emitter::new("dummy.c");
        let mut parser = Parser::new(Lexer::new(&read_source("samples/average.teeny")), &emitter);
        parser.parse();
    }

    #[test]
    fn test_parse_factorial() {
        let emitter = Emitter::new("dummy.c");
        let mut parser = Parser::new(
            Lexer::new(&read_source("samples/factorial.teeny")),
            &emitter,
        );
        parser.parse();
    }

    #[test]
    fn test_parse_hello() {
        let emitter = Emitter::new("dummy.c");
        let mut parser = Parser::new(Lexer::new(&read_source("samples/hello.teeny")), &emitter);
        parser.parse();
    }

    #[test]
    fn test_parse_statements() {
        let emitter = Emitter::new("dummy.c");
        let mut parser = Parser::new(
            Lexer::new(&read_source("samples/statements.teeny")),
            &emitter,
        );
        parser.parse();
    }

    #[test]
    fn test_parse_expressions() {
        let emitter = Emitter::new("dummy.c");
        let mut parser = Parser::new(
            Lexer::new(&read_source("samples/expression.teeny")),
            &emitter,
        );
        parser.parse();
    }

    #[test]
    fn test_parse_fib() {
        let emitter = Emitter::new("dummy.c");
        let mut parser = Parser::new(Lexer::new(&read_source("samples/fib.teeny")), &emitter);
        parser.parse();
    }

    #[test]
    fn test_parse_minmax() {
        let emitter = Emitter::new("dummy.c");
        let mut parser = Parser::new(Lexer::new(&read_source("samples/minmax.teeny")), &emitter);
        parser.parse();
    }

    #[test]
    fn test_parse_vector() {
        let emitter = Emitter::new("dummy.c");
        let mut parser = Parser::new(Lexer::new(&read_source("samples/vector.teeny")), &emitter);
        parser.parse();
    }
}
