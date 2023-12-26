use core::iter::Peekable;
use std::sync::Arc;

use feo_error::error::{CompileError, ErrorEmitted};
use feo_error::lex_error::{LexError, LexErrorKind};

use feo_types::{Delimiter, DocComment, Identifier, Keyword, Punctuation, TypeAnnotation};

use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral,
};

mod token;
pub use self::token::Token;
use self::token::TokenStream;

mod tokenize;
use self::tokenize::Tokenize;

#[allow(dead_code)]
struct Lexer<'a> {
    input: Arc<&'a str>,
    pos: usize,
    peekable_chars: Peekable<std::str::Chars<'a>>,
    errors: Vec<LexError>,
}

// TODO: refine error handling (use a `Handler`?)

#[allow(dead_code)]
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: Arc::new(input),
            pos: 0,
            peekable_chars: input.chars().peekable(),
            errors: Vec::new(),
        }
    }

    // progress through the source code so that the lexer can continue to process chars
    fn advance(&mut self) -> Option<char> {
        // update the lexer's position or other internal state if needed
        self.pos += 1;
        // move to the next char in the iterator
        self.peekable_chars.next()
    }

    // return the current char at the lexer's current position without advancing the pos
    fn current_char(&mut self) -> Option<char> {
        self.peekable_chars.peek().cloned()
    }

    // return the next char in the input stream (i.e., peek ahead by one char)
    fn peek_next(&mut self) -> Option<char> {
        // create a clone of the iterator and advance this cloned iterator
        let mut cloned_iter = self.peekable_chars.clone();
        cloned_iter.next();

        // peek at the next char from the original iterator
        self.peekable_chars.peek().cloned()
    }

    // advance the lexer's pos past any whitespace chars in the input stream
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                // if the current char is whitespace, advance to the next character
                self.advance();
            } else {
                // if the current char is not whitespace, break out of the loop
                break;
            }
        }
    }

    // log and store information about an error encountered during the lexing process
    fn log_error(&mut self, error_kind: LexErrorKind) {
        self.errors.push(LexError {
            error_kind,
            pos: self.pos,
        });
    }

    // log and store error info, and print it through the `ErrorEmitted::emit_err()` method
    // return `ErrorEmitted` to prove than an error was emitted (use in debugging, not production)
    fn emit_error(&mut self, error_kind: LexErrorKind) -> ErrorEmitted {
        self.log_error(error_kind.clone());

        let err = LexError {
            error_kind,
            pos: self.pos,
        };

        ErrorEmitted::emit_err(CompileError::Lex(err))
    }

    // main lexer function
    // return a stream of tokens, parsed and tokenized from an input stream (i.e., source code)
    pub fn lex(&mut self) -> Result<TokenStream<Token>, ErrorEmitted> {
        let mut tokens: Vec<Option<Token>> = Vec::new();

        let mut is_negative = false;

        let mut num_open_delimiters: usize = 0;

        while let Some(c) = self.current_char() {
            let start_pos = self.pos;

            match c {
                _ if c.is_whitespace() => {
                    self.skip_whitespace();
                }

                _ if c == '/' && self.peek_next() == Some('/') || self.peek_next() == Some('*') => {
                    self.advance();
                    match self.peek_next() {
                        Some('/') => {
                            self.advance();

                            if Some('/') == self.peek_next() {
                                self.advance();
                                self.skip_whitespace();

                                let start_pos = self.pos;

                                while let Some(c) = self.current_char() {
                                    if c == '\n' {
                                        break;
                                    } else {
                                        self.advance();
                                    }
                                }

                                let data = self.input[start_pos..self.pos].to_string();

                                let doc_comment_content = Arc::new(&data);

                                let doc_comment = DocComment::tokenize(
                                    &self.input,
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
                            }
                        }
                        Some('*') => {
                            self.advance();

                            while let Some(c) = self.current_char() {
                                if c == '*' {
                                    self.advance();
                                    self.advance();
                                    break;
                                } else {
                                    self.advance();
                                }
                            }
                        }

                        Some(_) | None => (),
                    }
                }

                // identifiers and keywords (cannot start with, but can contain, digits)
                'A'..='Z' | 'a'..='z' | '_' => {
                    let mut buf = String::new();

                    while let Some(c) = self.current_char() {
                        // check for type annotation syntax
                        if c.is_alphanumeric() || c == '_' {
                            buf.push(c);
                            self.advance();
                        } else if c == ':' {
                            self.advance(); // skip ':'
                            self.skip_whitespace();
                            let mut type_name = String::new();

                            let start_pos = self.pos;

                            while let Some(c) = self.current_char() {
                                if c.is_alphanumeric() || c == '_' {
                                    type_name.push(c);
                                    self.advance();
                                } else {
                                    break;
                                }
                            }

                            if !type_name.is_empty() {
                                if type_name == "true" || type_name == "false" {
                                    let bool_lit = BoolLiteral::tokenize(
                                        &self.input,
                                        &type_name,
                                        start_pos,
                                        self.pos,
                                    )?;

                                    tokens.push(bool_lit);
                                    break;
                                }

                                let type_ann = TypeAnnotation::tokenize(
                                    &self.input,
                                    &type_name,
                                    start_pos,
                                    self.pos,
                                )?;

                                tokens.push(type_ann);
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    if buf == "true" || buf == "false" {
                        let bool_lit = BoolLiteral::tokenize(
                            &self.input,
                            &buf,
                            start_pos,
                            start_pos + buf.len(),
                        )?;

                        tokens.push(bool_lit);
                        continue;
                    }

                    if is_keyword(&buf) {
                        let keyword =
                            Keyword::tokenize(&self.input, &buf, start_pos, start_pos + buf.len())?;

                        tokens.push(keyword);
                    } else if is_type_annotation(&buf) {
                        let type_ann = TypeAnnotation::tokenize(
                            &self.input,
                            &buf,
                            start_pos,
                            start_pos + buf.len(),
                        )?;

                        tokens.push(type_ann);
                    } else {
                        let iden = Identifier::tokenize(
                            &self.input,
                            &buf,
                            start_pos,
                            start_pos + buf.len(),
                        )?;
                        tokens.push(iden);
                    }
                }

                '(' | '[' | '{' => {
                    let start_pos = self.pos;

                    match c {
                        '(' => {
                            let delim = Delimiter::tokenize(&self.input, "(", start_pos, self.pos)?;

                            tokens.push(delim);
                        }
                        '[' => {
                            let delim = Delimiter::tokenize(&self.input, "[", start_pos, self.pos)?;

                            tokens.push(delim);
                        }

                        '{' => {
                            let delim = Delimiter::tokenize(&self.input, "{", start_pos, self.pos)?;

                            tokens.push(delim);
                        }
                        _ => unreachable!(),
                    };

                    num_open_delimiters += 1;
                    self.advance(); // skip opening delimiter
                }

                ')' | ']' | '}' => {
                    let start_pos = self.pos;

                    match c {
                        ')' => {
                            let delim = Delimiter::tokenize(&self.input, ")", start_pos, self.pos)?;

                            tokens.push(delim);
                        }
                        ']' => {
                            let delim = Delimiter::tokenize(&self.input, "]", start_pos, self.pos)?;

                            tokens.push(delim);
                        }

                        '}' => {
                            let delim = Delimiter::tokenize(&self.input, "}", start_pos, self.pos)?;

                            tokens.push(delim);
                        }
                        _ => unreachable!(),
                    };

                    num_open_delimiters -= 1;
                    self.advance(); // skip closing delimiter
                }

                '"' => {
                    self.advance(); // skip opening '"'
                    let mut string_literal_open = true;

                    let mut buf = String::new();

                    while let Some(c) = self.current_char() {
                        match c {
                            '\\' => {
                                self.advance(); // skip '\'

                                if let Some(esc_c) = self.current_char() {
                                    self.advance(); // return escaped char

                                    match esc_c {
                                        'n' => buf.push('\n'),
                                        'r' => buf.push('\r'),
                                        't' => buf.push('\t'),
                                        '\\' => buf.push('\\'),
                                        '0' => buf.push('\0'),
                                        '\'' => buf.push('\''),
                                        '"' => buf.push('"'),
                                        _ => {
                                            return Err(self
                                                .emit_error(LexErrorKind::InvalidEscapeSequence))
                                        }
                                    };
                                } else {
                                    // escape sequence is expected, but the input has ended
                                    return Err(
                                        self.emit_error(LexErrorKind::ExpectedEscapeSequence)
                                    );
                                }
                            }

                            '"' => {
                                self.advance(); // skip closing '"'
                                string_literal_open = false;

                                let string_lit = StringLiteral::tokenize(
                                    &self.input,
                                    &buf,
                                    start_pos,
                                    self.pos,
                                )?;

                                tokens.push(string_lit);
                                break;
                            }

                            _ => {
                                buf.push(c);
                                self.advance();
                            }
                        }
                    }

                    if string_literal_open {
                        return Err(self.emit_error(LexErrorKind::UnclosedStringLiteral));
                    }
                }
                '\'' => {
                    self.advance(); // skip opening '\'' (single quote)

                    if let Some(c) = self.current_char() {
                        match c {
                            '\\' => {
                                self.advance(); // skip '\'

                                if let Some(esc_c) = self.current_char() {
                                    self.advance(); // return escaped char

                                    let esc_char_lit =
                                        match esc_c {
                                            'n' => CharLiteral::tokenize(
                                                &self.input,
                                                "\n",
                                                start_pos,
                                                self.pos,
                                            )?,
                                            'r' => CharLiteral::tokenize(
                                                &self.input,
                                                "\r",
                                                start_pos,
                                                self.pos,
                                            )?,
                                            't' => CharLiteral::tokenize(
                                                &self.input,
                                                "\t",
                                                start_pos,
                                                self.pos,
                                            )?,
                                            '\\' => CharLiteral::tokenize(
                                                &self.input,
                                                "\\",
                                                start_pos,
                                                self.pos,
                                            )?,
                                            '0' => CharLiteral::tokenize(
                                                &self.input,
                                                "\0",
                                                start_pos,
                                                self.pos,
                                            )?,
                                            '\'' => CharLiteral::tokenize(
                                                &self.input,
                                                "'",
                                                start_pos,
                                                self.pos,
                                            )?,
                                            '"' => CharLiteral::tokenize(
                                                &self.input,
                                                "\"",
                                                start_pos,
                                                self.pos,
                                            )?,
                                            _ => Err(self
                                                .emit_error(LexErrorKind::InvalidEscapeSequence))?,
                                        };

                                    tokens.push(esc_char_lit);
                                } else {
                                    // escape sequence is expected, but the input has ended
                                    return Err(
                                        self.emit_error(LexErrorKind::ExpectedEscapeSequence)
                                    );
                                }
                            }
                            '\'' => {
                                return Err(self.emit_error(LexErrorKind::EmptyCharLiteral));
                            }
                            _ => {
                                self.advance(); // return the regular char
                                if self.current_char() == Some('\'') {
                                    self.advance(); // skip closing '\'' (single quote)

                                    let char_lit = CharLiteral::tokenize(
                                        &self.input,
                                        &c.to_string(),
                                        start_pos,
                                        self.pos,
                                    )?;

                                    tokens.push(char_lit);
                                } else {
                                    return Err(self.emit_error(LexErrorKind::InvalidPunctuation));
                                }
                            }
                        }
                    } else {
                        return Err(self.emit_error(LexErrorKind::ExpectedCharLiteral));
                    }
                }

                // hexadecimal uints
                _ if c == '0' && self.advance().map_or(false, |c| c == 'x' || c == 'X') => {
                    self.advance(); // skip '0'
                    self.advance(); // skip 'x'

                    println!("current char: {:?}", self.current_char());

                    let start_pos = self.pos;

                    while let Some(c) = self.current_char() {
                        if c.is_digit(16) {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let data = self.input[start_pos..self.pos].to_string();

                    let num_content = Arc::new(&data);

                    println!("data: {}", data);

                    let hex_uint_lit =
                        UIntLiteral::tokenize(&self.input, &num_content, start_pos, self.pos)?;

                    tokens.push(hex_uint_lit);
                }

                _ if c.is_digit(10) => {
                    self.advance();

                    // let mut is_hex = false;

                    // if c == '0' && self.peek_next().map_or(false, |c| c == 'x' || c == 'X') {
                    //     self.advance();
                    //     self.advance();
                    //     is_hex = true;
                    // }

                    // println!("current_char: {:?}", self.current_char());

                    let start_pos = if is_negative { self.pos - 1 } else { self.pos };

                    let mut is_float = false;

                    while let Some(c) = self.current_char() {
                        if c.is_digit(10) {
                            self.advance();
                        } else if c == '.' && !is_float {
                            self.advance();
                            is_float = true;
                        } else {
                            break;
                        }
                    }

                    let data = self.input[start_pos..self.pos].to_string();

                    let num_content = Arc::new(&data);

                    if is_float {
                        let float_lit =
                            FloatLiteral::tokenize(&self.input, &num_content, start_pos, self.pos)?;

                        tokens.push(float_lit);
                        continue;
                    }

                    if is_negative {
                        let int_lit =
                            IntLiteral::tokenize(&self.input, &num_content, start_pos, self.pos)?;
                        tokens.push(int_lit);
                    } else {
                        let uint_lit =
                            UIntLiteral::tokenize(&self.input, &num_content, start_pos, self.pos)?;
                        tokens.push(uint_lit);
                    }

                    is_negative = false; // reset `is_negative`
                }

                '!' | '#'..='&' | '*'..='/' | ':'..='@' | '|' => {
                    while let Some(c) = self.current_char() {
                        if c.is_ascii_punctuation() {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let data = self.input[start_pos..self.pos].to_string();

                    let punc_content = Arc::new(&data);

                    let punc =
                        Punctuation::tokenize(&self.input, &punc_content, start_pos, self.pos)?;

                    if punc_content.as_str() == "-"
                        && self.peek_next().is_some_and(|c| c.is_digit(10))
                    {
                        is_negative = true;
                        continue;
                    }

                    tokens.push(punc);
                }

                _ => self.log_error(LexErrorKind::InvalidChar(c)),
            }
        }

        if num_open_delimiters > 0 {
            return Err(self.emit_error(LexErrorKind::UnclosedDelimiters));
        }

        let stream = TokenStream::new(&self.input, tokens, 0, self.pos);
        Ok(stream)
    }
}

fn is_keyword(iden: &str) -> bool {
    [
        "break", "const", "continue", "deref", "else", "enum", "for", "func", "if", "impl",
        "import", "in", "let", "loop", "match", "mod", "mut", "pub", "ref", "return", "self",
        "static", "struct", "super", "trait", "type", "while",
    ]
    .contains(&iden)
}

fn is_type_annotation(iden: &str) -> bool {
    [
        "bool", "char", "f32", "f64", "i32", "i64", "String", "u8", "u16", "u32", "u64", "Vec",
    ]
    .contains(&iden)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex() {
        let source_code = r#"
        / line comment
        /*
        block comment
        */
        /// doc comment

        struct Foo {
            a: String,
            b: i32,
            c: char,
            d: bool
        }

        impl Foo {
            pub func new() -> Foo {
                let num: u32 = 0x1234;
                let vec = [1, 2, 3, 4];
                let mut new_vec: Vec<f64> = [];

                if foo < 0 {
                    print("{}", foo);
                } else {
                    print("{}", foo);
                }

                for i in vec {
                    new_vec.push(i + 1.0);
                }

                return Foo {
                    a: "foo",
                    b: -123,
                    c: 'a',
                    d: false
                };
            }
        }
        "#;

        let mut lexer = Lexer::new(&source_code);

        if let Ok(t) = lexer.lex() {
            for token in t.tokens() {
                println!("{:?} \n", token)
            }
        } else {
            println!("Error tokenizing file");
        }
    }
}

/*
// line comment
        /*
        block comment
        */
        /// doc comment

        struct Foo {
            a: String,
            b: i32,
            c: char,
            d: bool
        }

        impl Foo {
            pub func new() -> Foo {
                let num: u32 = 0x1234;
                let vec = [1, 2, 3, 4];
                let mut new_vec: Vec<f64> = [];

                if foo < 0 {
                    print("{}", foo);
                } else {
                    print("{}", foo);
                }

                for i in vec {
                    new_vec.push(i + 1.0);
                }

                return Foo {
                    a: "foo",
                    b: -123,
                    c: 'a',
                    d: false
                };
            }
        }
    */
