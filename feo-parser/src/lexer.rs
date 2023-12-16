use std::iter::Peekable;
use std::sync::Arc;

use feo_types::{
    Comment, DelimKind, Delimiter, DocComment, Identifier, Keyword, PathExpression, Punctuation,
    TypeAnnotation,
};

use crate::{
    error::ParserError,
    literals::{BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral},
    parse::Parse,
};

mod token;
pub(crate) use self::token::{Token, TokenStream, TokenTree};

pub(crate) struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    peekable_chars: Peekable<std::str::Chars<'a>>,
    errors: Vec<String>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            peekable_chars: input.chars().peekable(),
            errors: Vec::new(),
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.peekable_chars.next();
    }

    fn current_char(&self) -> Option<char> {
        self.peekable_chars.peek().cloned()
    }

    // *mutable
    fn peek_next(&mut self) -> Option<char> {
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

    fn log_error(&mut self, message: &str) {
        let error_message = format!("Error at position {}: {}", self.pos, message);
        self.errors.push(error_message);
    }

    fn tokenize(&mut self) -> Result<TokenStream<TokenTree>, ParserError> {
        let mut tokens: Vec<Option<Token>> = Vec::new();
        let mut token_trees: Vec<Option<TokenTree>> = Vec::new();

        let mut in_block_comment = false; // generic inline / multiline comment
        let mut is_negative_number = false;
        let mut is_hexadecimal_int = false;

        let mut num_open_delimiters: usize = 0;
        // let mut file_start_offset: usize = 0;

        while let Some(c) = self.current_char() {
            let start_pos = self.pos;

            match c {
                _ if c.is_whitespace() => {
                    // move the reader to the first char if there is whitespace at the start
                    // if start_pos - file_start_offset == 0 {
                    // file_start_offset += 1;
                    // }

                    self.skip_whitespace();
                }
                _ if c == '*' && self.peek_next() == Some('/') => {
                    if !in_block_comment {
                        self.log_error("Unexpected comment terminator without opener");
                    } else {
                        self.advance();
                        self.advance();
                        in_block_comment = false;
                    }
                }
                _ if c == '/' => match self.peek_next() {
                    Some('/') => {
                        self.advance();
                        self.advance();

                        let start_pos = self.pos;

                        if let Some('/') = self.peek_next() {
                            self.advance();

                            let start_pos = self.pos;

                            while let Some(c) = self.current_char() {
                                if c == '\n' {
                                    break;
                                } else {
                                    self.advance();
                                }
                            }

                            let doc_comment_content =
                                Arc::new(self.input[start_pos..self.pos].to_string());

                            let doc_comment = DocComment::parse(
                                self.input,
                                doc_comment_content,
                                start_pos,
                                self.pos,
                            )?;
                            tokens.push(doc_comment);
                        } else {
                            while let Some(c) = self.current_char() {
                                if c == '\n' {
                                    break;
                                } else {
                                    self.advance();
                                }
                            }

                            let comment =
                                Comment::parse(self.input, String::from(""), start_pos, self.pos)?;

                            tokens.push(comment);
                        }
                    }

                    Some('!') => {
                        self.advance();
                        self.advance();
                        in_block_comment = true;

                        let start_pos = self.pos;

                        while let Some(c) = self.current_char() {
                            if c == '\n' {
                                continue;
                            }

                            if c == '*' && self.peek_next() == Some('/') {
                                self.advance();
                                self.advance();
                                in_block_comment = false;

                                let doc_comment_content =
                                    Arc::new(self.input[start_pos..self.pos].trim().to_string());

                                let doc_comment = DocComment::parse(
                                    self.input,
                                    doc_comment_content,
                                    start_pos,
                                    self.pos,
                                )?;
                                tokens.push(doc_comment);
                                break;
                            } else {
                                self.advance();
                            }
                        }
                    }

                    Some('*') => {
                        self.advance();
                        self.advance();
                        in_block_comment = true;

                        let start_pos = self.pos;

                        while let Some(c) = self.current_char() {
                            if c == '\n' {
                                continue;
                            }

                            if c == '*' && self.peek_next() == Some('/') {
                                self.advance();
                                self.advance();
                                in_block_comment = false;

                                let comment = Comment::parse(
                                    self.input,
                                    String::from(""),
                                    start_pos,
                                    self.pos,
                                )?;

                                tokens.push(comment);
                                break;
                            } else {
                                self.advance();
                            }
                        }
                    }

                    Some(_) | None => {
                        self.log_error("Unexpected comment");
                    }
                },

                _ if c.is_alphabetic() || c == '_' => {
                    let mut buf = String::new();

                    while let Some(c) = self.current_char() {
                        if c.is_alphanumeric() || c == '_' {
                            buf.push(c);
                            self.advance()
                        } else if c == ':' {
                            self.advance(); // skip ':' for `TypeAnnotation`;
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
                                let type_ann =
                                    TypeAnnotation::parse(self.input, buf, start_pos, self.pos)?;
                                tokens.push(type_ann);
                            }
                        } else if c == ':' && self.peek_next() == Some(':') {
                            // check for `PathExpression` syntax
                            self.advance(); // skip first ':'
                            self.advance(); // skip second ':'

                            let mut path_components = vec![buf];

                            while let Some(c) = self.current_char() {
                                if c.is_alphabetic() || c == '_' {
                                    let mut component = String::new();
                                    component.push(c);
                                    self.advance();

                                    while let Some(next_c) = self.current_char() {
                                        if next_c.is_alphanumeric() || next_c == '_' {
                                            component.push(c);
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
                            let path = PathExpression::parse(
                                self.input,
                                path_components,
                                start_pos,
                                self.pos,
                            )?;
                            tokens.push(path);
                        } else {
                            break;
                        }
                    }

                    if let Ok(k) = Keyword::parse(self.input, buf, start_pos, self.pos) {
                        tokens.push(k);
                    } else if let Ok(b) = BoolLiteral::parse(self.input, buf, start_pos, self.pos) {
                        tokens.push(b);
                    } else {
                        let iden = Identifier::parse(self.input, buf, start_pos, self.pos)?;
                        tokens.push(iden);
                    }
                }

                '(' | '[' | '{' => {
                    num_open_delimiters += 1;
                    match c {
                        '(' => tokens.push(Delimiter::parse(self.input, '(', start_pos, self.pos)?),
                        '[' => tokens.push(Delimiter::parse(self.input, '[', start_pos, self.pos)?),
                        '{' => tokens.push(Delimiter::parse(self.input, '{', start_pos, self.pos)?),
                    };
                    let tree = TokenTree::build(
                        self.input,
                        std::mem::take(&mut tokens),
                        self.pos - tokens.len(),
                        self.pos,
                    )?;
                    token_trees.push(tree);
                }

                ')' | ']' | '}' => {
                    match c {
                        ')' => tokens.push(Delimiter::parse(self.input, ')', start_pos, self.pos)?),
                        ']' => tokens.push(Delimiter::parse(self.input, ']', start_pos, self.pos)?),
                        '}' => tokens.push(Delimiter::parse(self.input, '}', start_pos, self.pos)?),
                    };
                    // TODO: check that this closing delimiter matches the opening one
                    let tree = TokenTree::build(
                        self.input,
                        std::mem::take(&mut tokens),
                        self.pos - tokens.len(),
                        self.pos,
                    )?;
                    token_trees.push(tree);

                    num_open_delimiters -= 1;
                }

                '"' => {
                    self.advance(); // skip opening double quote
                    let start_pos = self.pos;

                    let mut buf = String::new();

                    while let Some(c) = self.current_char() {
                        match c {
                            '\\' => {
                                self.advance(); // skip '\'

                                if let Some(esc_c) = self.current_char() {
                                    self.advance(); // skip second '\'

                                    match esc_c {
                                        'n' => buf.push('\n'),
                                        'r' => buf.push('\r'),
                                        't' => buf.push('\t'),
                                        '"' => buf.push('"'),
                                        '\'' => buf.push('\''),
                                        _ => self
                                            .log_error("Invalid escape sequence in string literal"),
                                    };
                                }
                            }

                            '"' => {
                                self.advance(); // skip closing double quote
                                let string_lit =
                                    StringLiteral::parse(self.input, buf, start_pos, self.pos)?;
                                tokens.push(string_lit);
                                break;
                            }

                            _ => {
                                buf.push(c);
                                self.advance();
                            }
                        }
                    }
                }
                '\'' => {
                    self.advance(); // skip opening single quote
                    let start_pos = self.pos;

                    if let Some(c) = self.current_char() {
                        match c {
                            '\\' => {
                                self.advance(); // skip '\'

                                if let Some(esc_c) = self.current_char() {
                                    self.advance(); // skip second '\'

                                    let char_lit = match esc_c {
                                        'n' => CharLiteral::parse(
                                            self.input, '\n', start_pos, self.pos,
                                        ),
                                        'r' => CharLiteral::parse(
                                            self.input, '\r', start_pos, self.pos,
                                        ),
                                        't' => CharLiteral::parse(
                                            self.input, '\t', start_pos, self.pos,
                                        ),

                                        '"' => {
                                            CharLiteral::parse(self.input, '"', start_pos, self.pos)
                                        }

                                        '\'' => CharLiteral::parse(
                                            self.input, '\'', start_pos, self.pos,
                                        ),
                                        _ => return Err(ParserError::InvalidEscapeCode),
                                    }?;

                                    tokens.push(char_lit);
                                }
                            }
                            '\'' => self.log_error("Empty character literal"),
                            _ => {
                                self.advance(); // consume the char
                                if self.current_char() == Some('\'') {
                                    self.advance(); // skip closing single quote
                                    let char_lit =
                                        CharLiteral::parse(self.input, c, start_pos, self.pos)?;
                                    tokens.push(char_lit);
                                } else {
                                    self.log_error("Invalid character literal");
                                }
                            }
                        }
                    } else {
                        self.log_error("Unexpected end of input in character literal");
                    }
                }

                _ if c == '-' && self.peek_next().is_some_and(|c| c.is_digit(10)) => {
                    let mut is_float = false;
                    let mut is_hex = false;
                    let mut is_negative = false;

                    if self.current_char() == Some('-') {
                        is_negative = true;
                        self.advance(); // skip '-'
                    }

                    if self.current_char() == Some('0')
                        && self.peek_next().map_or(false, |c| c == 'x' || c == 'X')
                    {
                        self.advance(); // skip '0'
                        self.advance(); // skip 'x'
                        is_hex = true;
                    }

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

                    let num_content = Arc::new(self.input[start_pos..self.pos].to_string());

                    if is_float {
                        if let Ok(f) =
                            FloatLiteral::parse(self.input, num_content, start_pos, self.pos)
                        {
                            tokens.push(f);
                        } else {
                            self.log_error("Error parsing float");
                        }
                    } else if is_negative {
                        if let Ok(i) =
                            IntLiteral::parse(self.input, num_content, start_pos, self.pos)
                        {
                            tokens.push(i);
                        } else {
                            self.log_error("Error parsing integer");
                        }
                    } else if is_hex {
                        if let Ok(u) =
                            UIntLiteral::parse(self.input, num_content, start_pos, self.pos)
                        {
                            tokens.push(u);
                        } else {
                            self.log_error("Error parsing hexadecimal uint");
                        }
                    } else {
                        if let Ok(u) =
                            UIntLiteral::parse(self.input, num_content, start_pos, self.pos)
                        {
                            tokens.push(u);
                        } else {
                            self.log_error("Error parsing uint");
                        }
                    }
                }

                _ if c.is_ascii_punctuation() => {
                    while let Some(c) = self.current_char() {
                        if c.is_ascii_punctuation() {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let punc_content = Arc::new(self.input[start_pos..self.pos].to_string());

                    if let Ok(p) = Punctuation::parse(self.input, punc_content, start_pos, self.pos)
                    {
                        tokens.push(p)
                    } else {
                        self.log_error("Error punctuation uint");
                    }
                }
            }
        }

        if num_open_delimiters > 0 {
            self.log_error("Unexpected end of input within delimiter");
        }

        let stream = TokenStream::build(self.input, token_trees, 0, self.pos);
        stream
    }

    ///////////////////////////////////////////////////////////////////////////

    // CHAT-GPT FUNCTIONS

    ///////////////////////////////////////////////////////////////////////////

    fn gpt_new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            peekable_chars: input.chars().peekable(),
            errors: Vec::new(),
        }
    }

    fn gpt_advance(&mut self) {
        self.pos += 1;
        self.peekable_chars.next();
    }

    fn gpt_current_char(&self) -> Option<char> {
        self.peekable_chars.peek().cloned()
    }

    fn gpt_skip_whitespace(&mut self) {
        while let Some(c) = self.gpt_current_char() {
            if !c.is_whitespace() {
                break;
            }
            self.gpt_advance();
        }
    }

    fn gpt_log_error(&mut self, message: &str) {
        let error_message = format!("Error at position {}: {}", self.pos, message);
        // self.errors.push(error_message);
    }

    fn gpt_tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.gpt_current_char() {
            if c.is_whitespace() {
                self.gpt_skip_whitespace();
                tokens.push(Token::Whitespace);
            } else if c.is_digit(10) || (c == '-' && self.gpt_peek_next().is_digit(10)) {
                tokens.push(self.gpt_parse_number());
            } else if c.is_alphabetic() || c == '_' {
                tokens.push(self.gpt_parse_identifier_or_keyword());
            } else if c == '"' {
                tokens.push(self.gpt_parse_string());
            } else if c == '\'' {
                tokens.push(self.gpt_parse_char());
            } else if c == '/' && self.gpt_peek_next() == Some('/') {
                tokens.push(self.gpt_parse_comment());
            } else if c == '/' && self.gpt_peek_next() == Some('*') {
                tokens.push(self.gpt_parse_block_comment());
            } else {
                if gpt_is_punctuation(c) {
                    self.gpt_advance();
                    tokens.push(Token::Punc(c));
                } else {
                    match c {
                        '(' | ')' | '[' | ']' | '{' | '}' => {
                            tokens.push(self.gpt_tokenize_delimiter());
                        }
                        _ => {
                            // self.gpt_log_error(&format!("Unexpected character: {}", c));
                            self.gpt_advance();
                        }
                    }
                }
            }
        }

        tokens
    }

    fn gpt_tokenize_delimiter(&mut self) -> Token {
        let delimiter_type = match self.gpt_current_char() {
            Some('(') | Some(')') => DelimKind::Paren,
            Some('[') | Some(']') => DelimKind::Bracket,
            Some('{') | Some('}') => DelimKind::Brace,
            _ => unreachable!(), // Should not be called for non-delimiter characters
        };

        self.gpt_advance(); // Consume the delimiter character
        Token::Delim(delimiter_type) // MP: conflicting type
    }

    fn gpt_tokenize_token_tree(&mut self, delimiter_type: DelimKind) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut nesting_level = 1;

        while let Some(c) = self.gpt_current_char() {
            match c {
                '(' | '[' | '{' => {
                    nesting_level += 1;
                    self.gpt_advance();
                }
                ')' | ']' | '}' => {
                    nesting_level -= 1;
                    if nesting_level == 0 {
                        self.gpt_advance();
                        break;
                    } else {
                        self.gpt_advance();
                    }
                }
                _ => {
                    tokens.push(self.gpt_tokenize_token());
                }
            }
        }

        if nesting_level > 0 {
            self.gpt_log_error("Unexpected end of input within delimiter");
        }

        tokens
    }

    fn gpt_tokenize_token(&mut self) -> Token {
        match self.gpt_current_char() {
            Some('(') | Some('[') | Some('{') => {
                let delimiter_type = match self.gpt_current_char() {
                    Some('(') => DelimKind::Paren,
                    Some(')') => DelimKind::Paren,
                    Some('[') => DelimKind::Bracket,
                    Some(']') => DelimKind::Bracket,
                    Some('{') => DelimKind::Brace,
                    Some('}') => DelimKind::Brace,
                    _ => unreachable!(), // Should not be called for non-delimiter characters
                };

                self.gpt_advance();
                Token::Delim(delimiter_type) // MP: conflicting type
            }
            _ => {
                // Continue with existing tokenization logic
                // ...

                // Placeholder return to compile the code
                Token::Error("Tokenization not implemented yet".to_string())
            }
        }
    }

    fn gpt_parse_string(&mut self) -> Token {
        self.gpt_advance(); // Consume the opening double quote
        let mut string_content = String::new();

        while let Some(c) = self.gpt_current_char() {
            match c {
                '\\' => {
                    // Handle escape sequences
                    if let Some(escaped_char) = self.gpt_parse_escape_sequence() {
                        string_content.push(escaped_char);
                    } else {
                        // Invalid escape sequence
                        self.gpt_log_error("Invalid escape sequence in string literal");
                    }
                }
                '"' => {
                    // Consume the closing double quote
                    self.gpt_advance();
                    return Token::StringLit(string_content); // MP: conflicting types
                }
                _ => {
                    string_content.push(c);
                    self.gpt_advance();
                }
            }
        }

        // If we reach here, there's an unterminated string literal
        self.gpt_log_error("Unterminated string literal");
        Token::Error("Unterminated string literal".to_string())
    }

    fn gpt_parse_escape_sequence(&mut self) -> Option<char> {
        self.gpt_advance(); // Consume the backslash

        if let Some(escaped_char) = self.gpt_current_char() {
            self.gpt_advance(); // Consume the escaped character
            match escaped_char {
                'n' => Some('\n'),
                'r' => Some('\r'),
                't' => Some('\t'),
                '\\' => Some('\\'),
                '"' => Some('"'),
                '\'' => Some('\''),
                _ => {
                    // Invalid escape sequence
                    self.gpt_log_error("Invalid escape sequence in string literal");
                    None
                }
            }
        } else {
            // Escape sequence is expected, but the input has ended
            self.gpt_log_error("Unexpected end of input in escape sequence");
            None
        }
    }

    fn gpt_parse_char(&mut self) -> Token {
        self.gpt_advance(); // Consume the opening single quote

        if let Some(c) = self.gpt_current_char() {
            match c {
                '\\' => {
                    // Handle escape sequences
                    if let Some(escaped_char) = self.gpt_parse_escape_sequence() {
                        // Check for the closing single quote
                        if self.gpt_current_char() == Some('\'') {
                            self.gpt_advance(); // Consume the closing single quote
                            return Token::CharLit(escaped_char); // MP: conflicting type
                        } else {
                            // Invalid character literal
                            self.gpt_log_error("Invalid character literal");
                            return Token::Error("Invalid character literal".to_string());
                        }
                    } else {
                        // Invalid escape sequence
                        self.gpt_log_error("Invalid escape sequence in character literal");
                        return Token::Error(
                            "Invalid escape sequence in character literal".to_string(),
                        );
                    }
                }
                '\'' => {
                    // Empty character literal is invalid
                    self.gpt_log_error("Empty character literal");
                    Token::Error("Empty character literal".to_string())
                }
                _ => {
                    // Regular character
                    self.gpt_advance(); // Consume the character
                    if self.gpt_current_char() == Some('\'') {
                        self.gpt_advance(); // Consume the closing single quote
                        Token::CharLit(c) // MP: conflicting type
                    } else {
                        // Invalid character literal
                        self.gpt_log_error("Invalid character literal");
                        Token::Error("Invalid character literal".to_string())
                    }
                }
            }
        } else {
            // Unexpected end of input
            self.gpt_log_error("Unexpected end of input in character literal");
            Token::Error("Unexpected end of input in character literal".to_string())
        }
    }

    fn gpt_parse_line_comment(&mut self) -> Token {
        self.gpt_advance(); // Consume the first '/'
        self.gpt_advance(); // Consume the second '/'

        let mut comment_content = String::new();

        while let Some(c) = self.gpt_current_char() {
            if c == '\n' {
                // End of line, finish the comment
                break;
            } else {
                comment_content.push(c);
                self.gpt_advance();
            }
        }

        Token::LineComment(comment_content)
    }

    fn gpt_parse_doc_comment(&mut self) -> Token {
        self.gpt_advance(); // Skip '/'
        self.gpt_advance(); // Skip '!'
        let start_pos = self.pos;

        while let Some(c) = self.gpt_current_char() {
            if c == '*' && self.gpt_peek_next() == Some('/') {
                self.gpt_advance();
                self.gpt_advance();
                let end_pos = self.pos;
                let code = Arc::new(self.input[start_pos..end_pos].trim().to_string());
                return Token::DocComment(code); // MP: conflicting type
            } else {
                self.gpt_advance();
            }
        }

        self.gpt_log_error("Unterminated doc comment");
        Token::Error("Unterminated doc comment".to_string())
    }

    fn gpt_parse_block_comment(&mut self) -> Token {
        self.gpt_advance(); // Consume the first '/'
        self.gpt_advance(); // Consume the '!' character

        let mut comment_content = String::new();

        while let Some(c) = self.gpt_current_char() {
            if c == '*' && self.gpt_peek_next() == Some('/') {
                // Consume the '*' and '/'
                self.gpt_advance();
                self.gpt_advance();
                return Token::BlockComment(comment_content); // MP: conflicting type
            } else {
                comment_content.push(c);
                self.gpt_advance();
            }
        }

        // If we reach here, the block comment is unterminated
        self.gpt_log_error("Unterminated block comment");
        Token::Error("Unterminated block comment".to_string())
    }

    // Helper function to parse comments
    fn gpt_parse_comment(&mut self) -> Token {
        if self.gpt_current_char() == Some('/') && self.gpt_peek_next() == Some('/') {
            self.gpt_parse_line_comment()
        } else if self.current_char() == Some('/') && self.gpt_peek_next() == Some('*') {
            self.gpt_parse_block_comment()
        } else if self.current_char() == Some('/') && self.gpt_peek_next() == Some('!') {
            self.gpt_parse_doc_comment()
        } else {
            Token::Error("Unexpected comment format".to_string())
        }
    }

    fn gpt_parse_identifier_or_keyword(&mut self) -> Token {
        let mut identifier = String::new();

        while let Some(c) = self.gpt_current_char() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.gpt_advance();
            } else if c == ':' {
                // Check for type annotation syntax (e.g., ": Type")
                self.gpt_advance(); // Consume ':'
                self.gpt_skip_whitespace();
                let mut type_name = String::new();

                while let Some(c) = self.gpt_current_char() {
                    if c.is_alphanumeric() || c == '_' {
                        type_name.push(c);
                        self.gpt_advance();
                    } else {
                        break;
                    }
                }

                if !type_name.is_empty() {
                    return Token::Type(type_name);
                }
            } else if c == ':' && self.gpt_peek_next() == Some(':') {
                // Check for path expression syntax
                self.gpt_advance(); // Consume first ':'
                self.gpt_advance(); // Consume second ':'
                let mut path_components = vec![identifier];

                while let Some(c) = self.gpt_current_char() {
                    if c.is_alphanumeric() || c == '_' {
                        let mut component = String::new();
                        component.push(c);
                        self.gpt_advance();

                        // Collect the rest of the component
                        while let Some(next_c) = self.gpt_current_char() {
                            if next_c.is_alphanumeric() || next_c == '_' {
                                component.push(next_c);
                                self.gpt_advance();
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
        if gpt_is_keyword(&identifier) {
            Token::Keyword(identifier) // MP: conflicting type
        } else {
            Token::Iden(identifier) // MP: conflicting type
        }
    }

    // Helper function to parse integers, floats, and hexadecimal numbers
    fn gpt_parse_number(&mut self) -> Token {
        let start_pos = self.pos;
        let mut is_float = false;
        let mut is_hex = false;

        // Check for negative sign
        if self.gpt_current_char() == Some('-') {
            self.gpt_advance();
        }

        // Check for hexadecimal prefix
        if self.gpt_current_char() == Some('0')
            && self.gpt_peek_next().map_or(false, |c| c == 'x' || c == 'X')
        {
            self.gpt_advance(); // Skip '0'
            self.gpt_advance(); // Skip 'x'
            is_hex = true;
        }

        // Parse digits
        while let Some(c) = self.gpt_current_char() {
            if c.is_digit(10) || (is_hex && c.is_digit(16)) {
                self.gpt_advance();
            } else if c == '.' && !is_float {
                self.gpt_advance();
                is_float = true;
            } else {
                break;
            }
        }

        let code = &self.input[start_pos..self.pos];

        // Parse and return the appropriate token
        if is_float {
            if let Ok(float_value) = code.parse::<f64>() {
                Token::FloatLit(float_value) // MP: conflicting type
            } else {
                self.gpt_log_error("Error parsing float");
                Token::Error(format!("Error parsing float: {}", code))
            }
        } else if is_hex {
            if let Ok(uint_value) = u64::from_str_radix(code, 16) {
                Token::UIntLit(uint_value) // MP: conflicting type
            } else {
                self.gpt_log_error("Error parsing hexadecimal integer");
                Token::Error(format!("Error parsing hexadecimal integer: {}", code))
            }
        } else {
            if let Ok(int_value) = code.parse::<i64>() {
                Token::IntLit(int_value) // MP: conflicting type
            } else {
                self.gpt_log_error("Error parsing integer");
                Token::Error(format!("Error parsing integer: {}", code))
            }
        }
    }

    fn gpt_peek_next(&mut self) -> Option<char> {
        self.peekable_chars.peek().cloned()
    }
}

///////////////////////////////////////////////////////////////////////////

// CHAT-GPT FUNCTIONS

///////////////////////////////////////////////////////////////////////////

fn gpt_is_keyword(identifier: &str) -> bool {
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
fn gpt_is_punctuation(c: char) -> bool {
    ",;:()[]{}+-*/%&|^~<>=".contains(c)
}

///////////////////////////////////////////////////////////////////////////

// FEO IMPLEMENTATION

///////////////////////////////////////////////////////////////////////////

impl Iterator for Leer<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.input.chars().next() {
            self.pos += 1;
            Some(c)
        } else {
            self.pos = self.input.len();
            None
        }
    }
}

///////////////////////////////////////////////////////////////////////////

// CHAT-GPT IMPLEMENTATION

///////////////////////////////////////////////////////////////////////////

// impl<'a> Iterator for Lexer<'a> {
//     type Item = Token;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.gpt_skip_whitespace();

//         if let Some(c) = self.gpt_current_char() {
//             Some(match c {
//                 // ... (unchanged logic for other characters)
//                 _ => {
//                     // Handle delimiter characters
//                     if "([{}])".contains(c) {
//                         self.gpt_tokenize_delimiter()
//                     } else {
//                         // Continue with the existing tokenization logic
//                         // ...

//                         // Placeholder return to compile the code
//                         Token::Error("Tokenization not implemented yet".to_string())
//                     }
//                 }
//             })
//         } else {
//             None
//         }
//     }
// }
