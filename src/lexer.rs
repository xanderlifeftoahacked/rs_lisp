#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Float(f64),
    Integer(i64),
    Symbol(String),
    StringLiteral(String),
    Comment(String),
}

#[derive(Debug)]
pub enum LexerError {
    UnmatchedParen,
    UnexpectedChar(char),
    UnexpectedEof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    paren_count: isize,
}

static SYMB_CHARS: &str = "'=+!-*/><";

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            paren_count: 0,
        }
    }

    pub fn get_position(&self) -> usize {
        self.position
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Ok(None);
        }

        let current_char = self.input[self.position];

        match current_char {
            '(' => {
                self.position += 1;
                self.paren_count += 1;
                Ok(Some(Token::LParen))
            }
            ')' => {
                self.position += 1;
                self.paren_count -= 1;
                if self.paren_count < 0 {
                    return Err(LexerError::UnmatchedParen);
                }
                Ok(Some(Token::RParen))
            }
            '"' => self.read_string(),
            ';' => self.read_comment(),
            c if c.is_digit(10) || c == '-' => self.read_number(),
            c if is_symbol_start(c) => self.read_symbol(),
            _ => Err(LexerError::UnexpectedChar(current_char)),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn read_number(&mut self) -> Result<Option<Token>, LexerError> {
        let start_pos = self.position;
        let mut has_dot = false;

        while self.position < self.input.len() {
            let current_char = self.input[self.position];
            if current_char == '.' && !has_dot {
                has_dot = true;
            } else if !current_char.is_digit(10) && current_char != '.' && current_char != '-' {
                break;
            }
            self.position += 1;
        }

        let number_str: String = self.input[start_pos..self.position].iter().collect();
        if has_dot {
            match number_str.parse::<f64>() {
                Ok(num) => Ok(Some(Token::Float(num))),
                Err(_) => Err(LexerError::UnexpectedChar(self.input[self.position])),
            }
        } else {
            match number_str.parse::<i64>() {
                Ok(num) => Ok(Some(Token::Integer(num))),
                Err(_) => Err(LexerError::UnexpectedChar(self.input[self.position])),
            }
        }
    }

    fn read_string(&mut self) -> Result<Option<Token>, LexerError> {
        self.position += 1;
        let start_pos = self.position;

        while self.position < self.input.len() {
            match self.input[self.position] {
                '"' => {
                    let string_content: String =
                        self.input[start_pos..self.position].iter().collect();
                    self.position += 1;
                    return Ok(Some(Token::StringLiteral(string_content)));
                }
                _ => self.position += 1,
            }
        }

        Err(LexerError::UnexpectedEof)
    }

    fn read_comment(&mut self) -> Result<Option<Token>, LexerError> {
        self.position += 1;
        let start_pos = self.position;

        while self.position < self.input.len() && self.input[self.position] != ';' {
            self.position += 1;
        }

        let comment_content: String = self.input[start_pos..self.position].iter().collect();
        Ok(Some(Token::Comment(comment_content)))
    }

    fn read_symbol(&mut self) -> Result<Option<Token>, LexerError> {
        let start_pos = self.position;

        while self.position < self.input.len() && is_symbol_part(self.input[self.position]) {
            self.position += 1;
        }

        let symbol: String = self.input[start_pos..self.position].iter().collect();
        Ok(Some(Token::Symbol(symbol)))
    }
}

fn is_symbol_start(c: char) -> bool {
    c.is_alphabetic() || SYMB_CHARS.contains(c)
}

fn is_symbol_part(c: char) -> bool {
    c.is_alphanumeric() || SYMB_CHARS.contains(c)
}
