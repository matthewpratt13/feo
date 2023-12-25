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

    pub fn lex(&mut self) -> Result<TokenStream<Token>, ErrorEmitted> {
        let mut tokens: Vec<Option<Token>> = Vec::new();

        let mut is_negative = false;

        let mut num_open_delimiters: usize = 0;

        while let Some(c) = self.peek_next() {
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

                                while let Some(c) = self.peek_next() {
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
                                while let Some(c) = self.peek_next() {
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

                            while let Some(c) = self.peek_next() {
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

                    while let Some(c) = self.peek_next() {
                        // check for type annotation syntax
                        if c.is_alphanumeric() || c == '_' {
                            buf.push(c);
                            self.advance();
                        } else if c == ':' {
                            self.advance(); // skip ':'
                            self.skip_whitespace();
                            let mut type_name = String::new();

                            let start_pos = self.pos;

                            while let Some(c) = self.peek_next() {
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
                    num_open_delimiters += 1;
                    self.advance(); // skip opening delimiter

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
                }

                ')' | ']' | '}' => {
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

                    self.advance(); // skip delimiter
                    num_open_delimiters -= 1;
                    // panic!();
                }

                '"' => {
                    self.advance(); // skip opening double quote

                    let mut buf = String::new();

                    while let Some(c) = self.peek_next() {
                        match c {
                            '\\' => {
                                self.advance(); // skip first '\'

                                if let Some(esc_c) = self.peek_next() {
                                    self.advance(); // skip second '\'

                                    match esc_c {
                                        'n' => buf.push('\n'),
                                        'r' => buf.push('\r'),
                                        't' => buf.push('\t'),
                                        '\\' => buf.push('\\'),
                                        '0' => buf.push('\0'),
                                        '\'' => buf.push('\''),
                                        '"' => buf.push('"'),
                                        _ => self.log_error(LexErrorKind::InvalidEscapeSequence),
                                    };
                                } else {
                                    // escape sequence is expected, but the input has ended
                                    return Err(
                                        self.emit_error(LexErrorKind::ExpectedEscapeSequence)
                                    );
                                }
                            }

                            '"' => {
                                self.advance(); // skip closing double quote

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

                        //     // TODO: handle unclosed double quote
                    }
                }
                '\'' => {
                    self.advance(); // skip opening single quote

                    if let Some(c) = self.peek_next() {
                        match c {
                            '\\' => {
                                self.advance(); // skip first '\'

                                if let Some(esc_c) = self.peek_next() {
                                    self.advance(); // skip second '\'
                                    self.advance(); // return char to be read

                                    let char_lit =
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
                                            '"' => CharLiteral::tokenize(
                                                &self.input,
                                                "\"",
                                                start_pos,
                                                self.pos,
                                            )?,
                                            '\'' => CharLiteral::tokenize(
                                                &self.input,
                                                "'",
                                                start_pos,
                                                self.pos,
                                            )?,
                                            _ => Err(self
                                                .emit_error(LexErrorKind::InvalidEscapeSequence))?,
                                        };

                                    tokens.push(char_lit);
                                } else {
                                    return Err(
                                        self.emit_error(LexErrorKind::ExpectedEscapeSequence)
                                    );
                                }
                            }
                            '\'' => {
                                self.emit_error(LexErrorKind::EmptyCharLiteral);
                            }
                            _ => {
                                self.advance(); // consume the (regular) char
                                if self.peek_next() == Some('\'') {
                                    self.advance(); // skip closing single quote

                                    let char_lit = CharLiteral::tokenize(
                                        &self.input,
                                        &c.to_string(),
                                        start_pos,
                                        self.pos,
                                    )?;

                                    tokens.push(char_lit);
                                } else {
                                    // TODO: handle invalid char literal
                                    return Err(
                                        self.emit_error(LexErrorKind::ExpectedClosingSingleQuote)
                                    );
                                }
                            }
                        }
                    } else {
                        return Err(self.emit_error(LexErrorKind::ExpectedCharLiteral));
                    }
                }

                // TODO: add support for hexadecimal numbers
                _ if c.is_digit(10) => {
                    let mut is_float = false;

                    let start_pos = if is_negative { self.pos - 1 } else { self.pos };

                    while let Some(c) = self.peek_next() {
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
                    while let Some(c) = self.peek_next() {
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
            // TODO: add pos of missing delim
            return Err(self.emit_error(LexErrorKind::UnclosedDelimiters));
        }

        let stream = TokenStream::new(&self.input, tokens, 0, self.pos);
        Ok(stream)
    }

    fn advance(&mut self) -> Option<char> {
        self.pos += 1;
        self.peekable_chars.next()
    }

    fn peek_next(&mut self) -> Option<char> {
        self.peekable_chars.peek().cloned()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_next() {
            if !c.is_whitespace() {
                break;
            }

            self.advance();
        }
    }

    fn log_error(&mut self, error_kind: LexErrorKind) {
        self.errors.push(LexError {
            error_kind,
            pos: self.pos,
        });
    }

    fn emit_error(&mut self, error_kind: LexErrorKind) -> ErrorEmitted {
        self.log_error(error_kind.clone());

        let err = LexError {
            error_kind,
            pos: self.pos,
        };

        ErrorEmitted::emit_err(CompileError::Lex(err))
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
        {
            a: "a
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
