use std::{iter::Peekable, sync::Arc};

use bnum::BUint;
use feo_error::LexErrorKind;
use feo_types::DelimKind;

mod token;
pub use self::token::{Token, TokenStream, TokenTree};

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    peekable_chars: Peekable<std::str::Chars<'a>>,
    // errors: Vec<String>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            peekable_chars: input.chars().peekable(),
            // errors: Vec::new(),
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.peekable_chars.next();
    }

    fn current_char(&self) -> Option<char> {
        self.peekable_chars.peek().cloned()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    // fn log_error(&mut self, message: &str) {
    //     let error_message = format!("Error at position {}: {}", self.pos, message);
    //     self.errors.push(error_message);
    // }

    fn tokenize(&mut self) -> Result<TokenStream<TokenTree>, LexErrorKind> {
        let src = Arc::into_inner(Arc::clone(&self.input)).ok_or(LexErrorKind::SourceFileEmpty)?;

        let char_reader = &mut Lexer {
            src: Arc::clone(&self.input),
            pos: self.pos,
        }
        .peekable();

        let mut tokens: Vec<Option<Token>> = Vec::new();
        let mut token_trees: Vec<Option<TokenTree>> = Vec::new();

        let mut in_block_comment = false; // generic inline / multiline comment
        let mut is_negative_number = false;
        let mut is_hexadecimal_int = false;

        let mut num_open_delimiters: usize = 0;
        let mut file_start_offset: usize = 0;

        while let Some(c) = char_reader.next() {
            let start_index = self.pos;

            match c {
                _ if c.is_whitespace() => {
                    // move the reader to the first char if there is whitespace at the start
                    if start_index - file_start_offset == 0 {
                        file_start_offset += 1;
                    }
                    continue;
                }
                _ if c == '*' && char_reader.peek() == Some(&'/') => {
                    if !in_block_comment {
                        return Err(LexErrorKind::UnopenedBlockComment);
                    } else {
                        char_reader.next(); // skip '*'
                        char_reader.next(); // skip '/'
                        in_block_comment = false;
                    }
                    continue;
                }
                _ if c == '/' => {
                    match char_reader.next() {
                        Some('/') => {
                            char_reader.next();
                            // skip second '/'
                            if let Some('/') = char_reader.peek() {
                                char_reader.next(); // skip third '/'
                                                    //   parse doc comment
                            } else {
                                //  parse newline / trailing comment
                                continue;
                            }
                            continue;
                        }
                        Some('*') => {
                            char_reader.next(); // skip '*'
                            in_block_comment = true;
                            // parse inline / multiline comment
                            char_reader.next(); // skip closing '*'
                            char_reader.next(); // skip closing '/'
                            in_block_comment = false;
                            continue;
                        }
                        Some(_) | None => (),
                    }
                }

                _ if unicode_ident::is_xid_start(c) || c == '_' => {
                    // parse keywords and identifiers
                }

                '(' | '[' | '{' => {
                    num_open_delimiters += 1;
                    // parse opening delimiter
                }

                ')' | ']' | '}' => {
                    // parse closing delimiter
                    num_open_delimiters -= 1;
                }

                '"' => {
                    char_reader.next(); // skip opening double quote
                                        // parse string literal
                    char_reader.next(); // skip closing double quote
                }
                '\'' => {
                    char_reader.next(); // skip opening single quote
                                        // parse char literal
                    char_reader.next(); // skip opening single quote
                }

                // handle negative numbers; do we allow for example "-.3" ?
                // does `is_digit()` include floats?
                _ if c == '-' && char_reader.peek().is_some_and(|c| c.is_digit(10 | 16)) => {
                    is_negative_number = true;
                    char_reader.next();
                }

                // account for hexadecimal prefix
                _ if c == '0' && char_reader.peek() == Some(&'x') => {
                    char_reader.next(); // skip '0'
                    char_reader.next(); // skip 'x'
                    is_hexadecimal_int = true;
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    // parse digits
                    // continue;
                }

                _ if c.is_ascii_punctuation() => {
                    // parse punctuation
                    // continue;
                }
                _ => return Err(LexErrorKind::InvalidChar),
            }
        }

        if num_open_delimiters != 0 {
            return Err(LexErrorKind::UnclosedDelimiters);
        }

        let stream: TokenStream<TokenTree> = TokenStream::build(src, token_trees, 0, self.pos)?;
        Ok(stream)
    }

    ///////////////////////////////////////////////////////////////////////////

    // CHAT-GPT FUNCTIONS

    ///////////////////////////////////////////////////////////////////////////

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.skip_whitespace();
                tokens.push(Token::Whitespace);
            } else if c.is_digit(10) || (c == '-' && self.peek_next().is_digit(10)) {
                tokens.push(self.parse_number());
            } else if c.is_alphabetic() || c == '_' {
                tokens.push(self.parse_identifier_or_keyword());
            } else if c == '"' {
                tokens.push(self.parse_string());
            } else if c == '\'' {
                tokens.push(self.parse_char());
            } else if c == '/' && self.peek_next() == Some('/') {
                tokens.push(self.parse_comment());
            } else if c == '/' && self.peek_next() == Some('*') {
                tokens.push(self.parse_block_comment());
            } else {
                if is_punctuation(c) {
                    self.advance();
                    tokens.push(Token::Punctuation(c));
                } else {
                    match c {
                        '(' | ')' | '[' | ']' | '{' | '}' => {
                            tokens.push(self.tokenize_delimiter());
                        }
                        _ => {
                            self.log_error(&format!("Unexpected character: {}", c));
                            self.advance();
                        }
                    }
                }
            }
        }

        tokens
    }

    fn tokenize_delimiter(&mut self) -> Token {
        let delimiter_type = match self.current_char() {
            Some('(') | Some(')') => DelimKind::Paren,
            Some('[') | Some(']') => DelimKind::Bracket,
            Some('{') | Some('}') => DelimKind::Brace,
            _ => unreachable!(), // Should not be called for non-delimiter characters
        };

        self.advance(); // Consume the delimiter character
        Token::Delimiter(delimiter_type)
    }

    fn tokenize_token_tree(&mut self, delimiter_type: DelimKind) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut nesting_level = 1;

        while let Some(c) = self.current_char() {
            match c {
                '(' | '[' | '{' => {
                    nesting_level += 1;
                    self.advance();
                }
                ')' | ']' | '}' => {
                    nesting_level -= 1;
                    if nesting_level == 0 {
                        self.advance();
                        break;
                    } else {
                        self.advance();
                    }
                }
                _ => {
                    tokens.push(self.tokenize_token());
                }
            }
        }

        if nesting_level > 0 {
            self.log_error("Unexpected end of input within delimiter");
        }

        tokens
    }

    fn tokenize_token(&mut self) -> Token {
        match self.current_char() {
            Some('(') | Some('[') | Some('{') => {
                let delimiter_type = match self.current_char() {
                    Some('(') => DelimKind::Parenthesis,
                    Some(')') => DelimKind::Parenthesis,
                    Some('[') => DelimKind::SquareBracket,
                    Some(']') => DelimKind::SquareBracket,
                    Some('{') => DelimKind::CurlyBrace,
                    Some('}') => DelimKind::CurlyBrace,
                    _ => unreachable!(), // Should not be called for non-delimiter characters
                };

                self.advance();
                Token::Delimiter(delimiter_type)
            }
            _ => {
                // Continue with existing tokenization logic
                // ...

                // Placeholder return to compile the code
                Token::Error("Tokenization not implemented yet".to_string())
            }
        }
    }

    fn parse_string(&mut self) -> Token {
        self.advance(); // Consume the opening double quote
        let mut string_content = String::new();

        while let Some(c) = self.current_char() {
            match c {
                '\\' => {
                    // Handle escape sequences
                    if let Some(escaped_char) = self.parse_escape_sequence() {
                        string_content.push(escaped_char);
                    } else {
                        // Invalid escape sequence
                        self.log_error("Invalid escape sequence in string literal");
                    }
                }
                '"' => {
                    // Consume the closing double quote
                    self.advance();
                    return Token::Str(string_content);
                }
                _ => {
                    string_content.push(c);
                    self.advance();
                }
            }
        }

        // If we reach here, there's an unterminated string literal
        self.log_error("Unterminated string literal");
        Token::Error("Unterminated string literal".to_string())
    }

    fn parse_escape_sequence(&mut self) -> Option<char> {
        self.advance(); // Consume the backslash

        if let Some(escaped_char) = self.current_char() {
            self.advance(); // Consume the escaped character
            match escaped_char {
                'n' => Some('\n'),
                'r' => Some('\r'),
                't' => Some('\t'),
                '\\' => Some('\\'),
                '"' => Some('"'),
                '\'' => Some('\''),
                _ => {
                    // Invalid escape sequence
                    self.log_error("Invalid escape sequence in string literal");
                    None
                }
            }
        } else {
            // Escape sequence is expected, but the input has ended
            self.log_error("Unexpected end of input in escape sequence");
            None
        }
    }

    fn parse_char(&mut self) -> Token {
        self.advance(); // Consume the opening single quote

        if let Some(c) = self.current_char() {
            match c {
                '\\' => {
                    // Handle escape sequences
                    if let Some(escaped_char) = self.parse_escape_sequence() {
                        // Check for the closing single quote
                        if self.current_char() == Some('\'') {
                            self.advance(); // Consume the closing single quote
                            return Token::Char(escaped_char);
                        } else {
                            // Invalid character literal
                            self.log_error("Invalid character literal");
                            return Token::Error("Invalid character literal".to_string());
                        }
                    } else {
                        // Invalid escape sequence
                        self.log_error("Invalid escape sequence in character literal");
                        return Token::Error(
                            "Invalid escape sequence in character literal".to_string(),
                        );
                    }
                }
                '\'' => {
                    // Empty character literal is invalid
                    self.log_error("Empty character literal");
                    Token::Error("Empty character literal".to_string())
                }
                _ => {
                    // Regular character
                    self.advance(); // Consume the character
                    if self.current_char() == Some('\'') {
                        self.advance(); // Consume the closing single quote
                        Token::Char(c)
                    } else {
                        // Invalid character literal
                        self.log_error("Invalid character literal");
                        Token::Error("Invalid character literal".to_string())
                    }
                }
            }
        } else {
            // Unexpected end of input
            self.log_error("Unexpected end of input in character literal");
            Token::Error("Unexpected end of input in character literal".to_string())
        }
    }

    fn parse_line_comment(&mut self) -> Token {
        self.advance(); // Consume the first '/'
        self.advance(); // Consume the second '/'

        let mut comment_content = String::new();

        while let Some(c) = self.current_char() {
            if c == '\n' {
                // End of line, finish the comment
                break;
            } else {
                comment_content.push(c);
                self.advance();
            }
        }

        Token::LineComment(comment_content)
    }

    fn parse_doc_comment(&mut self) -> Token {
        self.advance(); // Skip '/'
        self.advance(); // Skip '!'
        let start_pos = self.pos;

        while let Some(c) = self.current_char() {
            if c == '*' && self.peek_next() == Some('/') {
                self.advance();
                self.advance();
                let end_pos = self.pos;
                let code = Arc::new(self.input[start_pos..end_pos].trim().to_string());
                return Token::DocComment(code);
            } else {
                self.advance();
            }
        }

        self.log_error("Unterminated doc comment");
        Token::Error("Unterminated doc comment".to_string())
    }

    fn parse_block_comment(&mut self) -> Token {
        self.advance(); // Consume the first '/'
        self.advance(); // Consume the '!' character

        let mut comment_content = String::new();

        while let Some(c) = self.current_char() {
            if c == '*' && self.peek_next() == Some('/') {
                // Consume the '*' and '/'
                self.advance();
                self.advance();
                return Token::BlockComment(comment_content);
            } else {
                comment_content.push(c);
                self.advance();
            }
        }

        // If we reach here, the block comment is unterminated
        self.log_error("Unterminated block comment");
        Token::Error("Unterminated block comment".to_string())
    }

    // Helper function to parse comments
    fn parse_comment(&mut self) -> Token {
        if self.current_char() == Some('/') && self.peek_next() == Some('/') {
            self.parse_line_comment()
        } else if self.current_char() == Some('/') && self.peek_next() == Some('*') {
            self.parse_block_comment()
        } else if self.current_char() == Some('/') && self.peek_next() == Some('!') {
            self.parse_doc_comment()
        } else {
            Token::Error("Unexpected comment format".to_string())
        }
    }

    fn parse_identifier_or_keyword(&mut self) -> Token {
        let mut identifier = String::new();

        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else if c == ':' {
                // Check for type annotation syntax (e.g., ": Type")
                self.advance(); // Consume ':'
                self.skip_whitespace();
                let mut type_name = String::new();

                while let Some(c) = self.current_char() {
                    if c.is_alphanumeric() || c == '_' {
                        type_name.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }

                if !type_name.is_empty() {
                    return Token::Type(type_name);
                }
            } else if c == ':' && self.peek_next() == Some(':') {
                // Check for path expression syntax
                self.advance(); // Consume first ':'
                self.advance(); // Consume second ':'
                let mut path_components = vec![identifier];

                while let Some(c) = self.current_char() {
                    if c.is_alphanumeric() || c == '_' {
                        let mut component = String::new();
                        component.push(c);
                        self.advance();

                        // Collect the rest of the component
                        while let Some(next_c) = self.current_char() {
                            if next_c.is_alphanumeric() || next_c == '_' {
                                component.push(next_c);
                                self.advance();
                            } else {
                                break;
                            }
                        }

                        path_components.push(component);
                    } else {
                        break;
                    }
                }

                return Token::Path(path_components);
            } else {
                break;
            }
        }

        // Check if the identifier is a keyword
        if is_keyword(&identifier) {
            Token::Keyword(identifier)
        } else {
            Token::Identifier(identifier)
        }
    }

    // Helper function to parse integers, floats, and hexadecimal numbers
    fn parse_number(&mut self) -> Token {
        let start_pos = self.pos;
        let mut is_float = false;
        let mut is_hex = false;

        // Check for negative sign
        if self.current_char() == Some('-') {
            self.advance();
        }

        // Check for hexadecimal prefix
        if self.current_char() == Some('0')
            && self.peek_next().map_or(false, |c| c == 'x' || c == 'X')
        {
            self.advance(); // Skip '0'
            self.advance(); // Skip 'x'
            is_hex = true;
        }

        // Parse digits
        while let Some(c) = self.current_char() {
            if c.is_digit(10) || (is_hex && c.is_digit(16)) {
                self.advance();
            } else if c == '.' && !is_float {
                self.advance();
                is_float = true;
            } else {
                break;
            }
        }

        let code = &self.input[start_pos..self.pos];

        // Parse and return the appropriate token
        if is_float {
            if let Ok(float_value) = code.parse::<f64>() {
                Token::Float(float_value)
            } else {
                self.log_error("Error parsing float");
                Token::Error(format!("Error parsing float: {}", code))
            }
        } else if is_hex {
            if let Ok(uint_value) = BUint::from_str_radix(code, 16) {
                Token::UInt(uint_value)
            } else {
                self.log_error("Error parsing hexadecimal integer");
                Token::Error(format!("Error parsing hexadecimal integer: {}", code))
            }
        } else {
            if let Ok(int_value) = code.parse::<i64>() {
                Token::Int(int_value)
            } else {
                self.log_error("Error parsing integer");
                Token::Error(format!("Error parsing integer: {}", code))
            }
        }
    }

    fn peek_next(&mut self) -> Option<char> {
        self.peekable_chars.peek().cloned()
    }
}

///////////////////////////////////////////////////////////////////////////

// CHAT-GPT FUNCTIONS

///////////////////////////////////////////////////////////////////////////

fn is_keyword(identifier: &str) -> bool {
    // Implement your keyword checking logic here
    // For example, check if the identifier is "let", "mut", "if", etc.
    // Return true if it's a keyword, false otherwise
    // This is a simplified example; you might need a more comprehensive approach
    [
        "let", "mut", "if", "else", "while", "fn", "struct", "impl", "return",
    ]
    .contains(&identifier)
}

// Helper function to check if a character is a punctuation character
fn is_punctuation(c: char) -> bool {
    ",;:()[]{}+-*/%&|^~<>=".contains(c)
}

///////////////////////////////////////////////////////////////////////////

// FEO IMPLEMENTATION

///////////////////////////////////////////////////////////////////////////


impl Iterator for Lexer<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let src = Arc::into_inner(Arc::clone(&self.input)).unwrap_or("");

        if let Some(c) = src.chars().next() {
            self.pos += 1;
            Some(c)
        } else {
            self.pos = src.len();
            None
        }
    }
}

///////////////////////////////////////////////////////////////////////////

// CHAT-GPT IMPLEMENTATION

///////////////////////////////////////////////////////////////////////////

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        if let Some(c) = self.current_char() {
            Some(match c {
                // ... (unchanged logic for other characters)
                _ => {
                    // Handle delimiter characters
                    if "([{}])".contains(c) {
                        self.tokenize_delimiter()
                    } else {
                        // Continue with the existing tokenization logic
                        // ...

                        // Placeholder return to compile the code
                        Token::Error("Tokenization not implemented yet".to_string())
                    }
                }
            })
        } else {
            None
        }
    }
}
