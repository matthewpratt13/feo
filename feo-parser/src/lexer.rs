use std::iter::Peekable;
use std::sync::Arc;

use feo_types::{
    Comment, Delimiter, DocComment, Identifier, Keyword, PathExpression, Punctuation,
    TypeAnnotation,
};

use crate::{
    error::ParserError,
    literals::{BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral},
    parse::{Parse, ParseVec},
};

mod token;
pub(crate) use self::token::Token;
use self::token::{TokenStream, TokenTree};

pub(crate) struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    peekable_chars: Peekable<std::str::Chars<'a>>,
    errors: Vec<String>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            peekable_chars: input.chars().peekable(),
            errors: Vec::new(),
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.peekable_chars.next();
    }

    fn current_char(&mut self) -> Option<char> {
        self.peekable_chars.next()
    }

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

    // TODO: return `LexError`
    pub fn tokenize(&mut self) -> Result<TokenStream<TokenTree>, ParserError> {
        let mut tokens: Vec<Option<Token>> = Vec::new();
        let mut token_trees: Vec<Option<TokenTree>> = Vec::new();

        let mut num_open_block_comments: usize = 0;
        let mut num_open_doc_comments: usize = 0;
        let mut num_open_delimiters: usize = 0;
        let mut num_open_single_quotes: usize = 0;
        let mut num_open_double_quotes: usize = 0;
        // let mut file_start_offset: usize = 0;

        let mut is_float = false;
        let mut is_negative = false;
        let mut is_hexadecimal = false;

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
                    if num_open_block_comments == 0 {
                        // TODO: return `Err`
                        self.log_error("Unexpected comment terminator without opener");
                    } else {
                        self.advance();
                        self.advance();
                        num_open_block_comments -= 1;
                    }
                }
                _ if c == '/' => match self.peek_next() {
                    Some('/') => {
                        self.advance(); // skip first '/'
                        self.advance(); // skip second '/'

                        let start_pos = self.pos;

                        if let Some('/') = self.peek_next() {
                            self.advance(); // skip third '/'

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
                                &doc_comment_content,
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

                            // no need to store ordinary comment content
                            let comment =
                                Comment::parse(self.input, &String::from(""), start_pos, self.pos)?;

                            tokens.push(comment);
                        }
                    }

                    Some('!') => {
                        self.advance(); // skip '/'
                        self.advance(); // skip '!'
                        num_open_doc_comments += 1;

                        let start_pos = self.pos;

                        while let Some(c) = self.current_char() {
                            if c == '\n' {
                                continue;
                            }

                            // close doc comment
                            if c == '*' && self.peek_next() == Some('/') {
                                self.advance(); // skip '*'
                                self.advance(); // skip '/'
                                num_open_doc_comments -= 1;

                                let doc_comment_content =
                                    Arc::new(self.input[start_pos..self.pos].trim().to_string());

                                let doc_comment = DocComment::parse(
                                    self.input,
                                    &doc_comment_content,
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
                        self.advance(); // skip '/'
                        self.advance(); // skip '*'
                        num_open_block_comments += 1;

                        let start_pos = self.pos;

                        while let Some(c) = self.current_char() {
                            if c == '*' && self.peek_next() == Some('/') {
                                self.advance(); // skip '*'
                                self.advance(); // skip '/'
                                num_open_block_comments -= 1;

                                // no need to store ordinary comment content
                                let comment = Comment::parse(
                                    self.input,
                                    &String::from(""),
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

                    // TODO: check comment opener/closer against '/' alone (i.e., division operator)
                    // TODO: before returning `()`
                    Some(_) | None => (),
                },

                // `Identifier` + `Keyword` (cannot start with digit, but can contain)
                'A'..='Z' | 'a'..='z' | '_' => {
                    let mut buf = String::new();

                    while let Some(c) = self.current_char() {
                        if c.is_alphanumeric() || c == '_' {
                            buf.push(c);
                            self.advance();
                        } else if c == ':' {
                            // check for `TypeAnnotation` syntax
                            self.advance(); // skip ':'
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
                                    TypeAnnotation::parse(self.input, &buf, start_pos, self.pos)?;
                                tokens.push(type_ann);
                                break;
                            }
                        } else if c == ':' && self.peek_next() == Some(':') {
                            // check for `PathExpression` syntax
                            self.advance(); // skip first ':'
                            self.advance(); // skip second ':'

                            let mut path_components: Vec<String> = Vec::new();

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
                                &path_components,
                                start_pos,
                                self.pos,
                            )?;
                            tokens.push(path);
                            break;
                        } else {
                            break;
                        }
                    }

                    if let Ok(k) = Keyword::parse(self.input, &buf, start_pos, self.pos) {
                        tokens.push(k);
                    } else if let Ok(b) = BoolLiteral::parse(self.input, &buf, start_pos, self.pos)
                    {
                        tokens.push(b);
                    } else {
                        let iden = Identifier::parse(self.input, &buf, start_pos, self.pos)?;
                        tokens.push(iden);
                    }
                }

                '(' | '[' | '{' => {
                    num_open_delimiters += 1;
                    match c {
                        '(' => {
                            let delim = Delimiter::parse(self.input, &'(', start_pos, self.pos)?;
                            tokens.push(delim);
                        }
                        '[' => {
                            let delim = Delimiter::parse(self.input, &'[', start_pos, self.pos)?;
                            tokens.push(delim);
                        }
                        '{' => {
                            let delim = Delimiter::parse(self.input, &'{', start_pos, self.pos)?;
                            tokens.push(delim);
                        }
                        _ => unreachable!(),
                    };
                    let tree = TokenTree::build(
                        self.input,
                        std::mem::take(&mut tokens),
                        self.pos - tokens.len(),
                        self.pos,
                    )?;
                    token_trees.push(tree);
                    self.advance(); // skip delimiter
                }

                ')' | ']' | '}' => {
                    match c {
                        ')' => {
                            let delim = Delimiter::parse(self.input, &')', start_pos, self.pos)?;
                            tokens.push(delim);
                        }
                        ']' => {
                            let delim = Delimiter::parse(self.input, &']', start_pos, self.pos)?;
                            tokens.push(delim)
                        }
                        '}' => {
                            let delim = Delimiter::parse(self.input, &'}', start_pos, self.pos)?;
                            tokens.push(delim);
                        }
                        _ => unreachable!(),
                    };
                    // TODO: check that this closing delimiter matches the opening one
                    let tree = TokenTree::build(
                        self.input,
                        std::mem::take(&mut tokens),
                        self.pos - tokens.len(),
                        self.pos,
                    )?;

                    token_trees.push(tree);

                    self.advance(); // skip delimiter
                    num_open_delimiters -= 1;
                }

                '"' => {
                    self.advance(); // skip opening double quote
                    num_open_double_quotes += 1;

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
                                        '\\' => buf.push('\\'),
                                        '0' => buf.push('\0'),
                                        '\'' => buf.push('\''),
                                        '"' => buf.push('"'),
                                        _ => self
                                            .log_error("Invalid escape sequence in char literal"),
                                    };
                                } else {
                                    // TODO: return `Err`
                                    // Escape sequence is expected, but the input has ended
                                    self.log_error("Unexpected end of input in escape sequence");
                                }
                            }

                            '"' => {
                                self.advance(); // skip closing double quote
                                num_open_double_quotes -= 1;

                                let string_lit =
                                    StringLiteral::parse(self.input, &buf, start_pos, self.pos)?;
                                tokens.push(string_lit);
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
                    num_open_single_quotes += 1;

                    if let Some(c) = self.current_char() {
                        match c {
                            '\\' => {
                                self.advance(); // skip '\'

                                if let Some(esc_c) = self.current_char() {
                                    self.advance(); // skip second '\'

                                    let char_lit = match esc_c {
                                        'n' => CharLiteral::parse(
                                            self.input, &'\n', start_pos, self.pos,
                                        ),
                                        'r' => CharLiteral::parse(
                                            self.input, &'\r', start_pos, self.pos,
                                        ),
                                        't' => CharLiteral::parse(
                                            self.input, &'\t', start_pos, self.pos,
                                        ),
                                        '\\' => CharLiteral::parse(
                                            self.input, &'\\', start_pos, self.pos,
                                        ),
                                        '0' => CharLiteral::parse(
                                            self.input, &'\0', start_pos, self.pos,
                                        ),
                                        '"' => CharLiteral::parse(
                                            self.input, &'"', start_pos, self.pos,
                                        ),
                                        '\'' => CharLiteral::parse(
                                            self.input, &'\'', start_pos, self.pos,
                                        ),
                                        _ => return Err(ParserError::InvalidEscapeSequence),
                                    }?;

                                    tokens.push(char_lit);
                                }
                            }
                            '\'' => {
                                // TODO: return `Err`
                                num_open_single_quotes -= 1;
                                self.log_error("Empty character literal");
                            }
                            _ => {
                                // regular char
                                self.advance(); // consume the char
                                if self.current_char() == Some('\'') {
                                    self.advance(); // skip closing single quote
                                    num_open_single_quotes -= 1;

                                    let char_lit =
                                        CharLiteral::parse(self.input, &c, start_pos, self.pos)?;
                                    tokens.push(char_lit);
                                } else {
                                    // TODO: return `Err`
                                    self.log_error("Invalid character literal");
                                }
                            }
                        }
                    } else {
                        // TODO: return `Err`
                        self.log_error("Unexpected end of input in character literal");
                    }
                }

                _ if c == '-' && self.peek_next().is_some_and(|c| c.is_digit(10)) => {
                    is_negative = true;
                    self.advance(); // skip '-'
                }

                _ if c == '0' && self.peek_next().map_or(false, |c| c == 'x' || c == 'X') => {
                    is_hexadecimal = true;
                    self.advance(); // skip '0'
                    self.advance(); // skip 'x'
                }

                _ if c.is_digit(10) || (is_hexadecimal && c.is_digit(16)) => {
                    let start_pos = if is_negative {
                        if is_hexadecimal {
                            self.pos - 3
                        } else {
                            self.pos - 1
                        }
                    } else {
                        self.pos
                    };

                    while let Some(c) = self.current_char() {
                        if c.is_digit(10 | 16) {
                            self.advance();
                        } else if c == '.' && !is_float {
                            self.advance();
                            is_float = true;
                        } else {
                            break;
                        }
                    }

                    let num_content = Arc::new(self.input[start_pos..self.pos].to_string());

                    // parse and push the appropriate tokens to `tokens`
                    if is_float {
                        if let Ok(f) =
                            FloatLiteral::parse(self.input, &num_content, start_pos, self.pos)
                        {
                            tokens.push(f);
                        } else {
                            self.log_error("Error parsing float");
                        }
                    } else if is_negative {
                        if let Ok(i) =
                            IntLiteral::parse(self.input, &num_content, start_pos, self.pos)
                        {
                            tokens.push(i);
                        } else {
                            self.log_error("Error parsing integer");
                        }
                    } else {
                        if let Ok(u) =
                            UIntLiteral::parse(self.input, &num_content, start_pos, self.pos)
                        {
                            tokens.push(u);
                        } else {
                            self.log_error("Error parsing uint");
                        }
                    }
                }

                // punctuation / escape codes
                '!' | '#'..='&' | '*'..='/' | ':'..='@' | '|' | '\0'..='\'' => {
                    while let Some(c) = self.current_char() {
                        if c.is_ascii_punctuation() {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let punc_content = Arc::new(self.input[start_pos..self.pos].to_string());

                    if let Ok(p) =
                        Punctuation::parse(self.input, &punc_content, start_pos, self.pos)
                    {
                        tokens.push(p)
                    } else {
                        // TODO: return `Err`
                        self.log_error(&format!("Unexpected character: {}", c));
                        self.advance();
                    }
                }

                _ => return Err(ParserError::UnexpectedChar),
            }
        }

        if num_open_doc_comments > 0 {
            // TODO: return `Err`
            self.log_error("Unexpected doc comment terminator without opener");
        }

        if num_open_block_comments > 0 {
            // TODO: return `Err`
            self.log_error("Unexpected block comment terminator without opener");
        }

        if num_open_delimiters > 0 {
            // TODO: return `Err`
            self.log_error("Unexpected end of input within delimiter");
        }

        if num_open_double_quotes > 0 {
            // TODO: return `Err`
            self.log_error("Unexpected end of input in string literal");
        }

        if num_open_single_quotes > 0 {
            // TODO: return `Err`
            self.log_error("Unexpected end of input in character literal");
        }

        let stream = TokenStream::build(self.input, token_trees, 0, self.pos);
        stream
    }
}
