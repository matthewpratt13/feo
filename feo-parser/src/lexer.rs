use std::sync::Arc;

use feo_error::LexErrorKind;

mod token;
pub use self::token::{Token, TokenStream, TokenTree};

pub struct Lexer<'a> {
    src: Arc<&'a str>,
    pos: usize,
}

impl Iterator for Lexer<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let src = Arc::into_inner(Arc::clone(&self.src)).unwrap_or("");

        if let Some(c) = src.chars().next() {
            self.pos += 1;
            Some(c)
        } else {
            self.pos = src.len();
            None
        }
    }
}

impl<'a> Lexer<'a> {
    fn new(data: &'a str) -> Self {
        Self {
            src: Arc::new(data),
            pos: 0,
        }
    }

    fn tokenize(&mut self) -> Result<TokenStream<TokenTree>, LexErrorKind> {
        let src = Arc::into_inner(Arc::clone(&self.src)).ok_or(LexErrorKind::SourceFileEmpty)?;

        let char_reader = &mut Lexer {
            src: Arc::clone(&self.src),
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

        tokens.push(Some(Token::EOF));

        let stream: TokenStream<TokenTree> = TokenStream::build(src, token_trees, 0, self.pos)?;
        Ok(stream)
    }
}
