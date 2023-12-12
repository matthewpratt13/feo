use std::iter::Peekable;
use std::sync::Arc;

use feo_error::LexErrorKind;

mod token;
pub use self::token::{Token, TokenStream, TokenTree};

pub struct Lexer<'a> {
    src: Arc<&'a str>,
    pos: usize,
}

type CharReader<'a> = Peekable<Lexer<'a>>;

impl Iterator for Lexer<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src: Arc::new(src),
            pos: 0,
        }
    }

    fn tokenize(&mut self) -> Result<TokenStream<TokenTree>, LexErrorKind> {
        let src = Arc::into_inner(Arc::clone(&self.src)).ok_or(LexErrorKind::SourceFileEmpty)?;

        let mut tokens: Vec<Option<Token>> = Vec::new();
        let mut token_trees: Vec<Option<TokenTree>> = Vec::new();

        let mut is_negative_number = false;
        let mut in_block_comment = false; // generic inline / multiline comment
        let mut is_hexadecimal_int = false;

        let mut open_delimiters: usize = 0;
        let mut file_start_offset: usize = 0;

        while let Some(c) = self.next() {
            let start_index = self.pos;

            match c {
                _ if c.is_whitespace() => {
                    if start_index - file_start_offset == 0 {
                        file_start_offset += 1;
                    }
                    continue;
                }
                // _ if c == '*' && self.peek() == Some('/') => {
                // if !in_block_comment {
                // throw error
                // } else {
                // self.next(); // skip '*'
                // self.next(); // skip '/'
                // in_block_comment = false;
                // }
                // continue;
                // }
                _ if c == '/' => {
                    match self.next() {
                        Some('/') => {
                            self.next(); // skip second '/'
                                         // if let Some('/') = self.peek() {
                                         // self.next(); // skip third '/'
                                         //              //   parse doc comment
                                         // } else {
                                         // //  parse newline / trailing comment
                                         // continue;
                                         // }
                            continue;
                        }
                        Some('*') => {
                            self.next(); // skip '*'
                            in_block_comment = true;
                            // parse inline / multiline comment
                            self.next(); // skip closing '*'
                            self.next(); // skip closing '/'
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
                    open_delimiters += 1;
                    // parse opening delimiter
                }

                ')' | ']' | '}' => {
                    // parse closing delimiter
                    open_delimiters -= 1;
                }

                '"' => {
                    self.next(); // skip opening double quote
                                 // parse string literal
                    self.next(); // skip closing double quote
                }
                '\'' => {
                    self.next(); // skip opening single quote
                                 // parse char literal
                    self.next(); // skip opening single quote
                }

                // handle negative numbers; do we allow for example "-.3" ?
                // does `is_digit()` include floats?
                // _ if c == '-' && self.peek().is_some_and(|c| c.is_digit(10 | 16)) => {
                // is_negative_number = true;
                // }

                // account for hexadecimal prefix
                // _ if c == '0' && self.peek() == Some('x') => {
                // self.next(); // skip '0'
                // self.next() // skip 'x'
                // is_hexadecimal_digit = true;
                // }
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    // parse digits
                    // continue;
                }

                _ if c.is_ascii_punctuation() => {
                    // parse punctuation
                    // continue;
                }
                _ => return { Err(LexErrorKind::InvalidChar) },
            }
        }

        if open_delimiters != 0 {
            return Err(LexErrorKind::UnclosedDelimiters);
        }

        tokens.push(Some(Token::EOF));

        let stream = TokenStream::new(src, token_trees, 0, self.pos);
        Ok(stream)
    }
}
