use std::str::Chars;

pub struct Cursor <'a>{
    initial_len: usize,
    chars: Chars<'a>,
}

pub const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor{
            initial_len: input.len(),
            chars: input.chars(),
        }
    }
    
    // Get next symbol without consuming it
    pub fn peek(&self) -> char {
        self.chars().nth(0).unwrap_or(EOF_CHAR)
    }
    
    // Grab the next character and consume it
    pub fn advance(&mut self) -> Option<char> {
        Some(self.chars.next()?)
    }
    
    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }
    
}

#[cfg(test)]
mod test {
    use super::Cursor;

    const TEST_STR: &str = "lex me!";

    #[test]
    fn test_peek() {
        let cursor = Cursor::new(TEST_STR);
        
        assert_eq!('l', cursor.peek());
        assert_eq!('l', cursor.peek());
    }
    
    #[test]
    fn test_chars() {
        let cursor = Cursor::new(TEST_STR);
        let chars = cursor.chars();

        assert_eq!(chars.as_str(),cursor.chars.clone().as_str());
    }

    #[test]
    fn test_advance() {
        let mut cursor = Cursor::new(TEST_STR);
        
        assert_eq!('l', cursor.advance().unwrap());
        assert_eq!('e', cursor.peek());
    }
}