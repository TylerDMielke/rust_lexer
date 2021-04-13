use super::cursor::{Cursor, EOF_CHAR};
use super::token::{Token, TokenType};

struct Scanner <'a>{
    cursor: Cursor<'a>,
    tokens: Vec<Token>,
}

impl <'a> Scanner <'a> {
    pub fn new(scan_str: &str) -> Scanner {
        Scanner {
            cursor: Cursor::new(scan_str),
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) {
        while self.cursor.peek() != EOF_CHAR {

            let next_char: char = match self.cursor.advance() {
                Some(char) => char,
                None => panic!("Did not find char but started while loop again")
            };

            let token: Token = self.build_token(next_char);

            self.tokens.push(token);
        }
    }

    fn build_token(&mut self, next_char: char) -> Token {
        let mut lexeme: String = next_char.to_string();
        let mut line: usize = 1;
        let token_type = match next_char {
            ')' => TokenType::RightParen,
            '(' => TokenType::LeftParen,
            '[' => TokenType::LeftBrace,
            ']' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::SemiColon,
            '*' => TokenType::Star,
            '/' => {
                if self.cursor.peek() == '/' {
                    self.cursor.advance();
                    while self.cursor.peek() != '\n' && self.cursor.peek() != EOF_CHAR {
                        self.cursor.advance();
                    }
                    TokenType::Comment
                }
                else {
                    TokenType::Slash
                }
            },
            '!' => {
                match self.cursor.peek() {
                    '=' => {
                        self.cursor.advance();
                        TokenType::BangEqual
                    },
                    _ => TokenType::Bang,
                }
            },
            '=' => {
                match self.cursor.peek() {
                    '=' => {
                        self.cursor.advance();
                        TokenType::BangEqual
                    },
                    _ => TokenType::Equal,
                }
            },
            '>' => {
                match self.cursor.peek() {
                    '=' => {
                        self.cursor.advance();
                        TokenType::GreatEqual
                    },
                    _ => TokenType::Greater,
                }
            },
            '<' => {
                match self.cursor.peek() {
                    '=' => {
                        self.cursor.advance();
                        TokenType::LessEqual
                    },
                    _ => TokenType::Less,
                }
            },
            '\"' => {
                while self.cursor.peek() != '\"' {
                    if self.cursor.peek() == EOF_CHAR {
                        // TODO: Handle this better. 
                        panic!("Unterminated string")
                    }
                    match self.cursor.advance() {
                        Some(char) => lexeme.push(char),
                        None => panic!("Could not find next character while trying to build string"),
                    };
                }
                lexeme.push(self.cursor.advance().unwrap());
                TokenType::String
            },
            ' ' => TokenType::WhiteSpace,
            '\t' => TokenType::WhiteSpace,
            '\n' => {
                line += 1;
                TokenType::WhiteSpace
            },

            // TODO: Figure out how to implement below
            // "v" => TokenType::Var,
            // "number" => TokenType::Number,
            // "and" => TokenType::And,
            // "class" => TokenType::Class,
            // "else" => TokenType::Else,
            // "false" => TokenType::False,
            // "function" => TokenType::Fun,
            // "for" => TokenType::For,
            // "if" => TokenType::If,
            // "nil" => TokenType::Nil,
            // "or" => TokenType::Or,
            // "print" => TokenType::Print,
            // "return" => TokenType::Return,
            // "super" => TokenType::Super,
            // "this" => TokenType::This,
            // "true" => TokenType::True,
            // "while" => TokenType::While,
            // "identifier" => TokenType::Identifier,
            _ => TokenType::Unknown, 
        };
        
        Token::new(token_type, lexeme, line)
    }
}

#[cfg(test)]
mod test {
    use super::Scanner;
    use super::super::token::{Token, TokenType};

    #[test]
    fn test_scan_tokens() {
        let lex_string = "var lex = print(\"test\");";
        let mut scanner = Scanner::new(lex_string);
        let expected_vec = build_expected_token_vec();

        scanner.scan_tokens();

        assert_vec_equal(expected_vec, scanner.tokens);
    }
    
    fn assert_vec_equal(expected_vec: Vec<Token>, actual_vec: Vec<Token>) {
        for i in 0..actual_vec.len() {
            assert_eq!(expected_vec[i].as_str(), actual_vec[i].as_str());
        }
    }
    
    fn build_expected_token_vec() -> Vec<Token>{
        let mut expected_vec: Vec<Token> = Vec::new();
        expected_vec.push(Token::new(TokenType::Unknown, String::from("v"), 1));
        expected_vec.push(Token::new(TokenType::Unknown, String::from("a"), 1));
        expected_vec.push(Token::new(TokenType::Unknown, String::from("r"), 1));
        expected_vec.push(Token::new(TokenType::WhiteSpace, String::from(" "), 1));
        expected_vec.push(Token::new(TokenType::Unknown, String::from("l"), 1));
        expected_vec.push(Token::new(TokenType::Unknown, String::from("e"), 1));
        expected_vec.push(Token::new(TokenType::Unknown, String::from("x"), 1));
        expected_vec.push(Token::new(TokenType::WhiteSpace, String::from(" "), 1));
        expected_vec.push(Token::new(TokenType::Equal, String::from("="), 1));
        expected_vec.push(Token::new(TokenType::WhiteSpace, String::from(" "), 1));
        expected_vec.push(Token::new(TokenType::Unknown, String::from("p"), 1));
        expected_vec.push(Token::new(TokenType::Unknown, String::from("r"), 1));
        expected_vec.push(Token::new(TokenType::Unknown, String::from("i"), 1));
        expected_vec.push(Token::new(TokenType::Unknown, String::from("n"), 1));
        expected_vec.push(Token::new(TokenType::Unknown, String::from("t"), 1));
        expected_vec.push(Token::new(TokenType::LeftParen, String::from("("), 1));
        expected_vec.push(Token::new(TokenType::String, String::from("\"test\""), 1));
        expected_vec.push(Token::new(TokenType::RightParen, String::from(")"), 1));
        expected_vec.push(Token::new(TokenType::SemiColon, String::from(";"), 1));
        
        expected_vec
    }
}