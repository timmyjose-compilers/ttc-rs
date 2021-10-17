///! The lexer module

pub struct Lexer {
    pub source: String,
    pub curpos: isize,
    pub curchar: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut source = input.to_owned();
        source.push('\n');

        let mut lexer = Lexer {
            source: source,
            curpos: -1,
            curchar: '\u{0000}',
        };

        lexer.next_char();

        lexer
    }

    fn next_char(&mut self) {
        self.curpos += 1;

        if self.curpos as usize >= self.source.len() {
            self.curchar = '\u{0000}';
        } else {
            self.curchar = self.source.chars().nth(self.curpos as usize).unwrap();
        }
    }

    fn peek(&self) -> Option<char> {
        if (self.curpos + 1) as usize >= self.source.len() {
            return Some('\u{0000}');
        }
        self.source.chars().nth((self.curpos + 1) as usize)
    }

    fn abort(&self, message: &str) {
        panic!("Lexer error: {}", message);
    }

    fn skip_whitespace(&mut self) {
        while self.curchar == ' ' || self.curchar == '\t' || self.curchar == '\r' {
            self.next_char();
        }
    }

    fn skip_comment(&mut self) {
        if self.curchar == '#' {
            while self.curchar != '\n' {
                self.next_char();
            }
        }
    }

    pub fn get_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comment();

        let mut token = Token::new(TokenType::Eof, "");

        match self.curchar {
            '\n' => token = Token::new(TokenType::Newline, "\n"),
            '+' => token = Token::new(TokenType::Plus, "+"),
            '-' => token = Token::new(TokenType::Minus, "-"),
            '*' => token = Token::new(TokenType::Asterisk, "*"),
            '/' => token = Token::new(TokenType::Slash, "/"),
            '=' => {
                if self.peek() == Some('=') {
                    self.next_char();
                    token = Token::new(TokenType::EqEq, "==");
                } else {
                    token = Token::new(TokenType::Eq, "=");
                }
            }
            '<' => {
                if self.peek() == Some('=') {
                    self.next_char();
                    token = Token::new(TokenType::Lte, "<=");
                } else {
                    token = Token::new(TokenType::Lt, "<");
                }
            }
            '>' => {
                if self.peek() == Some('=') {
                    self.next_char();
                    token = Token::new(TokenType::Gte, ">=");
                } else {
                    token = Token::new(TokenType::Gt, ">");
                }
            }
            '!' => {
                if self.peek() == Some('=') {
                    self.next_char();
                    token = Token::new(TokenType::NotEq, "!=");
                } else {
                    self.abort("! must be followed by =");
                }
            }

            '"' => {
                self.next_char();
                let startpos = self.curpos as usize;

                while self.curchar != '"' {
                    if self.curchar == '%'
                        || self.curchar == '\r'
                        || self.curchar == '\n'
                        || self.curchar == '\\'
                        || self.curchar == '\t'
                    {
                        self.abort(&format!(
                            "Unsupported character in string: {}",
                            self.curchar
                        ));
                    }
                    self.next_char();
                }

                token = Token::new(
                    TokenType::String,
                    &self.source[startpos..self.curpos as usize],
                );
            }

            c if c.is_digit(10) => {
                let startpos = self.curpos as usize;

                while let Some(c) = self.peek() {
                    if c.is_digit(10) {
                        self.next_char();
                    } else {
                        break;
                    }
                }

                if let Some('.') = self.peek() {
                    self.next_char();

                    if let Some(c) = self.peek() {
                        if !c.is_digit(10) {
                            self.abort(
                                "numbers must have at least one digit after the decimal point",
                            );
                        }
                    }

                    self.next_char();
                    while let Some(c) = self.peek() {
                        if c.is_digit(10) {
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                }

                token = Token::new(
                    TokenType::Number,
                    &self.source[startpos..(self.curpos + 1) as usize],
                );
            }

            c if c.is_ascii_alphabetic() => {
                let startpos = self.curpos as usize;

                while let Some(c) = self.peek() {
                    if c.is_ascii_alphanumeric() {
                        self.next_char();
                    } else {
                        break;
                    }
                }

                token = Token::new(
                    TokenType::Ident,
                    &self.source[startpos..(self.curpos + 1) as usize],
                );
            }

            '\u{0000}' => {}

            _ => self.abort(&format!("Unsupported token: {}", self.curchar)),
        }

        self.next_char();
        token
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub spelling: String,
}

impl Token {
    pub fn new(kind: TokenType, spelling: &str) -> Self {
        Token {
            kind: if kind == TokenType::Ident {
                TokenType::get_token_type_for_ident(spelling)
            } else {
                kind
            },
            spelling: spelling.to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
    Asterisk,
    Endif,
    Endwhile,
    Eof,
    Eq,
    EqEq,
    Goto,
    Gt,
    Gte,
    Ident,
    If,
    Input,
    Label,
    Let,
    Lt,
    Lte,
    Minus,
    Newline,
    NotEq,
    Number,
    Plus,
    Print,
    Repeat,
    Slash,
    String,
    Then,
    While,
}

impl TokenType {
    pub fn get_token_type_for_ident(ident: &str) -> TokenType {
        match ident {
            "ENDIF" => TokenType::Endif,
            "ENDWHILE" => TokenType::Endwhile,
            "GOTO" => TokenType::Goto,
            "IF" => TokenType::If,
            "INPUT" => TokenType::Input,
            "LABEL" => TokenType::Label,
            "LET" => TokenType::Let,
            "REPEAT" => TokenType::Repeat,
            "THEN" => TokenType::Then,
            "WHILE" => TokenType::While,
            "PRINT" => TokenType::Print,
            _ => TokenType::Ident,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::{Lexer, TokenType};

    #[test]
    fn test_tokenize() {
        let input = "LET foo = 123";
        let mut lexer = Lexer::new(input);

        while lexer.peek() != Some('\u{0000}') {
            print!("{} ", lexer.curchar);
            lexer.next_char();
        }
        println!();
    }

    fn read_source(infile: &str) -> String {
        use std::fs::File;
        use std::io::{BufReader, Read};

        let mut reader = BufReader::new(File::open(infile).unwrap());
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();
        buffer
    }

    fn lex(source: &str) {
        let mut lexer = Lexer::new(source);

        let mut token = lexer.get_token();
        while token.kind != TokenType::Eof {
            println!("{:?}", token);
            token = lexer.get_token();
        }
    }

    #[test]
    fn test_lex_operators() {
        lex("+ -\t* /   ");
    }

    #[test]
    fn test_lex_all_operators() {
        lex("+- */ >>= = != <<= ==");
    }

    #[test]
    fn test_lex_comments() {
        lex("+- # This is a comment!\n */");
    }

    #[test]
    fn test_lex_strings() {
        lex("+- \"This is a string\" # This is a comment!\n */");
    }

    #[test]
    fn test_lex_numbers() {
        lex("+-123 9.8654*/");
    }

    #[test]
    fn test_lex_average() {
        lex(&read_source("samples/average.teeny"));
    }

    #[test]
    fn test_lex_keyword() {
        lex("IF+-123 foo*THEN/");
    }

    #[test]
    fn test_lex_factorial() {
        lex(&read_source("samples/factorial.teeny"));
    }

    #[test]
    fn test_lex_hello() {
        lex(&read_source("samples/hello.teeny"));
    }

    #[test]
    fn test_lex_statements() {
        lex(&read_source("samples/statements.teeny"));
    }

    #[test]
    fn test_lex_expressions() {
        lex(&read_source("samples/expression.teeny"));
    }

    #[test]
    fn test_lex_fib() {
        lex(&read_source("samples/fib.teeny"));
    }

    #[test]
    fn test_lex_minmax() {
        lex(&read_source("samples/minmax.teeny"));
    }

    #[test]
    fn test_lex_vector() {
        lex(&read_source("samples/vector.teeny"));
    }
}
