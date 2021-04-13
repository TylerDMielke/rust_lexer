pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
    
    pub fn as_str(&self) -> String {
        format!("Type: {0}, Lexeme: {1}, Line: {2}", self.token_type_as_str(), self.lexeme, self.line)
    }
    
    // This is stupid but I don't know a smarter way
    fn token_type_as_str(&self) -> &str {
        match self.token_type {
            TokenType::RightParen => ")",
            TokenType::LeftParen => "(",
            TokenType::LeftBrace => "[",
            TokenType::RightBrace => "]",
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::Minus => "-",
            TokenType::Plus => "+",
            TokenType::SemiColon => ";",
            TokenType::Slash => "/",
            TokenType::Star => "*",
            TokenType::Bang => "!",
            TokenType::BangEqual => "!=",
            TokenType::Equal => "=",
            TokenType::EqualEqual => "==",
            TokenType::Greater => ">",
            TokenType::GreatEqual => ">=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",
            TokenType::Identifier => "identifier",
            TokenType::String => "string",
            TokenType::Number => "number",
            TokenType::And => "and",
            TokenType::Class => "class",
            TokenType::Else => "else",
            TokenType::False => "false",
            TokenType::Fun => "function",
            TokenType::For => "for",
            TokenType::If => "if",
            TokenType::Nil => "nil",
            TokenType::Or => "or",
            TokenType::Print => "print",
            TokenType::Return => "return",
            TokenType::Super => "super",
            TokenType::This => "this",
            TokenType::True => "true",
            TokenType::Var => "var",
            TokenType::While => "while",
            TokenType::Comment => "comment",
            TokenType::WhiteSpace => "ws",
            TokenType::Unknown => "unknown",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens. 
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreatEqual,
    Less,
    LessEqual,
    Comment,

    // Literals.
    Identifier,
    String,
    Number,

    //Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    
    // White space
    WhiteSpace,
    
    //Unknown token type
    Unknown,
}

#[cfg(test)]
mod test {
    use super::Token;
    use super::TokenType;

    #[test]
    fn test_as_str() {
        let token = Token::new(TokenType::String, String::from("\"string\""), 1);
        let expected_string = "Type: string, Lexeme: \"string\", Line: 1";
        
        assert_eq!(expected_string, token.as_str());
    }
}