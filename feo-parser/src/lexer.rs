use std::iter::Peekable;
use std::sync::Arc;

use feo_error::lex_error::{LexError, LexErrorKind};
use feo_types::{
    Delimiter, DocComment, Identifier, Keyword, PuncKind, Punctuation, TypeAnnotation,
};

use crate::{
    literals::{BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral},
    parse::Parse,
};

mod token;
pub(crate) use self::token::Token;
use self::token::TokenStream;

#[allow(dead_code)]
pub(crate) struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    peekable_chars: Peekable<std::str::Chars<'a>>,
    errors: Vec<LexError>,
}

// TODO: sort out error handling – i.e., log error (push to `self.errors`) or return error or panic

#[allow(dead_code)]
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            peekable_chars: input.chars().peekable(),
            errors: Vec::new(),
        }
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

    fn throw_error(&mut self, error_kind: LexErrorKind) {
        self.log_error(error_kind);
        let err = self.errors.clone().pop();
        println!("Error: {:?}", err);
    }

    pub fn tokenize(&mut self) -> Result<TokenStream<Token>, ()> {
        let mut tokens: Vec<Option<Token>> = Vec::new();

        let mut is_negative = false;

        let mut num_open_delimiters: usize = 0;

        while let Some(c) = self.peek_next() {
            let start_pos = self.pos;

            match c {
                _ if c.is_whitespace() => {
                    self.skip_whitespace();
                }

                // TODO: handle unexpected block comment terminator

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

                                let doc_comment_content =
                                    Arc::new(self.input[start_pos..self.pos].to_string());

                                if let Ok(dc) = DocComment::parse(
                                    self.input,
                                    &doc_comment_content,
                                    start_pos,
                                    self.pos,
                                ) {
                                    tokens.push(dc);
                                } else {
                                    self.log_error(LexErrorKind::ParseDocCommentError)
                                }
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
                            // check for `TypeAnnotation` syntax
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
                                    if let Ok(b) = BoolLiteral::parse(
                                        self.input, &type_name, start_pos, self.pos,
                                    ) {
                                        tokens.push(b);
                                        break;
                                    } else {
                                        self.log_error(LexErrorKind::ParseBoolError);
                                    }
                                }

                                if let Ok(t) = TypeAnnotation::parse(
                                    self.input, &type_name, start_pos, self.pos,
                                ) {
                                    tokens.push(t);
                                    break;
                                } else {
                                    self.log_error(LexErrorKind::ParseTypeAnnError)
                                }
                            }
                        } else {
                            break;
                        }
                    }

                    if buf == "true" || buf == "false" {
                        if let Ok(b) =
                            BoolLiteral::parse(self.input, &buf, start_pos, start_pos + buf.len())
                        {
                            tokens.push(b);
                            continue;
                        } else {
                            self.log_error(LexErrorKind::ParseBoolError);
                        }
                    }

                    if let Ok(k) =
                        Keyword::parse(self.input, &buf, start_pos, start_pos + buf.len())
                    {
                        tokens.push(k);
                    } else if let Ok(i) =
                        Identifier::parse(self.input, &buf, start_pos, start_pos + buf.len())
                    {
                        tokens.push(i);
                    } else {
                        self.log_error(LexErrorKind::UnexpectedIdentifier(buf));
                    }
                }

                '(' | '[' | '{' => {
                    num_open_delimiters += 1;
                    self.advance(); // skip delimiter

                    match c {
                        '(' => {
                            if let Ok(d) = Delimiter::parse(self.input, "[", start_pos, self.pos) {
                                tokens.push(d);
                            } else {
                                self.log_error(LexErrorKind::ParseDelimError);
                            }
                        }
                        '[' => {
                            if let Ok(d) = Delimiter::parse(self.input, "[", start_pos, self.pos) {
                                tokens.push(d);
                            } else {
                                self.log_error(LexErrorKind::ParseDelimError)
                            }
                        }
                        '{' => {
                            if let Ok(d) = Delimiter::parse(self.input, "{", start_pos, self.pos) {
                                tokens.push(d);
                            } else {
                                self.log_error(LexErrorKind::ParseDelimError)
                            }
                        }
                        _ => unreachable!(),
                    };
                }

                ')' | ']' | '}' => {
                    self.advance(); // skip delimiter

                    match c {
                        ')' => {
                            if let Ok(d) = Delimiter::parse(self.input, ")", start_pos, self.pos) {
                                tokens.push(d);
                            } else {
                                self.log_error(LexErrorKind::ParseDelimError)
                            }
                        }
                        ']' => {
                            if let Ok(d) = Delimiter::parse(self.input, "]", start_pos, self.pos) {
                                tokens.push(d);
                            } else {
                                self.log_error(LexErrorKind::ParseDelimError)
                            }
                        }
                        '}' => {
                            if let Ok(d) = Delimiter::parse(self.input, "}", start_pos, self.pos) {
                                tokens.push(d);
                            } else {
                                self.log_error(LexErrorKind::ParseDelimError)
                            }
                        }
                        _ => unreachable!(),
                    };

                    self.advance(); // skip delimiter
                    num_open_delimiters -= 1;
                }

                '"' => {
                    self.advance(); // skip opening double quote

                    let mut buf = String::new();

                    while let Some(c) = self.peek_next() {
                        match c {
                            '\\' => {
                                self.advance(); // skip '\'

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
                                    self.log_error(LexErrorKind::ExpectedEscapeSequence);
                                }
                            }

                            '"' => {
                                self.advance(); // skip closing double quote

                                let string_lit =
                                    StringLiteral::parse(self.input, &buf, start_pos, self.pos)
                                        .map_err(|_| self.throw_error(LexErrorKind::ParseError))?;
                                tokens.push(string_lit);
                                break;
                            }

                            _ => {
                                buf.push(c);
                                self.advance();
                            }
                        }

                        // TODO: handle unclosed double quote
                    }
                }
                '\'' => {
                    self.advance(); // skip opening single quote

                    if let Some(c) = self.peek_next() {
                        match c {
                            '\\' => {
                                self.advance(); // skip '\'

                                if let Some(esc_c) = self.peek_next() {
                                    self.advance(); // skip second '\'
                                    self.advance(); // return char to be read

                                    let char_lit = match esc_c {
                                        'n' => CharLiteral::parse(
                                            self.input, "\n", start_pos, self.pos,
                                        ),
                                        'r' => CharLiteral::parse(
                                            self.input, "\r", start_pos, self.pos,
                                        ),
                                        't' => CharLiteral::parse(
                                            self.input, "\t", start_pos, self.pos,
                                        ),
                                        '\\' => CharLiteral::parse(
                                            self.input, "\\", start_pos, self.pos,
                                        ),
                                        '0' => CharLiteral::parse(
                                            self.input, "\0", start_pos, self.pos,
                                        ),
                                        '"' => CharLiteral::parse(
                                            self.input, "\"", start_pos, self.pos,
                                        ),
                                        '\'' => {
                                            CharLiteral::parse(self.input, "'", start_pos, self.pos)
                                        }
                                        _ => {
                                            return Err(self
                                                .throw_error(LexErrorKind::InvalidEscapeSequence))
                                        }
                                    }
                                    .map_err(|_| self.log_error(LexErrorKind::ParseCharError))?;

                                    tokens.push(char_lit);
                                } else {
                                    return Err(
                                        self.throw_error(LexErrorKind::ExpectedEscapeSequence)
                                    );
                                }
                            }
                            '\'' => {
                                self.throw_error(LexErrorKind::EmptyCharLiteral);
                            }
                            _ => {
                                self.advance(); // consume the char
                                                // regular char
                                if self.peek_next() == Some('\'') {
                                    self.advance(); // skip closing single quote

                                    let char_lit = CharLiteral::parse(
                                        self.input,
                                        &c.to_string(),
                                        start_pos,
                                        self.pos,
                                    )
                                    .map_err(|_| self.throw_error(LexErrorKind::ParseCharError))?;
                                    tokens.push(char_lit);
                                } else {
                                    // TODO: handle invalid char literal
                                    return Err(
                                        self.throw_error(LexErrorKind::ExpectedClosingSingleQuote)
                                    );
                                }
                            }
                        }
                    } else {
                        return Err(self.throw_error(LexErrorKind::ExpectedCharLiteral));
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

                    let num_content = Arc::new(self.input[start_pos..self.pos].to_string());

                    if is_float {
                        if let Ok(f) =
                            FloatLiteral::parse(self.input, &num_content, start_pos, self.pos)
                        {
                            tokens.push(f);
                        } else {
                            self.log_error(LexErrorKind::ParseFloatError);
                        }
                    }

                    if is_negative {
                        if let Ok(i) =
                            IntLiteral::parse(self.input, &num_content, start_pos, self.pos)
                        {
                            tokens.push(i);
                        } else {
                            self.log_error(LexErrorKind::ParseIntError);
                        }
                    } else {
                        if let Ok(u) =
                            UIntLiteral::parse(self.input, &num_content, start_pos, self.pos)
                        {
                            tokens.push(u);
                        } else {
                            self.log_error(LexErrorKind::ParseUIntError);
                        }
                    }

                    is_negative = false; // reset `is_negative`
                }

                '!' | '#'..='&' | '*'..='/' | ':'..='@' | '|' | '\0'..='\'' => {
                    while let Some(c) = self.peek_next() {
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
                        let punc_kind = Punctuation::try_from(p.clone().unwrap())?.punc_kind;

                        if punc_kind == PuncKind::Minus
                            && self.peek_next().is_some_and(|c| c.is_digit(10))
                        {
                            is_negative = true;
                            continue;
                        }

                        tokens.push(p);
                    } else {
                        self.log_error(LexErrorKind::UnexpectedChar(c));
                        self.advance();
                    }
                }

                _ => self.log_error(LexErrorKind::InvalidChar(c)),
            }
        }

        if num_open_delimiters > 0 {
            return Err(self.throw_error(LexErrorKind::UnclosedDelimiters));
        }

        let stream = TokenStream::new(self.input, tokens, 0, self.pos);
        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize() {
        let source_code = r#"
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
        "#;

        let mut lexer = Lexer::new(&source_code);

        if let Ok(t) = lexer.tokenize() {
            for token in t.tokens() {
                println!("{:?} \n", token)
            }
        } else {
            println!("Error tokenizing file");
        }
    }
}
